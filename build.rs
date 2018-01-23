extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "use attributes::AttributeTag;\n ").unwrap();
    write!(&mut file, "use phf;\n ").unwrap();
    write!(&mut file, "use network::SpawnTrajectory;\n ").unwrap();

    write!(&mut file, "pub static SPAWN_STATS: phf::Map<&'static str, SpawnTrajectory> = ").unwrap();
    phf_codegen::Map::new()
        .entry("TAGame.Ball_Breakout_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Breakout", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Ball_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_BasketBall_Mutator", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_BasketBall", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Default", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.Ball_Puck", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Ball.CubeBall", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Car_Season_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("TAGame.Car_TA", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.Car.Car_Default", "SpawnTrajectory::LocationAndRotation")
        .entry("Archetypes.GameEvent.GameEvent_Season:CarArchetype", "SpawnTrajectory::LocationAndRotation")

        .entry("TAGame.CameraSettingsActor_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Boost_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Dodge_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_DoubleJump_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_FlipCar_TA", "SpawnTrajectory::Location")
        .entry("TAGame.CarComponent_Jump_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_Season_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_Soccar_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_SoccarPrivate_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GameEvent_SoccarSplitscreen_TA", "SpawnTrajectory::Location")
        .entry("TAGame.GRI_TA", "SpawnTrajectory::Location")
        .entry("TAGame.PRI_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallCarSpring_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallFreeze_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallGravity_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallLasso_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BallVelcro_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Batarang_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_BoostOverride_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_GrapplingHook_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_HitForce_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Swapper_TA", "SpawnTrajectory::Location")
        .entry("TAGame.SpecialPickup_Tornado_TA", "SpawnTrajectory::Location")
        .entry("TAGame.Team_Soccar_TA", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Boost", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Dodge", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_DoubleJump", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_FlipCar", "SpawnTrajectory::Location")
        .entry("Archetypes.CarComponents.CarComponent_Jump", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Basketball", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_BasketballPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_BasketballSplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Breakout", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Hockey", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_HockeyPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_HockeySplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Items", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Season", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_Soccar", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_SoccarPrivate", "SpawnTrajectory::Location")
        .entry("Archetypes.GameEvent.GameEvent_SoccarSplitscreen", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallFreeze", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallLasso", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallSpring", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallVelcro", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Batarang", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BoostOverride", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_CarSpring", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_GravityWell", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_StrongHit", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Swapper", "SpawnTrajectory::Location")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Tornado", "SpawnTrajectory::Location")
        .entry("Archetypes.Teams.Team0", "SpawnTrajectory::Location")
        .entry("Archetypes.Teams.Team1", "SpawnTrajectory::Location")
        .entry("GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype", "SpawnTrajectory::Location")
        .entry("TAGame.Default__CameraSettingsActor_TA", "SpawnTrajectory::Location")
        .entry("TAGame.Default__PRI_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.BreakOutActor_Platform_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.CrowdActor_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.CrowdManager_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.InMapScoreboard_TA", "SpawnTrajectory::Location")
        .entry("TheWorld:PersistentLevel.VehiclePickup_Boost_TA", "SpawnTrajectory::Location")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static ATTRIBUTES: phf::Map<&'static str, AttributeTag> = ").unwrap();
    phf_codegen::Map::new()
        .entry("Engine.Actor:bBlockActors", "AttributeTag::Boolean")
        .entry("Engine.Actor:bCollideActors", "AttributeTag::Boolean")
        .entry("Engine.Actor:bHidden", "AttributeTag::Boolean")
        .entry("Engine.Actor:DrawScale", "AttributeTag::Float")
        .entry("Engine.Actor:Role", "AttributeTag::Enum")
        .entry("Engine.GameReplicationInfo:bMatchIsOver", "AttributeTag::Boolean")
        .entry("Engine.GameReplicationInfo:GameClass", "AttributeTag::Flagged")
        .entry("Engine.GameReplicationInfo:ServerName", "AttributeTag::String")
        .entry("Engine.Pawn:PlayerReplicationInfo", "AttributeTag::Flagged")
        .entry("Engine.PlayerReplicationInfo:bBot", "AttributeTag::Boolean")
        .entry("Engine.PlayerReplicationInfo:bIsSpectator", "AttributeTag::Boolean")
        .entry("Engine.PlayerReplicationInfo:bReadyToPlay", "AttributeTag::Boolean")
        .entry("Engine.PlayerReplicationInfo:bWaitingPlayer", "AttributeTag::Boolean")
        .entry("Engine.PlayerReplicationInfo:Ping", "AttributeTag::Byte")
        .entry("Engine.PlayerReplicationInfo:PlayerID", "AttributeTag::Int")
        .entry("Engine.PlayerReplicationInfo:PlayerName", "AttributeTag::String")
        .entry("Engine.PlayerReplicationInfo:RemoteUserData", "AttributeTag::String")
        .entry("Engine.PlayerReplicationInfo:Score", "AttributeTag::Int")
        .entry("Engine.PlayerReplicationInfo:Team", "AttributeTag::Flagged")
        .entry("Engine.PlayerReplicationInfo:UniqueId", "AttributeTag::UniqueId")
        .entry("Engine.TeamInfo:Score", "AttributeTag::Int")
        .entry("ProjectX.GRI_X:bGameStarted", "AttributeTag::Boolean")
        .entry("ProjectX.GRI_X:GameServerID", "AttributeTag::QWord")
        .entry("ProjectX.GRI_X:MatchGUID", "AttributeTag::String")
        .entry("ProjectX.GRI_X:ReplicatedGameMutatorIndex", "AttributeTag::Int")
        .entry("ProjectX.GRI_X:ReplicatedGamePlaylist", "AttributeTag::Int")
        .entry("ProjectX.GRI_X:Reservations", "AttributeTag::Reservation")
        .entry("TAGame.Ball_Breakout_TA:AppliedDamage", "AttributeTag::AppliedDamage")
        .entry("TAGame.Ball_Breakout_TA:DamageIndex", "AttributeTag::Int")
        .entry("TAGame.Ball_Breakout_TA:LastTeamTouch", "AttributeTag::Byte")
        .entry("TAGame.Ball_TA:GameEvent", "AttributeTag::Flagged")
        .entry("TAGame.Ball_TA:HitTeamNum", "AttributeTag::Byte")
        .entry("TAGame.Ball_TA:ReplicatedAddedCarBounceScale", "AttributeTag::Float")
        .entry("TAGame.Ball_TA:ReplicatedBallMaxLinearSpeedScale", "AttributeTag::Float")
        .entry("TAGame.Ball_TA:ReplicatedBallScale", "AttributeTag::Float")
        .entry("TAGame.Ball_TA:ReplicatedExplosionData", "AttributeTag::Explosion")
        .entry("TAGame.Ball_TA:ReplicatedExplosionDataExtended", "AttributeTag::ExtendedExplosion")
        .entry("TAGame.Ball_TA:ReplicatedWorldBounceScale", "AttributeTag::Float")
        .entry("TAGame.BreakOutActor_Platform_TA:DamageState", "AttributeTag::DamageState")
        .entry("TAGame.CameraSettingsActor_TA:bUsingBehindView", "AttributeTag::Boolean")
        .entry("TAGame.CameraSettingsActor_TA:bUsingSecondaryCamera", "AttributeTag::Boolean")
        .entry("TAGame.CameraSettingsActor_TA:CameraPitch", "AttributeTag::Byte")
        .entry("TAGame.CameraSettingsActor_TA:CameraYaw", "AttributeTag::Byte")
        .entry("TAGame.CameraSettingsActor_TA:PRI", "AttributeTag::Flagged")
        .entry("TAGame.CameraSettingsActor_TA:ProfileSettings", "AttributeTag::CamSettings")
        .entry("TAGame.Car_TA:AddedBallForceMultiplier", "AttributeTag::Float")
        .entry("TAGame.Car_TA:AddedCarForceMultiplier", "AttributeTag::Float")
        .entry("TAGame.Car_TA:AttachedPickup", "AttributeTag::Flagged")
        .entry("TAGame.Car_TA:ClubColors", "AttributeTag::ClubColors")
        .entry("TAGame.Car_TA:ReplicatedDemolish", "AttributeTag::Demolish")
        .entry("TAGame.Car_TA:TeamPaint", "AttributeTag::TeamPaint")
        .entry("TAGame.CarComponent_Boost_TA:bNoBoost", "AttributeTag::Boolean")
        .entry("TAGame.CarComponent_Boost_TA:BoostModifier", "AttributeTag::Float")
        .entry("TAGame.CarComponent_Boost_TA:bUnlimitedBoost", "AttributeTag::Boolean")
        .entry("TAGame.CarComponent_Boost_TA:RechargeDelay", "AttributeTag::Float")
        .entry("TAGame.CarComponent_Boost_TA:RechargeRate", "AttributeTag::Float")
        .entry("TAGame.CarComponent_Boost_TA:ReplicatedBoostAmount", "AttributeTag::Byte")
        .entry("TAGame.CarComponent_Boost_TA:UnlimitedBoostRefCount", "AttributeTag::Int")
        .entry("TAGame.CarComponent_Dodge_TA:DodgeTorque", "AttributeTag::Location")
        .entry("TAGame.CarComponent_FlipCar_TA:bFlipRight", "AttributeTag::Boolean")
        .entry("TAGame.CarComponent_FlipCar_TA:FlipCarTime", "AttributeTag::Float")
        .entry("TAGame.CarComponent_TA:ReplicatedActive", "AttributeTag::Byte")
        .entry("TAGame.CarComponent_TA:ReplicatedActivityTime", "AttributeTag::Float")
        .entry("TAGame.CarComponent_TA:Vehicle", "AttributeTag::Flagged")
        .entry("TAGame.CrowdActor_TA:GameEvent", "AttributeTag::Flagged")
        .entry("TAGame.CrowdActor_TA:ModifiedNoise", "AttributeTag::Float")
        .entry("TAGame.CrowdActor_TA:ReplicatedCountDownNumber", "AttributeTag::Int")
        .entry("TAGame.CrowdActor_TA:ReplicatedOneShotSound", "AttributeTag::Flagged")
        .entry("TAGame.CrowdActor_TA:ReplicatedRoundCountDownNumber", "AttributeTag::Int")
        .entry("TAGame.CrowdManager_TA:GameEvent", "AttributeTag::Flagged")
        .entry("TAGame.CrowdManager_TA:ReplicatedGlobalOneShotSound", "AttributeTag::Flagged")
        .entry("TAGame.GameEvent_Soccar_TA:bBallHasBeenHit", "AttributeTag::Boolean")
        .entry("TAGame.GameEvent_Soccar_TA:bOverTime", "AttributeTag::Boolean")
        .entry("TAGame.GameEvent_Soccar_TA:GameTime", "AttributeTag::Int")
        .entry("TAGame.GameEvent_Soccar_TA:ReplicatedMusicStinger", "AttributeTag::MusicStinger")
        .entry("TAGame.GameEvent_Soccar_TA:ReplicatedScoredOnTeam", "AttributeTag::Byte")
        .entry("TAGame.GameEvent_Soccar_TA:RoundNum", "AttributeTag::Int")
        .entry("TAGame.GameEvent_Soccar_TA:SecondsRemaining", "AttributeTag::Int")
        .entry("TAGame.GameEvent_Soccar_TA:SubRulesArchetype", "AttributeTag::Flagged")
        .entry("TAGame.GameEvent_SoccarPrivate_TA:MatchSettings", "AttributeTag::PrivateMatchSettings")
        .entry("TAGame.GameEvent_TA:bCanVoteToForfeit", "AttributeTag::Boolean")
        .entry("TAGame.GameEvent_TA:bHasLeaveMatchPenalty", "AttributeTag::Boolean")
        .entry("TAGame.GameEvent_TA:BotSkill", "AttributeTag::Int")
        .entry("TAGame.GameEvent_TA:GameMode", "AttributeTag::GameMode")
        .entry("TAGame.GameEvent_TA:MatchTypeClass", "AttributeTag::Flagged")
        .entry("TAGame.GameEvent_TA:ReplicatedGameStateTimeRemaining", "AttributeTag::Int")
        .entry("TAGame.GameEvent_TA:ReplicatedStateIndex", "AttributeTag::Byte")
        .entry("TAGame.GameEvent_TA:ReplicatedStateName", "AttributeTag::Int")
        .entry("TAGame.GameEvent_Team_TA:bForfeit", "AttributeTag::Boolean")
        .entry("TAGame.GameEvent_Team_TA:MaxTeamSize", "AttributeTag::Int")
        .entry("TAGame.GRI_TA:NewDedicatedServerIP", "AttributeTag::String")
        .entry("TAGame.PRI_TA:bIsInSplitScreen", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bMatchMVP", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bOnlineLoadoutSet", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bOnlineLoadoutsSet", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:BotProductName", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:bReady", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bUsingBehindView", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bUsingItems", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:bUsingSecondaryCamera", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:CameraPitch", "AttributeTag::Byte")
        .entry("TAGame.PRI_TA:CameraSettings", "AttributeTag::CamSettings")
        .entry("TAGame.PRI_TA:CameraYaw", "AttributeTag::Byte")
        .entry("TAGame.PRI_TA:ClientLoadout", "AttributeTag::Loadout")
        .entry("TAGame.PRI_TA:ClientLoadoutOnline", "AttributeTag::LoadoutOnline")
        .entry("TAGame.PRI_TA:ClientLoadouts", "AttributeTag::TeamLoadout")
        .entry("TAGame.PRI_TA:ClientLoadoutsOnline", "AttributeTag::LoadoutsOnline")
        .entry("TAGame.PRI_TA:MatchAssists", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MatchBreakoutDamage", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MatchGoals", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MatchSaves", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MatchScore", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MatchShots", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:MaxTimeTillItem", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:PartyLeader", "AttributeTag::PartyLeader")
        .entry("TAGame.PRI_TA:PawnType", "AttributeTag::Byte")
        .entry("TAGame.PRI_TA:PersistentCamera", "AttributeTag::Flagged")
        .entry("TAGame.PRI_TA:PlayerHistoryValid", "AttributeTag::Boolean")
        .entry("TAGame.PRI_TA:ReplicatedGameEvent", "AttributeTag::Flagged")
        .entry("TAGame.PRI_TA:SteeringSensitivity", "AttributeTag::Float")
        .entry("TAGame.PRI_TA:SkillTier", "AttributeTag::FlaggedByte")
        .entry("TAGame.PRI_TA:TimeTillItem", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:Title", "AttributeTag::Int")
        .entry("TAGame.PRI_TA:TotalXP", "AttributeTag::Int")
        .entry("TAGame.RBActor_TA:bFrozen", "AttributeTag::Boolean")
        .entry("TAGame.RBActor_TA:bIgnoreSyncing", "AttributeTag::Boolean")
        .entry("TAGame.RBActor_TA:bReplayActor", "AttributeTag::Boolean")
        .entry("TAGame.RBActor_TA:ReplicatedRBState", "AttributeTag::RigidBody")
        .entry("TAGame.RBActor_TA:WeldedInfo", "AttributeTag::Welded")
        .entry("TAGame.SpecialPickup_BallFreeze_TA:RepOrigSpeed", "AttributeTag::Float")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:AttachTime", "AttributeTag::Float")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:bBroken", "AttributeTag::Boolean")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:bHit", "AttributeTag::Boolean")
        .entry("TAGame.SpecialPickup_BallVelcro_TA:BreakTime", "AttributeTag::Float")
        .entry("TAGame.SpecialPickup_Targeted_TA:Targeted", "AttributeTag::Flagged")
        .entry("TAGame.Team_Soccar_TA:GameScore", "AttributeTag::Int")
        .entry("TAGame.Team_TA:ClubColors", "AttributeTag::ClubColors")
        .entry("TAGame.Team_TA:CustomTeamName", "AttributeTag::String")
        .entry("TAGame.Team_TA:GameEvent", "AttributeTag::Flagged")
        .entry("TAGame.Team_TA:LogoData", "AttributeTag::Flagged")
        .entry("TAGame.Vehicle_TA:bDriving", "AttributeTag::Boolean")
        .entry("TAGame.Vehicle_TA:bReplicatedHandbrake", "AttributeTag::Boolean")
        .entry("TAGame.Vehicle_TA:ReplicatedSteer", "AttributeTag::Byte")
        .entry("TAGame.Vehicle_TA:ReplicatedThrottle", "AttributeTag::Byte")
        .entry("TAGame.VehiclePickup_TA:bNoPickup", "AttributeTag::Boolean")
        .entry("TAGame.VehiclePickup_TA:ReplicatedPickupData", "AttributeTag::Pickup")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static OBJECT_CLASSES: phf::Map<&'static str, &'static str> = ").unwrap();

    phf_codegen::Map::new()
        .entry("Archetypes.Ball.Ball_BasketBall_Mutator", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Basketball", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Breakout", "\"TAGame.Ball_Breakout_TA\"")
        .entry("Archetypes.Ball.Ball_Default", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.Ball_Puck", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Ball.CubeBall", "\"TAGame.Ball_TA\"")
        .entry("Archetypes.Car.Car_Default", "\"TAGame.Car_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Boost", "\"TAGame.CarComponent_Boost_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Dodge", "\"TAGame.CarComponent_Dodge_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_DoubleJump", "\"TAGame.CarComponent_DoubleJump_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_FlipCar", "\"TAGame.CarComponent_FlipCar_TA\"")
        .entry("Archetypes.CarComponents.CarComponent_Jump", "\"TAGame.CarComponent_Jump_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Basketball", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_BasketballPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_BasketballSplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Breakout", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Hockey", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_HockeyPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_HockeySplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Items", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Season:CarArchetype", "\"TAGame.Car_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Season", "\"TAGame.GameEvent_Season_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_Soccar", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_SoccarPrivate", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("Archetypes.GameEvent.GameEvent_SoccarSplitscreen", "\"TAGame.GameEvent_SoccarSplitscreen_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallFreeze", "\"TAGame.SpecialPickup_BallFreeze_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook", "\"TAGame.SpecialPickup_GrapplingHook_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallLasso", "\"TAGame.SpecialPickup_BallLasso_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallSpring", "\"TAGame.SpecialPickup_BallCarSpring_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BallVelcro", "\"TAGame.SpecialPickup_BallVelcro_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Batarang", "\"TAGame.SpecialPickup_Batarang_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_BoostOverride", "\"TAGame.SpecialPickup_BoostOverride_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_CarSpring", "\"TAGame.SpecialPickup_BallCarSpring_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_GravityWell", "\"TAGame.SpecialPickup_BallGravity_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_StrongHit", "\"TAGame.SpecialPickup_HitForce_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Swapper", "\"TAGame.SpecialPickup_Swapper_TA\"")
        .entry("Archetypes.SpecialPickups.SpecialPickup_Tornado", "\"TAGame.SpecialPickup_Tornado_TA\"")
        .entry("Archetypes.Teams.Team0", "\"TAGame.Team_Soccar_TA\"")
        .entry("Archetypes.Teams.Team1", "\"TAGame.Team_Soccar_TA\"")
        .entry("GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype", "\"TAGame.GRI_TA\"")
        .entry("TAGame.Default__CameraSettingsActor_TA", "\"TAGame.CameraSettingsActor_TA\"")
        .entry("TAGame.Default__PRI_TA", "\"TAGame.PRI_TA\"")
        .entry("TheWorld:PersistentLevel.BreakOutActor_Platform_TA", "\"TAGame.BreakOutActor_Platform_TA\"")
        .entry("TheWorld:PersistentLevel.CrowdActor_TA", "\"TAGame.CrowdActor_TA\"")
        .entry("TheWorld:PersistentLevel.CrowdManager_TA", "\"TAGame.CrowdManager_TA\"")
        .entry("TheWorld:PersistentLevel.InMapScoreboard_TA", "\"TAGame.InMapScoreboard_TA\"")
        .entry("TheWorld:PersistentLevel.VehiclePickup_Boost_TA", "\"TAGame.VehiclePickup_Boost_TA\"")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();


    write!(&mut file, "pub static PARENT_CLASSES: phf::Map<&'static str, &'static str> = ").unwrap();

    phf_codegen::Map::new()
        .entry("Engine.Actor", "\"Core.Object\"")
        .entry("Engine.GameReplicationInfo", "\"Engine.ReplicationInfo\"")
        .entry("Engine.Info", "\"Engine.Actor\"")
        .entry("Engine.Pawn", "\"Engine.Actor\"")
        .entry("Engine.PlayerReplicationInfo", "\"Engine.ReplicationInfo\"")
        .entry("Engine.ReplicationInfo", "\"Engine.Info\"")
        .entry("Engine.TeamInfo", "\"Engine.ReplicationInfo\"")
        .entry("ProjectX.GRI_X", "\"Engine.GameReplicationInfo\"")
        .entry("ProjectX.Pawn_X", "\"Engine.Pawn\"")
        .entry("ProjectX.PRI_X", "\"Engine.PlayerReplicationInfo\"")
        .entry("TAGame.Ball_TA", "\"TAGame.RBActor_TA\"")
        .entry("TAGame.CameraSettingsActor_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.Car_Season_TA", "\"TAGame.PRI_TA\"")
        .entry("TAGame.Car_TA", "\"TAGame.Vehicle_TA\"")
        .entry("TAGame.CarComponent_Boost_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_Dodge_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_DoubleJump_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_FlipCar_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_Jump_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.CarComponent_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.CrowdActor_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.CrowdManager_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.GameEvent_Season_TA", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("TAGame.GameEvent_Soccar_TA", "\"TAGame.GameEvent_Team_TA\"")
        .entry("TAGame.GameEvent_SoccarPrivate_TA", "\"TAGame.GameEvent_Soccar_TA\"")
        .entry("TAGame.GameEvent_SoccarSplitscreen_TA", "\"TAGame.GameEvent_SoccarPrivate_TA\"")
        .entry("TAGame.GameEvent_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.GameEvent_Team_TA", "\"TAGame.GameEvent_TA\"")
        .entry("TAGame.GRI_TA", "\"ProjectX.GRI_X\"")
        .entry("TAGame.InMapScoreboard_TA", "\"Engine.Actor\"")
        .entry("TAGame.PRI_TA", "\"ProjectX.PRI_X\"")
        .entry("TAGame.RBActor_TA", "\"ProjectX.Pawn_X\"")
        .entry("TAGame.SpecialPickup_BallCarSpring_TA", "\"TAGame.SpecialPickup_Spring_TA\"")
        .entry("TAGame.SpecialPickup_BallFreeze_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_BallGravity_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_BallLasso_TA", "\"TAGame.SpecialPickup_GrapplingHook_TA\"")
        .entry("TAGame.SpecialPickup_BallVelcro_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Batarang_TA", "\"TAGame.SpecialPickup_BallLasso_TA\"")
        .entry("TAGame.SpecialPickup_BoostOverride_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_GrapplingHook_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_HitForce_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Spring_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_Swapper_TA", "\"TAGame.SpecialPickup_Targeted_TA\"")
        .entry("TAGame.SpecialPickup_TA", "\"TAGame.CarComponent_TA\"")
        .entry("TAGame.SpecialPickup_Targeted_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.SpecialPickup_Tornado_TA", "\"TAGame.SpecialPickup_TA\"")
        .entry("TAGame.Team_Soccar_TA", "\"TAGame.Team_TA\"")
        .entry("TAGame.Team_TA", "\"Engine.TeamInfo\"")
        .entry("TAGame.Vehicle_TA", "\"TAGame.RBActor_TA\"")
        .entry("TAGame.VehiclePickup_TA", "\"Engine.ReplicationInfo\"")
        .entry("TAGame.VehiclePickup_Boost_TA", "\"TAGame.VehiclePickup_TA\"")
        .build(&mut file)
        .unwrap();

    write!(&mut file, ";\n").unwrap();
}
