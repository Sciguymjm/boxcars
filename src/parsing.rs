//! # Parsing
//!
//! A Rocket League game replay is a little endian binary encoded file with an emphasis. The number
//! 100 would be represented as the four byte sequence:
//!
//! ```plain
//! 0x64 0x00 0x00 0x00
//! ```
//!
//! This in contrast to big-endian, which would represent the number as:
//!
//! ```plain
//! 0x00 0x00 0x00 0x64
//! ```
//!
//! A replay is split into three major sections, a header, body, and footer.
//!
//! ## Header
//!
//! The first four bytes of a replay is the number of bytes that comprises the header. A length
//! prefixed integer is very common throughout a replay. This prefix may either be in reference to
//! the number of bytes an elements takes up, as just seen, or the number of elements in a list.
//!
//! The next four bytes make up the [cyclic redundancy check
//! (CRC)](https://en.wikipedia.org/wiki/Cyclic_redundancy_check) for the header. The check ensures
//! that the data has not be tampered with or, more likely, corrupted.
//!
//! The game's major and minor version follow, each 32bit integers.
//!
//! Subsequently, the game type is encoded as a string. Strings in Rocket League Replay files are
//! length prefixed and null terminated.
//!
//! The properties is where all the good nuggets of info reside. Visualize the properties as a map
//! of strings to various types (number, string, array) that continues until a "None" key is found.
//!
//! ## Body
//!
//! Out of the body we get:
//!
//! - Levels (what level did the match take place)
//! - `KeyFrames`
//! - The body's crc. This check is actually for the rest of the content (including the footer).
//!
//! Since everything is length prefixed, we're able to skip the network stream data. This would be
//! 90% of the file.  Most of the interesting bits like player stats and goals are contained in the
//! header, so it's not a tremendous loss if we can't parse the network data.
//!
//! ## Footer
//!
//! After the network stream there we see:
//!
//! - Debug info
//! - Tickmarks
//! - Packages
//! - Etc

use encoding_rs::{UTF_16LE, WINDOWS_1252};
use models::*;
use crc::calc_crc;
use errors::{ParseError, AttributeError};
use std::borrow::Cow;
use failure::{Error, ResultExt};
use byteorder::{ByteOrder, LittleEndian};
use bitter::BitGet;
use {SPAWN_STATS, ATTRIBUTES, OBJECT_CLASSES, PARENT_CLASSES};
use network::{SpawnTrajectory, NewActor, Trajectory, UpdatedAttribute, Frame, normalize_object};
use attributes::{AttributeDecoder, Attribute};
use std::collections::HashMap;
use fnv::FnvHashMap;
use std::ops::Deref;

/// Determines under what circumstances the parser should perform the crc check for replay
/// corruption. Since the crc check is the most time consuming check for parsing (causing
/// microseconds to turn into milliseconds), clients should choose under what circumstances a crc
/// check is performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrcCheck {
    /// Always perform the crc check. Useful when the replay has had its contents modified. This
    /// will catch a user that increased the number of goals they scored (easy) but only if they
    /// didn't update the crc as well (not as easy).
    Always,

    /// Never perform the crc check. Useful only when it doesn't matter to know if a replay is
    /// corrupt or not, you either want the data or the parsing error.
    Never,

    /// Only perform the crc check when parsing a section fails. This option gets the best of both
    /// worlds. If parsing fails, the crc check will determine if it is a programming error or the
    /// replay is corrupt. If parsing succeeds it won't precious time performing the check. This
    /// option is the default for parsing.
    OnError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkParse {
    Always,
    Never,
    IgnoreOnError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorAction {
    Created,
    Updated,
    Destroyed,
}

/// Intermediate parsing structure for the header
#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    pub major_version: i32,
    pub minor_version: i32,
    pub net_version: Option<i32>,
    pub game_type: Cow<'a, str>,
    pub properties: Vec<(&'a str, HeaderProp<'a>)>,
}

impl<'a> Header<'a> {
    fn num_frames(&self) -> Option<i32> {
        self.properties.iter()
            .find(|&&(key, _)| key == "NumFrames")
            .and_then(|&(_, ref prop)| if let HeaderProp::Int(v) = *prop {
                Some(v)
            } else {
                None
            })
    }

    fn max_channels(&self) -> Option<i32> {
        self.properties.iter()
            .find(|&&(key, _)| key == "MaxChannels")
            .and_then(|&(_, ref prop)| if let HeaderProp::Int(v) = *prop {
                Some(v)
            } else {
                None
            })
    }
}

/// Intermediate parsing structure for the body / footer
#[derive(Debug, PartialEq)]
struct ReplayBody<'a> {
    levels: Vec<Cow<'a, str>>,
    keyframes: Vec<KeyFrame>,
    debug_info: Vec<DebugInfo<'a>>,
    tick_marks: Vec<TickMark<'a>>,
    packages: Vec<Cow<'a, str>>,
    objects: Vec<Cow<'a, str>>,
    names: Vec<Cow<'a, str>>,
    class_indices: Vec<ClassIndex<'a>>,
    net_cache: Vec<ClassNetCache>,
    network_data: &'a [u8],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserBuilder<'a> {
    data: &'a [u8],
    crc_check: Option<CrcCheck>,
    network_parse: Option<NetworkParse>,
}

impl<'a> ParserBuilder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ParserBuilder {
            data: data,
            crc_check: None,
            network_parse: None,
        }
    }

    pub fn always_check_crc(mut self) -> ParserBuilder<'a> {
        self.crc_check = Some(CrcCheck::Always);
        self
    }

    pub fn never_check_crc(mut self) -> ParserBuilder<'a> {
        self.crc_check = Some(CrcCheck::Never);
        self
    }

    pub fn on_error_check_crc(mut self) -> ParserBuilder<'a> {
        self.crc_check = Some(CrcCheck::OnError);
        self
    }

    pub fn with_crc_check(mut self, check: CrcCheck) -> ParserBuilder<'a> {
        self.crc_check = Some(check);
        self
    }

    pub fn must_parse_network_data(mut self) -> ParserBuilder<'a> {
        self.network_parse = Some(NetworkParse::Always);
        self
    }

    pub fn never_parse_network_data(mut self) -> ParserBuilder<'a> {
        self.network_parse = Some(NetworkParse::Never);
        self
    }

    pub fn ignore_network_data_on_error(mut self) -> ParserBuilder<'a> {
        self.network_parse = Some(NetworkParse::IgnoreOnError);
        self
    }

    pub fn with_network_parse(mut self, parse: NetworkParse) -> ParserBuilder<'a> {
        self.network_parse = Some(parse);
        self
    }

    pub fn parse(self) -> Result<Replay<'a>, Error> {
        let mut parser = Parser::new(
            self.data,
            self.crc_check.unwrap_or(CrcCheck::OnError),
            self.network_parse.unwrap_or(NetworkParse::IgnoreOnError),
        );
        parser.parse()
    }
}

struct CacheInfo<'a> {
    max_prop_id: i32,
    prop_id_bits: i32,
    attributes: &'a HashMap<i32, fn(&AttributeDecoder, &mut BitGet) -> Result<Attribute, AttributeError>>,
}


/// Holds the current state of parsing a replay
#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'a> {
    /// A slice (not the whole) view of the replay. Bytes are popped off as data is read.
    data: &'a [u8],

    /// Current offset in regards to the whole view of the replay
    col: i32,
    crc_check: CrcCheck,
    network_parse: NetworkParse,
}

impl<'a> Parser<'a> {
    fn new(data: &'a [u8], crc_check: CrcCheck, network_parse: NetworkParse) -> Self {
        Parser {
            data: data,
            col: 0,
            crc_check: crc_check,
            network_parse: network_parse,
        }
    }

    fn err_str(&self, desc: &'static str, e: &ParseError) -> String {
        format!(
            "Could not decode replay {} at offset ({}): {}",
            desc,
            self.col,
            e
        )
    }

    fn parse(&mut self) -> Result<Replay<'a>, Error> {
        let header_size = self.take(4, le_i32)
            .with_context(|e| self.err_str("header size", e))?;

        let header_crc = self.take(4, le_i32)
            .with_context(|e| self.err_str("header crc", e))?;

        let header_data = self.view_data(header_size as usize)
            .with_context(|e| self.err_str("header data", e))?;

        let header =
            self.crc_section(header_data, header_crc as u32, "header", Self::parse_header)?;

        let content_size = self.take(4, le_i32)
            .with_context(|e| self.err_str("content size", e))?;

        let content_crc = self.take(4, le_i32)
            .with_context(|e| self.err_str("content crc", e))?;

        let content_data = self.view_data(content_size as usize)
            .with_context(|e| self.err_str("content data", e))?;

        let body = self.crc_section(content_data, content_crc as u32, "body", Self::parse_body)?;

        let mut network: Option<NetworkFrames> = None;
        match self.network_parse {
            NetworkParse::Always => {
                network = Some(self.parse_network(&header, &body)?);
            }
            NetworkParse::IgnoreOnError => {
                if let Ok(v) = self.parse_network(&header, &body) {
                    network = Some(v);
                }
            }
            NetworkParse::Never => network = None,
        }

        Ok(Replay {
            header_size: header_size,
            header_crc: header_crc,
            major_version: header.major_version,
            minor_version: header.minor_version,
            net_version: header.net_version,
            game_type: header.game_type,
            properties: header.properties,
            content_size: content_size,
            content_crc: content_crc,
            network_frames: network,
            levels: body.levels,
            keyframes: body.keyframes,
            debug_info: body.debug_info,
            tick_marks: body.tick_marks,
            packages: body.packages,
            objects: body.objects,
            names: body.names,
            class_indices: body.class_indices,
            net_cache: body.net_cache,
        })
    }

    fn parse_network(&mut self, header: &Header, body: &ReplayBody) -> Result<NetworkFrames, Error> {
        // Create a parallel vector where each object has it's name normalized
        let normalized_objects: Vec<&str> = body.objects.iter().map(|x|
            normalize_object(x.deref())
        ).collect();

		let spawns: Vec<SpawnTrajectory> = 
            body.objects.iter()
                .map(|x|
                    SPAWN_STATS.get(x.deref()).cloned().unwrap_or(SpawnTrajectory::None)
                )
                .collect();

        let attrs: Vec<_> =
            normalized_objects.iter()
                .map(|x|
                    ATTRIBUTES.get(x.deref()).cloned().unwrap_or(AttributeDecoder::decode_not_implemented)
                )
                .collect();

        // Create a map of an object's normalized name to a list of indices in the object
        // vector that have that same normalized name
        let mut normalized_name_obj_ind: HashMap<&str, Vec<usize>> = HashMap::new();
        for (i, x) in normalized_objects.iter().enumerate() {
            normalized_name_obj_ind.entry(x).or_insert_with(Vec::new).push(i);
        }

        let name_obj_ind: HashMap<&str, usize> =
            body.objects.iter().enumerate().map(|(ind, name)|
                (name.deref(), ind)
            ).collect();

        let mut object_ind_attrs: HashMap<i32, HashMap<_, _>> = HashMap::new();

        for cache in &body.net_cache {
            let mut all_props: HashMap<i32, _> = cache.properties.iter()
                .map(|x| (x.stream_id, attrs[x.object_ind as usize]))
                .collect();

            // cache ids can occur multiple times
            // If the class we are looking at appears in `PARENT_CLASSES` that has first priority
            let object_name: &str = &*body.objects[cache.object_ind as usize];
            if let Some(parent_name) = PARENT_CLASSES.get(object_name) {
                if let Some(parent_ind) = name_obj_ind.get(parent_name) {
                    if let Some(parent_attrs) = object_ind_attrs.get(&(*parent_ind as i32)) {
                        all_props.extend(parent_attrs.iter());
                    }
                }
            }

            object_ind_attrs.insert(cache.object_ind, all_props);
        }

        for (obj, parent) in OBJECT_CLASSES.entries(){
            // It's ok if an object class doesn't appear in our replay. For instance, basketball
            // objects don't appear in a soccer replay.
            if let Some(indices) = normalized_name_obj_ind.get(obj) {
                let parent_ind = name_obj_ind.get(parent).ok_or_else(||
                    format_err!("Replay contained object index: {} but not the parent class: {}",
                        obj, parent))?; 

                for i in indices {
                    let parent_attrs: HashMap<_, _> = object_ind_attrs.get(&(*parent_ind as i32)).unwrap().clone();
                    object_ind_attrs.insert((*i as i32), parent_attrs);
                }
            }
        }

        let object_ind_attributes: FnvHashMap<i32, CacheInfo> =
            object_ind_attrs.iter().map(|(obj_ind, attrs)| {
                let key = *obj_ind;
                let max = *attrs.keys().max().unwrap_or(&2) + 1;
                (key, CacheInfo { 
                    max_prop_id: max,
                    prop_id_bits: log2((max as u32).checked_next_power_of_two().unwrap()) as i32,
                    attributes: attrs,
                })
            }).collect();

        let color_ind = *name_obj_ind.get("TAGame.ProductAttribute_UserColor_TA").unwrap_or(&0) as u32;
        let painted_ind = *name_obj_ind.get("TAGame.ProductAttribute_Painted_TA").unwrap_or(&0)  as u32;

        // 1023 stolen from rattletrap
        let channels = header.max_channels().unwrap_or(1023);
        let channel_bits = log2((channels as u32).checked_next_power_of_two().unwrap()) as i32;
        let num_frames = header.num_frames();

        if let Some(frame_len) = num_frames {
            let attr_decoder = AttributeDecoder::new(header, color_ind, painted_ind);
            let mut frames: Vec<Frame> = Vec::with_capacity(frame_len as usize);
            let mut actors = FnvHashMap::default();
            let mut bits = BitGet::new(body.network_data);

            loop {
                if bits.is_empty() {
                    break;
                }

                let time = bits.read_f32_unchecked();
                if time < 0.0 || (time > 0.0 && time < 1e-10) {
                    bail!("Time too small");
                }

                let delta = bits.read_f32_unchecked();
                if delta < 0.0 || (delta > 0.0 && delta < 1e-10) {
                    bail!("Delta too small");
                }

                if time == 0.0 && delta == 0.0 {
                    break;
                }

                let mut new_actors = Vec::new();
                let mut updated_actors = Vec::new();
                let mut deleted_actors = Vec::new();

                while bits.read_bit_unchecked() {
                    let actor_id = bits.read_i32_bits_unchecked(channel_bits);
                    
                    // alive
                    if bits.read_bit_unchecked() {
                        // new
                        if bits.read_bit_unchecked() {
                            let name_id = 
                                if header.major_version > 868 || (header.major_version == 868 && header.minor_version >= 14) {
                                    Some(bits.read_u32_unchecked() as i32)
                                } else {
                                    None
                                };

                            let _ = bits.read_bit_unchecked();
                            let type_id = bits.read_u32_unchecked() as i32;
                            
                            actors.insert(actor_id, type_id);

                            // fix index
                            let traj = Trajectory::from_spawn_unchecked(&mut bits, spawns[type_id as usize]);
                            new_actors.push(NewActor {
                                actor_id: actor_id,
                                object_ind: type_id,
                                initial_trajectory: traj
                            });

                        } else {
                            let type_id = actors.get(&actor_id).ok_or_else(|| format_err!("Actor id: {} was not found", actor_id))?;
                            let cache_info = object_ind_attributes.get(type_id)
                                .ok_or_else(|| format_err!("Actor id: {} of object index: {} ({}) but not attributes found",
                                    actor_id,
                                    type_id,
                                    normalized_objects.get(*type_id as usize).unwrap_or(&"Out of bounds")))?;

                            while bits.read_bit_unchecked() {
                                let prop_id = bits.read_bits_max_unchecked(cache_info.prop_id_bits, cache_info.max_prop_id) as i32;
                                assert!(prop_id < cache_info.max_prop_id);

                                let attr = cache_info.attributes.get(&prop_id)
                                    .ok_or_else(|| format_err!("Actor id: {} of object index: {} ({}) but attribute cache id: {} not found in {:?}",
                                        actor_id,
                                        type_id,
                                        normalized_objects.get(*type_id as usize).unwrap_or(&"Out of bounds"),
                                        prop_id,
                                        cache_info.attributes.keys()))?;

                                let attribute = attr(&attr_decoder, &mut bits).unwrap(); 
                                updated_actors.push(UpdatedAttribute {
                                    actor_id: actor_id,
                                    attribute_id: prop_id,
                                    attribute: attribute,
                                });
                            }
                        }
                    } else {
                        deleted_actors.push(actor_id);
                        actors.remove(&actor_id);
                    }
                }
                
                frames.push(Frame {
                    time: time,
                    delta: delta,
                    new_actors: new_actors,
                    deleted_actors: deleted_actors,
                    updated_actors: updated_actors,
                });
            }

            Ok(NetworkFrames { frames: frames })
        } else {
            Ok(NetworkFrames { frames: Vec::new() })
        }
    }

    fn parse_header(&mut self) -> Result<Header<'a>, Error> {
        let major_version = self.take(4, le_i32)
            .with_context(|e| self.err_str("major version", e))?;

        let minor_version = self.take(4, le_i32)
            .with_context(|e| self.err_str("minor version", e))?;

        let net_version = if major_version > 865 && minor_version > 17 {
            Some(self.take(4, le_i32)
                .with_context(|e| self.err_str("net version", e))?)
        } else {
            None
        };

        let game_type = self.parse_text()
            .with_context(|e| self.err_str("game type", e))?;

        let properties = self.parse_rdict()
            .with_context(|e| self.err_str("header properties", e))?;

        Ok(Header {
            major_version: major_version,
            minor_version: minor_version,
            net_version: net_version,
            game_type: game_type,
            properties: properties,
        })
    }

    /// Parses a section and performs a crc check as configured
    fn crc_section<T, F>(
        &mut self,
        data: &[u8],
        crc: u32,
        section: &str,
        mut f: F,
    ) -> Result<T, Error>
    where
        F: FnMut(&mut Self) -> Result<T, Error>,
    {
        match (self.crc_check, f(self)) {
            (CrcCheck::Always, res) => {
                let actual = calc_crc(data);
                if actual != crc as u32 {
                    Err(Error::from(ParseError::CrcMismatch(crc, actual)))
                } else {
                    res
                }
            }
            (CrcCheck::OnError, Err(e)) => {
                let actual = calc_crc(data);
                if actual != crc as u32 {
                    Err(e.context(format!(
                        "Failed to parse {} and crc check failed. Replay is corrupt",
                        section
                    )).into())
                } else {
                    Err(e)
                }
            }
            (CrcCheck::OnError, Ok(s)) => Ok(s),
            (CrcCheck::Never, res) => res,
        }
    }

    fn parse_body(&mut self) -> Result<ReplayBody<'a>, Error> {
        let levels = self.text_list()
            .with_context(|e| self.err_str("levels", e))?;

        let keyframes = self.parse_keyframe()
            .with_context(|e| self.err_str("keyframes", e))?;

        let network_size = self.take(4, le_i32)
            .with_context(|e| self.err_str("network size", e))?;

        let network_data = self.take(network_size as usize, |d| d)
            .with_context(|e| self.err_str("network data", e))?;

        let debug_infos = self.parse_debuginfo()
            .with_context(|e| self.err_str("debug info", e))?;

        let tickmarks = self.parse_tickmarks()
            .with_context(|e| self.err_str("tickmarks", e))?;

        let packages = self.text_list()
            .with_context(|e| self.err_str("packages", e))?;
        let objects = self.text_list()
            .with_context(|e| self.err_str("objects", e))?;
        let names = self.text_list().with_context(|e| self.err_str("names", e))?;

        let class_index = self.parse_classindex()
            .with_context(|e| self.err_str("class index", e))?;

        let net_cache = self.parse_classcache()
            .with_context(|e| self.err_str("net cache", e))?;

        Ok(ReplayBody {
            levels: levels,
            keyframes: keyframes,
            debug_info: debug_infos,
            tick_marks: tickmarks,
            packages: packages,
            objects: objects,
            names: names,
            class_indices: class_index,
            net_cache: net_cache,
            network_data: network_data,
        })
    }

    /// Used for skipping some amount of data
    fn advance(&mut self, ind: usize) {
        self.col += ind as i32;
        self.data = &self.data[ind..];
    }

    /// Returns a slice of the replay after ensuring there is enough space for the requested slice
    fn view_data(&self, size: usize) -> Result<&'a [u8], ParseError> {
        if size > self.data.len() {
            Err(ParseError::InsufficientData(
                size as i32,
                self.data.len() as i32,
            ))
        } else {
            Ok(&self.data[..size])
        }
    }

    /// Take the next `size` of bytes and interpret them in an infallible fashion
    #[inline]
    fn take<F, T>(&mut self, size: usize, mut f: F) -> Result<T, ParseError>
    where
        F: FnMut(&'a [u8]) -> T,
    {
        let res = f(self.view_data(size)?);
        self.advance(size);
        Ok(res)
    }

    /// Take the next `size` of bytes and interpret them, but this interpretation can fail
    fn take_res<F, T>(&mut self, size: usize, mut f: F) -> Result<T, ParseError>
    where
        F: FnMut(&'a [u8]) -> Result<T, ParseError>,
    {
        let res = f(self.view_data(size)?)?;
        self.advance(size);
        Ok(res)
    }

    /// Repeatedly parse the same elements from replay until `size` elements parsed
    fn repeat<F, T>(&mut self, size: usize, mut f: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        if size > 25_000 {
            return Err(ParseError::ListTooLarge(size));
        }

        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(f(self)?);
        }
        Ok(res)
    }

    fn list_of<F, T>(&mut self, f: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        let size = self.take(4, le_i32)?;
        self.repeat(size as usize, f)
    }

    fn text_list(&mut self) -> Result<Vec<Cow<'a, str>>, ParseError> {
        self.list_of(|s| s.parse_text())
    }

    /// Parses UTF-8 string from replay
    fn parse_str(&mut self) -> Result<&'a str, ParseError> {
        let size = self.take(4, le_i32)? as usize;
        self.take_res(size, decode_str)
    }

    /// Parses either UTF-16 or Windows-1252 encoded strings
    fn parse_text(&mut self) -> Result<Cow<'a, str>, ParseError> {
        // The number of bytes that the string is composed of. If negative, the string is UTF-16,
        // else the string is windows 1252 encoded.
        let characters = self.take(4, le_i32)?;

        // size.abs() will panic at min_value, so we eschew it for manual checking
        if characters == 0 {
            Err(ParseError::ZeroSize)
        } else if characters > 10_000 || characters < -10_000 {
            Err(ParseError::TextTooLarge(characters))
        } else if characters < 0 {
            // We're dealing with UTF-16 and each character is two bytes, we
            // multiply the size by 2. The last two bytes included in the count are
            // null terminators
            let size = characters * -2;
            self.take_res(size as usize, |d| decode_utf16(d))
        } else {
            self.take_res(characters as usize, |d| decode_windows1252(d))
        }
    }

    fn parse_rdict(&mut self) -> Result<Vec<(&'a str, HeaderProp<'a>)>, ParseError> {
        // Other the actual network data, the header property associative array is the hardest to parse.
        // The format is to:
        // - Read string
        // - If string is "None", we're done
        // - else we're dealing with a property, and the string just read is the key. Now deserialize the
        //   value.
        // The return type of this function is a key value vector because since there is no format
        // specification, we can't rule out duplicate keys. Possibly consider a multi-map in the future.

        let mut res: Vec<_> = Vec::new();
        loop {
            let key = self.parse_str()?;
            if key == "None" {
                break;
            }

            let val = match self.parse_str()? {
                "ArrayProperty" => self.array_property(),
                "BoolProperty" => self.bool_property(),
                "ByteProperty" => self.byte_property(),
                "FloatProperty" => self.float_property(),
                "IntProperty" => self.int_property(),
                "NameProperty" => self.name_property(),
                "QWordProperty" => self.qword_property(),
                "StrProperty" => self.str_property(),
                x => Err(ParseError::UnexpectedProperty(String::from(x))),
            }?;

            res.push((key, val));
        }

        Ok(res)
    }

    // Header properties are encoded in a pretty simple format, with some oddities. The first 64bits
    // is data that can be discarded, some people think that the 64bits is the length of the data
    // while others think that the first 32bits is the header length in bytes with the subsequent
    // 32bits unknown. Doesn't matter to us, we throw it out anyways. The rest of the bytes are
    // decoded property type specific.

    fn byte_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        // It's unknown (to me at least) why the byte property has two strings in it.
        self.take(8, |_d| ())?;
        self.parse_str()?;
        self.parse_str()?;
        Ok(HeaderProp::Byte)
    }

    fn str_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(8, |_d| ())?;
        Ok(HeaderProp::Str(self.parse_text()?))
    }

    fn name_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(8, |_d| ())?;
        Ok(HeaderProp::Name(self.parse_text()?))
    }

    fn int_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(12, |d| HeaderProp::Int(le_i32(&d[8..])))
    }

    fn bool_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(9, |d| HeaderProp::Bool(d[8] == 1))
    }

    fn float_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(12, |d| HeaderProp::Float(le_f32(&d[8..])))
    }

    fn qword_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        self.take(16, |d| HeaderProp::QWord(le_i64(&d[8..])))
    }

    fn array_property(&mut self) -> Result<HeaderProp<'a>, ParseError> {
        let size = self.take(12, |d| le_i32(&d[8..]))?;
        let arr = self.repeat(size as usize, |s| s.parse_rdict())?;
        Ok(HeaderProp::Array(arr))
    }

    fn parse_tickmarks(&mut self) -> Result<Vec<TickMark<'a>>, ParseError> {
        self.list_of(|s| {
            Ok(TickMark {
                description: s.parse_text()?,
                frame: s.take(4, le_i32)?,
            })
        })
    }

    fn parse_keyframe(&mut self) -> Result<Vec<KeyFrame>, ParseError> {
        self.list_of(|s| {
            Ok(KeyFrame {
                time: s.take(4, le_f32)?,
                frame: s.take(4, le_i32)?,
                position: s.take(4, le_i32)?,
            })
        })
    }

    fn parse_debuginfo(&mut self) -> Result<Vec<DebugInfo<'a>>, ParseError> {
        self.list_of(|s| {
            Ok(DebugInfo {
                frame: s.take(4, le_i32)?,
                user: s.parse_text()?,
                text: s.parse_text()?,
            })
        })
    }

    fn parse_classindex(&mut self) -> Result<Vec<ClassIndex<'a>>, ParseError> {
        self.list_of(|s| {
            Ok(ClassIndex {
                class: s.parse_str()?,
                index: s.take(4, le_i32)?,
            })
        })
    }

    fn parse_cacheprop(&mut self) -> Result<Vec<CacheProp>, ParseError> {
        self.list_of(|s| {
            Ok(CacheProp {
                object_ind: s.take(4, le_i32)?,
                stream_id: s.take(4, le_i32)?,
            })
        })
    }

    fn parse_classcache(&mut self) -> Result<Vec<ClassNetCache>, ParseError> {
        self.list_of(|x| {
            Ok(ClassNetCache {
                object_ind: x.take(4, le_i32)?,
                parent_id: x.take(4, le_i32)?,
                cache_id: x.take(4, le_i32)?,
                properties: x.parse_cacheprop()?,
            })
        })
    }
}

const MULTIPLY_DE_BRUIJN_BIT_POSITION2: [u32; 32] = 
[
  0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8, 
  31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
];


// https://graphics.stanford.edu/~seander/bithacks.html#IntegerLogDeBruijn
fn log2(v: u32) -> u32 {
	MULTIPLY_DE_BRUIJN_BIT_POSITION2[((v.wrapping_mul(0x077CB531)) >> 27) as usize]
}

/// Reads a string of a given size from the data. The size includes a null
/// character as the last character, so we drop it in the returned string
/// slice. It may seem redundant to store this information, but stackoverflow
/// contains a nice reasoning for why it may have been done this way:
/// http://stackoverflow.com/q/6293457/433785
fn decode_str(input: &[u8]) -> Result<&str, ParseError> {
    if input.is_empty() {
        Err(ParseError::ZeroSize)
    } else {
        Ok(::std::str::from_utf8(&input[..input.len() - 1])?)
    }
}

pub fn decode_utf16(input: &[u8]) -> Result<Cow<str>, ParseError> {
    if input.len() < 2 {
        Err(ParseError::ZeroSize)
    } else {
        let (s, _) = UTF_16LE.decode_without_bom_handling(&input[..input.len() - 2]);
        Ok(s)
    }
}

pub fn decode_windows1252(input: &[u8]) -> Result<Cow<str>, ParseError> {
    if input.is_empty() {
        Err(ParseError::ZeroSize)
    } else {
        let (s, _) = WINDOWS_1252.decode_without_bom_handling(&input[..input.len() - 1]);
        Ok(s)
    }
}

#[inline]
fn le_i32(d: &[u8]) -> i32 {
    LittleEndian::read_i32(d)
}

#[inline]
fn le_f32(d: &[u8]) -> f32 {
    LittleEndian::read_f32(d)
}

#[inline]
fn le_i64(d: &[u8]) -> i64 {
    LittleEndian::read_i64(d)
}

#[cfg(test)]
mod tests {
    use super::{CrcCheck, Parser, NetworkParse};
    use errors::ParseError;
    use models::{HeaderProp, TickMark};
    use std::borrow::Cow;

    #[test]
    fn parse_text_encoding() {
        // dd skip=16 count=28 if=rumble.replay of=text.replay bs=1
        let data = include_bytes!("../assets/text.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        assert_eq!(parser.parse_str().unwrap(), "TAGame.Replay_Soccar_TA");
    }

    #[test]
    fn parse_text_encoding_bad() {
        // dd skip=16 count=28 if=rumble.replay of=text.replay bs=1
        let data = include_bytes!("../assets/text.replay");
        let mut parser = Parser::new(&data[..data.len() - 1], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_str();
        assert!(res.is_err());
        let error = res.unwrap_err();
        assert_eq!(error, ParseError::InsufficientData(24, 23));
    }

    #[test]
    fn parse_text_zero_size() {
        let mut parser = Parser::new(&[0, 0, 0, 0, 0], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_str();
        assert!(res.is_err());
        let error = res.unwrap_err();
        assert_eq!(error, ParseError::ZeroSize);
    }

    #[test]
    fn parse_text_encoding_bad_2() {
        // Test for when there is not enough data to decode text length
        // dd skip=16 count=28 if=rumble.replay of=text.replay bs=1
        let data = include_bytes!("../assets/text.replay");
        let mut parser = Parser::new(&data[..2], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_str();
        assert!(res.is_err());
        let error = res.unwrap_err();
        assert_eq!(error, ParseError::InsufficientData(4, 2));
    }

    #[test]
    fn parse_utf16_string() {
        // dd skip=((0x120)) count=28 if=utf-16.replay of=utf-16-text.replay bs=1
        let data = include_bytes!("../assets/utf-16-text.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_text().unwrap();
        assert_eq!(res, "\u{2623}D[e]!v1zz\u{2623}");
    }

    #[test]
    fn test_windows1252_string() {
        let data = include_bytes!("../assets/windows_1252.replay");
        let mut parser = Parser::new(&data[0x1ad..0x1c4], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_text().unwrap();
        assert_eq!(res, "caudillman6000\u{b3}(2)");
    }

    /// Define behavior on invalid UTF-16 sequences.
    #[test]
    fn parse_invalid_utf16_string() {
        let data = [0xfd, 0xff, 0xff, 0xff, 0xd8, 0xd8, 0x00, 0x00, 0x00, 0x00];
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_text().unwrap();
        assert_eq!(res, "�\u{0}");
    }

    #[test]
    fn rdict_no_elements() {
        let data = [0x05, 0x00, 0x00, 0x00, b'N', b'o', b'n', b'e', 0x00];
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(res, Vec::new());
    }

    #[test]
    fn rdict_one_element() {
        // dd skip=$((0x1269)) count=$((0x12a8 - 0x1269)) if=rumble.replay of=rdict_one.replay bs=1
        let data = include_bytes!("../assets/rdict_one.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(
            res,
            vec![("PlayerName", HeaderProp::Str(Cow::Borrowed("comagoosie")))]
        );
    }

    #[test]
    fn rdict_one_int_element() {
        // dd skip=$((0x250)) count=$((0x284 - 0x250)) if=rumble.replay of=rdict_int.replay bs=1
        let data = include_bytes!("../assets/rdict_int.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(res, vec![("PlayerTeam", HeaderProp::Int(0))]);
    }

    #[test]
    fn rdict_one_bool_element() {
        // dd skip=$((0xa0f)) count=$((0xa3b - 0xa0f)) if=rumble.replay of=rdict_bool.replay bs=1
        let data = include_bytes!("../assets/rdict_bool.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(res, vec![("bBot", HeaderProp::Bool(false))]);
    }

    fn append_none(input: &[u8]) -> Vec<u8> {
        let append = [0x05, 0x00, 0x00, 0x00, b'N', b'o', b'n', b'e', 0x00];
        let mut v = Vec::new();
        v.extend_from_slice(input);
        v.extend_from_slice(&append);
        v
    }

    #[test]
    fn rdict_one_name_element() {
        // dd skip=$((0x1237)) count=$((0x1269 - 0x1237)) if=rumble.replay of=rdict_name.replay bs=1
        let data = append_none(include_bytes!("../assets/rdict_name.replay"));
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(
            res,
            vec![("MatchType", HeaderProp::Name(Cow::Borrowed("Online")))]
        );
    }

    #[test]
    fn rdict_one_float_element() {
        // dd skip=$((0x10a2)) count=$((0x10ce - 0x10a2)) if=rumble.replay of=rdict_float.replay bs=1
        let data = append_none(include_bytes!("../assets/rdict_float.replay"));
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(res, vec![("RecordFPS", HeaderProp::Float(30.0))]);
    }

    #[test]
    fn rdict_one_qword_element() {
        // dd skip=$((0x576)) count=$((0x5a5 - 0x576)) if=rumble.replay of=rdict_qword.replay bs=1
        let data = append_none(include_bytes!("../assets/rdict_qword.replay"));
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(
            res,
            vec![("OnlineID", HeaderProp::QWord(76561198101748375))]
        );
    }

    #[test]
    fn rdict_one_array_element() {
        // dd skip=$((0xab)) count=$((0x3f7 + 36)) if=rumble.replay of=rdict_array.replay bs=1
        let data = append_none(include_bytes!("../assets/rdict_array.replay"));
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        let expected = vec![
            vec![
                ("frame", HeaderProp::Int(441)),
                ("PlayerName", HeaderProp::Str(Cow::Borrowed("Cakeboss"))),
                ("PlayerTeam", HeaderProp::Int(1)),
            ],
            vec![
                ("frame", HeaderProp::Int(1738)),
                ("PlayerName", HeaderProp::Str(Cow::Borrowed("Sasha Kaun"))),
                ("PlayerTeam", HeaderProp::Int(0)),
            ],
            vec![
                ("frame", HeaderProp::Int(3504)),
                (
                    "PlayerName",
                    HeaderProp::Str(Cow::Borrowed("SilentWarrior")),
                ),
                ("PlayerTeam", HeaderProp::Int(0)),
            ],
            vec![
                ("frame", HeaderProp::Int(5058)),
                ("PlayerName", HeaderProp::Str(Cow::Borrowed("jeffreyj1"))),
                ("PlayerTeam", HeaderProp::Int(1)),
            ],
            vec![
                ("frame", HeaderProp::Int(5751)),
                ("PlayerName", HeaderProp::Str(Cow::Borrowed("GOOSE LORD"))),
                ("PlayerTeam", HeaderProp::Int(0)),
            ],
            vec![
                ("frame", HeaderProp::Int(6083)),
                ("PlayerName", HeaderProp::Str(Cow::Borrowed("GOOSE LORD"))),
                ("PlayerTeam", HeaderProp::Int(0)),
            ],
            vec![
                ("frame", HeaderProp::Int(7021)),
                (
                    "PlayerName",
                    HeaderProp::Str(Cow::Borrowed("SilentWarrior")),
                ),
                ("PlayerTeam", HeaderProp::Int(0)),
            ],
        ];
        assert_eq!(res, vec![("Goals", HeaderProp::Array(expected))]);
    }

    #[test]
    fn rdict_one_byte_element() {
        // dd skip=$((0xdf0)) count=$((0xe41 - 0xdf0)) if=rumble.replay of=rdict_byte.replay bs=1
        let data = append_none(include_bytes!("../assets/rdict_byte.replay"));
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let res = parser.parse_rdict().unwrap();
        assert_eq!(res, vec![("Platform", HeaderProp::Byte)]);
    }

    #[test]
    fn key_frame_list() {
        let data = include_bytes!("../assets/rumble.replay");

        // List is 2A long, each keyframe is 12 bytes. Then add four for list length = 508
        let mut parser = Parser::new(&data[0x12ca..0x12ca + 508], CrcCheck::Never, NetworkParse::Never);
        let frames = parser.parse_keyframe().unwrap();
        assert_eq!(frames.len(), 42);
    }

    #[test]
    fn tickmark_list() {
        let data = include_bytes!("../assets/rumble.replay");

        // 7 tick marks at 8 bytes + size of tick list
        let mut parser = Parser::new(&data[0xf6cce..0xf6d50], CrcCheck::Never, NetworkParse::Never);
        let ticks = parser.parse_tickmarks().unwrap();

        assert_eq!(ticks.len(), 7);
        assert_eq!(
            ticks[0],
            TickMark {
                description: Cow::Borrowed("Team1Goal"),
                frame: 396,
            }
        );
    }

    #[test]
    fn test_the_whole_shebang() {
        let data = include_bytes!("../assets/rumble.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        assert!(parser.parse().is_ok())
    }

    #[test]
    fn test_the_parsing_empty() {
        let mut parser = Parser::new(&[], CrcCheck::Never, NetworkParse::Never);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_the_parsing_text_too_long() {
        let data = include_bytes!("../assets/fuzz-string-too-long.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        assert!(parser.parse().is_err())
    }

    #[test]
    fn test_fuzz_corpus_slice_index() {
        let data = include_bytes!("../assets/fuzz-slice-index.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        assert!(parser.parse().is_err())
    }

    #[test]
    fn test_the_fuzz_corpus_abs_panic() {
        let data = include_bytes!("../assets/fuzz-corpus.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        assert!(parser.parse().is_err())
    }

    #[test]
    fn test_the_fuzz_corpus_large_list() {
        let data = include_bytes!("../assets/fuzz-list-too-large.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Never, NetworkParse::Never);
        let err = parser.parse().unwrap_err();
        assert!(
            format!("{}", err).starts_with(
                "Could not decode replay debug info at offset (1010894): list of size"
            )
        );
    }

    #[test]
    fn test_the_fuzz_corpus_large_list_on_error_crc() {
        let data = include_bytes!("../assets/fuzz-list-too-large.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::OnError, NetworkParse::Never);
        let err = parser.parse().unwrap_err();
        assert_eq!(
            "Failed to parse body and crc check failed. Replay is corrupt",
            format!("{}", err)
        );

        assert!(
            format!("{}", err.cause().cause().unwrap()).starts_with(
                "Could not decode replay debug info at offset (1010894): list of size"
            )
        );
    }

    #[test]
    fn test_the_fuzz_corpus_large_list_always_crc() {
        let data = include_bytes!("../assets/fuzz-list-too-large.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Never);
        let err = parser.parse().unwrap_err();
        assert_eq!(
            "Crc mismatch. Expected 3765941959 but received 1314727725",
            format!("{}", err)
        );
        assert!(err.cause().cause().is_none());
    }

    #[test]
    fn test_the_whole_shebang_with_crc() {
        let data = include_bytes!("../assets/rumble.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Never);
        assert!(parser.parse().is_ok())
    }

    #[test]
    fn test_net_version() {
        let data = include_bytes!("../assets/netversion.replay");
        let mut parser = Parser::new(&data[8..], CrcCheck::Always, NetworkParse::Never);
        let header = parser.parse_header().unwrap();
        assert_eq!(header.major_version, 868);
        assert_eq!(header.net_version, Some(2));
    }

    #[test]
    fn test_no_frames() {
        let data = include_bytes!("../assets/no-frames.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Always);
        let replay = parser.parse().unwrap();
        let expected: Vec<_> = Vec::new();
        assert_eq!(replay.network_frames.unwrap().frames, expected);
    }

    #[test]
    fn test_small_frames() {
        let data = include_bytes!("../assets/small-frames.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Always);
        let replay = parser.parse().unwrap();
        assert_eq!(replay.network_frames.unwrap().frames.len(), 231);
    }

    #[test]
    fn test_3381_replay() {
        let data = include_bytes!("../assets/3381.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Always);
        let replay = parser.parse().unwrap();
        assert_eq!(replay.network_frames.unwrap().frames.len(), 13320);
    }

    #[test]
    fn test_rumble_body() {
        let data = include_bytes!("../assets/rumble.replay");
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Always);
        assert!(parser.parse().is_ok());
    }

    #[test]
    fn test_crc_check_with_bad() {
        let mut data = include_bytes!("../assets/rumble.replay").to_vec();

        // Changing this byte won't make the parsing fail but will make the crc check fail
        data[4775] = 100;
        let mut parser = Parser::new(&data[..], CrcCheck::Always, NetworkParse::Never);
        let res = parser.parse();
        assert!(res.is_err());
        assert_eq!(
            "Crc mismatch. Expected 337843175 but received 2877465516",
            format!("{}", res.unwrap_err())
        );

        parser = Parser::new(&data[..], CrcCheck::OnError, NetworkParse::Never);
        assert!(parser.parse().is_ok());
    }
}
