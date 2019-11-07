use crate::network::{AttributeTag, SpawnTrajectory};

pub (crate) static SPAWN_STATS: phf::Map<&'static str, SpawnTrajectory> = phf::phf_map! {
    "TAGame.Ball_Breakout_TA" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Breakout" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Trajectory" => SpawnTrajectory::LocationAndRotation,
    "TAGame.Ball_TA" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_BasketBall_Mutator" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_BasketBall" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Basketball" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Default" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Puck" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Anniversary" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.CubeBall" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Haunted" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Ball.Ball_Training" => SpawnTrajectory::LocationAndRotation,
    "TAGame.Ball_Haunted_TA" => SpawnTrajectory::LocationAndRotation,
    "TAGame.Car_Season_TA" => SpawnTrajectory::LocationAndRotation,
    "TAGame.Car_TA" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.Car.Car_Default" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.GameEvent.GameEvent_Season:CarArchetype" => SpawnTrajectory::LocationAndRotation,
    "Archetypes.SpecialPickups.SpecialPickup_HauntedBallBeam" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_Rugby" => SpawnTrajectory::Location,
    "TAGame.CameraSettingsActor_TA" => SpawnTrajectory::Location,
    "TAGame.CarComponent_Boost_TA" => SpawnTrajectory::Location,
    "TAGame.CarComponent_Dodge_TA" => SpawnTrajectory::Location,
    "TAGame.CarComponent_DoubleJump_TA" => SpawnTrajectory::Location,
    "TAGame.CarComponent_FlipCar_TA" => SpawnTrajectory::Location,
    "TAGame.CarComponent_Jump_TA" => SpawnTrajectory::Location,
    "TAGame.GameEvent_Season_TA" => SpawnTrajectory::Location,
    "TAGame.GameEvent_Soccar_TA" => SpawnTrajectory::Location,
    "TAGame.GameEvent_SoccarPrivate_TA" => SpawnTrajectory::Location,
    "TAGame.GameEvent_SoccarSplitscreen_TA" => SpawnTrajectory::Location,
    "TAGame.GRI_TA" => SpawnTrajectory::Location,
    "TAGame.PRI_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BallCarSpring_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BallFreeze_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BallGravity_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BallLasso_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BallVelcro_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_Batarang_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_BoostOverride_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_GrapplingHook_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_HitForce_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_Swapper_TA" => SpawnTrajectory::Location,
    "TAGame.SpecialPickup_Tornado_TA" => SpawnTrajectory::Location,
    "TAGame.Team_Soccar_TA" => SpawnTrajectory::Location,
    "Archetypes.CarComponents.CarComponent_Boost" => SpawnTrajectory::Location,
    "Archetypes.CarComponents.CarComponent_Dodge" => SpawnTrajectory::Location,
    "Archetypes.CarComponents.CarComponent_DoubleJump" => SpawnTrajectory::Location,
    "Archetypes.CarComponents.CarComponent_FlipCar" => SpawnTrajectory::Location,
    "Archetypes.CarComponents.CarComponent_Jump" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Basketball" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_BasketballPrivate" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_BasketballSplitscreen" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Breakout" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Hockey" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_HockeyPrivate" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_HockeySplitscreen" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Items" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Season" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_Soccar" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_SoccarLan" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_SoccarPrivate" => SpawnTrajectory::Location,
    "Archetypes.GameEvent.GameEvent_SoccarSplitscreen" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BallFreeze" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BallLasso" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BallSpring" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BallVelcro" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_Batarang" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_BoostOverride" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_CarSpring" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_GravityWell" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_StrongHit" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_Swapper" => SpawnTrajectory::Location,
    "Archetypes.SpecialPickups.SpecialPickup_Tornado" => SpawnTrajectory::Location,
    "Archetypes.Teams.Team0" => SpawnTrajectory::Location,
    "Archetypes.Teams.Team1" => SpawnTrajectory::Location,
    "GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "GameInfo_Tutorial.GameInfo.GameInfo_Tutorial:GameReplicationInfoArchetype" => SpawnTrajectory::Location,
    "TAGame.Default__CameraSettingsActor_TA" => SpawnTrajectory::Location,
    "TAGame.Default__PRI_TA" => SpawnTrajectory::Location,
    "TheWorld:PersistentLevel.BreakOutActor_Platform_TA" => SpawnTrajectory::Location,
    "TheWorld:PersistentLevel.CrowdActor_TA" => SpawnTrajectory::Location,
    "TheWorld:PersistentLevel.CrowdManager_TA" => SpawnTrajectory::Location,
    "TheWorld:PersistentLevel.InMapScoreboard_TA" => SpawnTrajectory::Location,
    "TheWorld:PersistentLevel.VehiclePickup_Boost_TA" => SpawnTrajectory::Location,
    "TAGame.HauntedBallTrapTrigger_TA" => SpawnTrajectory::Location,
    "ProjectX.Default__NetModeReplicator_X" => SpawnTrajectory::Location,
    "GameInfo_Tutorial.GameEvent.GameEvent_Tutorial_Aerial" => SpawnTrajectory::Location,
    "Archetypes.Tutorial.Cannon" => SpawnTrajectory::Location,
};

pub (crate) static ATTRIBUTES: phf::Map<&'static str, AttributeTag> = phf::phf_map! {
    "Engine.Actor:bBlockActors" => AttributeTag::Boolean,
    "Engine.Actor:bCollideActors" => AttributeTag::Boolean,
    "Engine.Actor:bHidden" => AttributeTag::Boolean,
    "Engine.Actor:bTearOff" => AttributeTag::Boolean,
    "Engine.Actor:DrawScale" => AttributeTag::Float,
    "Engine.Actor:RemoteRole" => AttributeTag::Enum,
    "Engine.Actor:Role" => AttributeTag::Enum,
    "Engine.Actor:Rotation" => AttributeTag::RotationTag,
    "Engine.GameReplicationInfo:bMatchIsOver" => AttributeTag::Boolean,
    "Engine.GameReplicationInfo:GameClass" => AttributeTag::Flagged,
    "Engine.GameReplicationInfo:ServerName" => AttributeTag::String,
    "Engine.Pawn:PlayerReplicationInfo" => AttributeTag::Flagged,
    "Engine.Pawn:HealthMax" => AttributeTag::Int,
    "Engine.PlayerReplicationInfo:bBot" => AttributeTag::Boolean,
    "Engine.PlayerReplicationInfo:bIsSpectator" => AttributeTag::Boolean,
    "Engine.PlayerReplicationInfo:bReadyToPlay" => AttributeTag::Boolean,
    "Engine.PlayerReplicationInfo:bTimedOut" => AttributeTag::Boolean,
    "Engine.PlayerReplicationInfo:bWaitingPlayer" => AttributeTag::Boolean,
    "Engine.PlayerReplicationInfo:Ping" => AttributeTag::Byte,
    "Engine.PlayerReplicationInfo:PlayerID" => AttributeTag::Int,
    "Engine.PlayerReplicationInfo:PlayerName" => AttributeTag::String,
    "Engine.PlayerReplicationInfo:RemoteUserData" => AttributeTag::String,
    "Engine.PlayerReplicationInfo:Score" => AttributeTag::Int,
    "Engine.PlayerReplicationInfo:Team" => AttributeTag::Flagged,
    "Engine.PlayerReplicationInfo:UniqueId" => AttributeTag::UniqueId,
    "Engine.TeamInfo:Score" => AttributeTag::Int,
    "ProjectX.GRI_X:bGameStarted" => AttributeTag::Boolean,
    "ProjectX.GRI_X:GameServerID" => AttributeTag::QWord,
    "ProjectX.GRI_X:MatchGUID" => AttributeTag::String,
    "ProjectX.GRI_X:ReplicatedGameMutatorIndex" => AttributeTag::Int,
    "ProjectX.GRI_X:ReplicatedGamePlaylist" => AttributeTag::Int,
    "ProjectX.GRI_X:Reservations" => AttributeTag::Reservation,
    "TAGame.Ball_Breakout_TA:AppliedDamage" => AttributeTag::AppliedDamage,
    "TAGame.Ball_Breakout_TA:DamageIndex" => AttributeTag::Int,
    "TAGame.Ball_Breakout_TA:LastTeamTouch" => AttributeTag::Byte,
    "TAGame.Ball_TA:GameEvent" => AttributeTag::Flagged,
    "TAGame.Ball_TA:HitTeamNum" => AttributeTag::Byte,
    "TAGame.Ball_TA:ReplicatedAddedCarBounceScale" => AttributeTag::Float,
    "TAGame.Ball_TA:ReplicatedBallMaxLinearSpeedScale" => AttributeTag::Float,
    "TAGame.Ball_TA:ReplicatedBallScale" => AttributeTag::Float,
    "TAGame.Ball_TA:ReplicatedExplosionData" => AttributeTag::Explosion,
    "TAGame.Ball_TA:ReplicatedExplosionDataExtended" => AttributeTag::ExtendedExplosion,
    "TAGame.Ball_TA:ReplicatedWorldBounceScale" => AttributeTag::Float,
    "TAGame.BreakOutActor_Platform_TA:DamageState" => AttributeTag::DamageState,
    "TAGame.CameraSettingsActor_TA:bUsingBehindView" => AttributeTag::Boolean,
    "TAGame.CameraSettingsActor_TA:bMouseCameraToggleEnabled" => AttributeTag::Boolean,
    "TAGame.CameraSettingsActor_TA:bUsingSecondaryCamera" => AttributeTag::Boolean,
    "TAGame.CameraSettingsActor_TA:bUsingSwivel" => AttributeTag::Boolean,
    "TAGame.CameraSettingsActor_TA:CameraPitch" => AttributeTag::Byte,
    "TAGame.CameraSettingsActor_TA:CameraYaw" => AttributeTag::Byte,
    "TAGame.CameraSettingsActor_TA:PRI" => AttributeTag::Flagged,
    "TAGame.CameraSettingsActor_TA:ProfileSettings" => AttributeTag::CamSettings,
    "TAGame.Car_TA:AddedBallForceMultiplier" => AttributeTag::Float,
    "TAGame.Car_TA:AddedCarForceMultiplier" => AttributeTag::Float,
    "TAGame.Car_TA:AttachedPickup" => AttributeTag::Flagged,
    "TAGame.Car_TA:ClubColors" => AttributeTag::ClubColors,
    "TAGame.Car_TA:ReplicatedCarScale" => AttributeTag::Float,
    "TAGame.Car_TA:ReplicatedDemolish" => AttributeTag::Demolish,
    "TAGame.Car_TA:TeamPaint" => AttributeTag::TeamPaint,
    "TAGame.CarComponent_Boost_TA:bNoBoost" => AttributeTag::Boolean,
    "TAGame.CarComponent_Boost_TA:BoostModifier" => AttributeTag::Float,
    "TAGame.CarComponent_Boost_TA:bUnlimitedBoost" => AttributeTag::Boolean,
    "TAGame.CarComponent_Boost_TA:RechargeDelay" => AttributeTag::Float,
    "TAGame.CarComponent_Boost_TA:RechargeRate" => AttributeTag::Float,
    "TAGame.CarComponent_Boost_TA:ReplicatedBoostAmount" => AttributeTag::Byte,
    "TAGame.CarComponent_Boost_TA:UnlimitedBoostRefCount" => AttributeTag::Int,
    "TAGame.CarComponent_Dodge_TA:DodgeTorque" => AttributeTag::Location,
    "TAGame.CarComponent_FlipCar_TA:bFlipRight" => AttributeTag::Boolean,
    "TAGame.CarComponent_FlipCar_TA:FlipCarTime" => AttributeTag::Float,
    "TAGame.CarComponent_TA:ReplicatedActive" => AttributeTag::Byte,
    "TAGame.CarComponent_TA:ReplicatedActivityTime" => AttributeTag::Float,
    "TAGame.CarComponent_TA:Vehicle" => AttributeTag::Flagged,
    "TAGame.CrowdActor_TA:GameEvent" => AttributeTag::Flagged,
    "TAGame.CrowdActor_TA:ModifiedNoise" => AttributeTag::Float,
    "TAGame.CrowdActor_TA:ReplicatedCountDownNumber" => AttributeTag::Int,
    "TAGame.CrowdActor_TA:ReplicatedOneShotSound" => AttributeTag::Flagged,
    "TAGame.CrowdActor_TA:ReplicatedRoundCountDownNumber" => AttributeTag::Int,
    "TAGame.CrowdManager_TA:GameEvent" => AttributeTag::Flagged,
    "TAGame.CrowdManager_TA:ReplicatedGlobalOneShotSound" => AttributeTag::Flagged,
    "TAGame.GameEvent_Soccar_TA:bBallHasBeenHit" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:bClubMatch" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:bOverTime" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:bMatchEnded" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:bNoContest" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:bUnlimitedTime" => AttributeTag::Boolean,
    "TAGame.GameEvent_Soccar_TA:GameTime" => AttributeTag::Int,
    "TAGame.GameEvent_Soccar_TA:GameWinner" => AttributeTag::Flagged,
    "TAGame.GameEvent_Soccar_TA:MatchWinner" => AttributeTag::Flagged,
    "TAGame.GameEvent_Soccar_TA:MVP" => AttributeTag::Flagged,
    "TAGame.GameEvent_Soccar_TA:ReplicatedMusicStinger" => AttributeTag::MusicStinger,
    "TAGame.GameEvent_Soccar_TA:ReplicatedScoredOnTeam" => AttributeTag::Byte,
    "TAGame.GameEvent_Soccar_TA:ReplicatedServerPerformanceState" => AttributeTag::Byte,
    "TAGame.GameEvent_Soccar_TA:ReplicatedStatEvent" => AttributeTag::StatEvent,
    "TAGame.GameEvent_Soccar_TA:RoundNum" => AttributeTag::Int,
    "TAGame.GameEvent_Soccar_TA:SecondsRemaining" => AttributeTag::Int,
    "TAGame.GameEvent_Soccar_TA:SeriesLength" => AttributeTag::Int,
    "TAGame.GameEvent_Soccar_TA:SubRulesArchetype" => AttributeTag::Flagged,
    "TAGame.GameEvent_SoccarPrivate_TA:MatchSettings" => AttributeTag::PrivateMatchSettings,
    "TAGame.GameEvent_TA:bAllowReadyUp" => AttributeTag::Boolean,
    "TAGame.GameEvent_TA:bCanVoteToForfeit" => AttributeTag::Boolean,
    "TAGame.GameEvent_TA:bHasLeaveMatchPenalty" => AttributeTag::Boolean,
    "TAGame.GameEvent_TA:BotSkill" => AttributeTag::Int,
    "TAGame.GameEvent_TA:GameMode" => AttributeTag::GameMode,
    "TAGame.GameEvent_TA:MatchTypeClass" => AttributeTag::Flagged,
    "TAGame.GameEvent_TA:ReplicatedGameStateTimeRemaining" => AttributeTag::Int,
    "TAGame.GameEvent_TA:ReplicatedRoundCountDownNumber" => AttributeTag::Int,
    "TAGame.GameEvent_TA:ReplicatedStateIndex" => AttributeTag::Byte,
    "TAGame.GameEvent_TA:ReplicatedStateName" => AttributeTag::Int,
    "TAGame.GameEvent_Team_TA:bForfeit" => AttributeTag::Boolean,
    "TAGame.GameEvent_Team_TA:MaxTeamSize" => AttributeTag::Int,
    "TAGame.GRI_TA:NewDedicatedServerIP" => AttributeTag::String,
    "TAGame.PRI_TA:bIsInSplitScreen" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bMatchMVP" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bOnlineLoadoutSet" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bOnlineLoadoutsSet" => AttributeTag::Boolean,
    "TAGame.PRI_TA:BotProductName" => AttributeTag::Int,
    "TAGame.PRI_TA:bReady" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bUsingBehindView" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bUsingItems" => AttributeTag::Boolean,
    "TAGame.PRI_TA:bUsingSecondaryCamera" => AttributeTag::Boolean,
    "TAGame.PRI_TA:CameraPitch" => AttributeTag::Byte,
    "TAGame.PRI_TA:CameraSettings" => AttributeTag::CamSettings,
    "TAGame.PRI_TA:CameraYaw" => AttributeTag::Byte,
    "TAGame.PRI_TA:ClientLoadout" => AttributeTag::Loadout,
    "TAGame.PRI_TA:ClientLoadoutOnline" => AttributeTag::LoadoutOnline,
    "TAGame.PRI_TA:ClientLoadouts" => AttributeTag::TeamLoadout,
    "TAGame.PRI_TA:ClientLoadoutsOnline" => AttributeTag::LoadoutsOnline,
    "TAGame.PRI_TA:ClubID" => AttributeTag::Int64,
    "TAGame.PRI_TA:MatchAssists" => AttributeTag::Int,
    "TAGame.PRI_TA:MatchBreakoutDamage" => AttributeTag::Int,
    "TAGame.PRI_TA:MatchGoals" => AttributeTag::Int,
    "TAGame.PRI_TA:MatchSaves" => AttributeTag::Int,
    "TAGame.PRI_TA:MatchScore" => AttributeTag::Int,
    "TAGame.PRI_TA:MatchShots" => AttributeTag::Int,
    "TAGame.PRI_TA:MaxTimeTillItem" => AttributeTag::Int,
    "TAGame.PRI_TA:PartyLeader" => AttributeTag::PartyLeader,
    "TAGame.PRI_TA:PawnType" => AttributeTag::Byte,
    "TAGame.PRI_TA:PersistentCamera" => AttributeTag::Flagged,
    "TAGame.PRI_TA:PlayerHistoryKey" => AttributeTag::PlayerHistoryKey,
    "TAGame.PRI_TA:PlayerHistoryValid" => AttributeTag::Boolean,
    "TAGame.PRI_TA:ReplicatedGameEvent" => AttributeTag::Flagged,
    "TAGame.PRI_TA:ReplicatedWorstNetQualityBeyondLatency" => AttributeTag::Byte,
    "TAGame.PRI_TA:RepStatTitles" => AttributeTag::RepStatTitle,
    "TAGame.PRI_TA:SteeringSensitivity" => AttributeTag::Float,
    "TAGame.PRI_TA:SkillTier" => AttributeTag::FlaggedByte,
    "TAGame.PRI_TA:TimeTillItem" => AttributeTag::Int,
    "TAGame.PRI_TA:Title" => AttributeTag::Int,
    "TAGame.PRI_TA:TotalXP" => AttributeTag::Int,
    "TAGame.PRI_TA:PrimaryTitle" => AttributeTag::Title,
    "TAGame.PRI_TA:SecondaryTitle" => AttributeTag::Title,
    "TAGame.PRI_TA:SpectatorShortcut" => AttributeTag::Int,
    "TAGame.RBActor_TA:bFrozen" => AttributeTag::Boolean,
    "TAGame.RBActor_TA:bIgnoreSyncing" => AttributeTag::Boolean,
    "TAGame.RBActor_TA:bReplayActor" => AttributeTag::Boolean,
    "TAGame.RBActor_TA:ReplicatedRBState" => AttributeTag::RigidBody,
    "TAGame.RBActor_TA:WeldedInfo" => AttributeTag::Welded,
    "TAGame.SpecialPickup_BallFreeze_TA:RepOrigSpeed" => AttributeTag::Float,
    "TAGame.SpecialPickup_BallVelcro_TA:AttachTime" => AttributeTag::Float,
    "TAGame.SpecialPickup_BallVelcro_TA:bBroken" => AttributeTag::Boolean,
    "TAGame.SpecialPickup_BallVelcro_TA:bHit" => AttributeTag::Boolean,
    "TAGame.SpecialPickup_BallVelcro_TA:BreakTime" => AttributeTag::Float,
    "TAGame.SpecialPickup_Targeted_TA:Targeted" => AttributeTag::Flagged,
    "TAGame.Team_Soccar_TA:GameScore" => AttributeTag::Int,
    "TAGame.Team_TA:ClubColors" => AttributeTag::ClubColors,
    "TAGame.Team_TA:ClubID" => AttributeTag::Int64,
    "TAGame.Team_TA:CustomTeamName" => AttributeTag::String,
    "TAGame.Team_TA:GameEvent" => AttributeTag::Flagged,
    "TAGame.Team_TA:LogoData" => AttributeTag::Flagged,
    "TAGame.Vehicle_TA:bDriving" => AttributeTag::Boolean,
    "TAGame.Vehicle_TA:bPodiumMode" => AttributeTag::Boolean,
    "TAGame.Vehicle_TA:bReplicatedHandbrake" => AttributeTag::Boolean,
    "TAGame.Vehicle_TA:ReplicatedSteer" => AttributeTag::Byte,
    "TAGame.Vehicle_TA:ReplicatedThrottle" => AttributeTag::Byte,
    "TAGame.VehiclePickup_TA:bNoPickup" => AttributeTag::Boolean,
    "TAGame.VehiclePickup_TA:ReplicatedPickupData" => AttributeTag::Pickup,
    "TAGame.VehiclePickup_TA:NewReplicatedPickupData" => AttributeTag::PickupNew,
    "TAGame.Ball_Haunted_TA:LastTeamTouch" => AttributeTag::Byte,
    "TAGame.Ball_Haunted_TA:TotalActiveBeams" => AttributeTag::Byte,
    "TAGame.Ball_Haunted_TA:DeactivatedGoalIndex" => AttributeTag::Byte,
    "TAGame.Ball_Haunted_TA:ReplicatedBeamBrokenValue" => AttributeTag::Byte,
    "TAGame.Ball_Haunted_TA:bIsBallBeamed" => AttributeTag::Boolean,
    "TAGame.SpecialPickup_Rugby_TA:bBallWelded" => AttributeTag::Boolean,
    "TAGame.Cannon_TA:Pitch" => AttributeTag::Float,
    "TAGame.Cannon_TA:FireCount" => AttributeTag::Byte,
};

pub (crate) fn object_classes() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Archetypes.Ball.Ball_BasketBall_Mutator", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Basketball", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_BasketBall", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Beachball", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Breakout", "TAGame.Ball_Breakout_TA"),
        ("Archetypes.Ball.Ball_Default", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Trajectory", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Haunted", "TAGame.Ball_Haunted_TA"),
        ("Archetypes.Ball.Ball_Puck", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Anniversary", "TAGame.Ball_TA"),
        ("Archetypes.Ball.CubeBall", "TAGame.Ball_TA"),
        ("Archetypes.Ball.Ball_Training", "TAGame.Ball_TA"),
        ("Archetypes.Car.Car_Default", "TAGame.Car_TA"),
        ("Archetypes.CarComponents.CarComponent_Boost", "TAGame.CarComponent_Boost_TA"),
        ("Archetypes.CarComponents.CarComponent_Dodge", "TAGame.CarComponent_Dodge_TA"),
        ("Archetypes.CarComponents.CarComponent_DoubleJump", "TAGame.CarComponent_DoubleJump_TA"),
        ("Archetypes.CarComponents.CarComponent_FlipCar", "TAGame.CarComponent_FlipCar_TA"),
        ("Archetypes.CarComponents.CarComponent_Jump", "TAGame.CarComponent_Jump_TA"),
        ("Archetypes.GameEvent.GameEvent_Basketball", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_BasketballPrivate", "TAGame.GameEvent_SoccarPrivate_TA"),
        ("Archetypes.GameEvent.GameEvent_BasketballSplitscreen", "TAGame.GameEvent_SoccarSplitscreen_TA"),
        ("Archetypes.GameEvent.GameEvent_Breakout", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_Hockey", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_HockeyPrivate", "TAGame.GameEvent_SoccarPrivate_TA"),
        ("Archetypes.GameEvent.GameEvent_HockeySplitscreen", "TAGame.GameEvent_SoccarSplitscreen_TA"),
        ("Archetypes.GameEvent.GameEvent_Items", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_Season:CarArchetype", "TAGame.Car_TA"),
        ("Archetypes.GameEvent.GameEvent_Season", "TAGame.GameEvent_Season_TA"),
        ("Archetypes.GameEvent.GameEvent_Soccar", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_SoccarLan", "TAGame.GameEvent_Soccar_TA"),
        ("Archetypes.GameEvent.GameEvent_SoccarPrivate", "TAGame.GameEvent_SoccarPrivate_TA"),
        ("Archetypes.GameEvent.GameEvent_SoccarSplitscreen", "TAGame.GameEvent_SoccarSplitscreen_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BallFreeze", "TAGame.SpecialPickup_BallFreeze_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BallGrapplingHook", "TAGame.SpecialPickup_GrapplingHook_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BallLasso", "TAGame.SpecialPickup_BallLasso_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BallSpring", "TAGame.SpecialPickup_BallCarSpring_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BallVelcro", "TAGame.SpecialPickup_BallVelcro_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_Batarang", "TAGame.SpecialPickup_Batarang_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_BoostOverride", "TAGame.SpecialPickup_BoostOverride_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_CarSpring", "TAGame.SpecialPickup_BallCarSpring_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_GravityWell", "TAGame.SpecialPickup_BallGravity_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_StrongHit", "TAGame.SpecialPickup_HitForce_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_Swapper", "TAGame.SpecialPickup_Swapper_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_Tornado", "TAGame.SpecialPickup_Tornado_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_HauntedBallBeam", "TAGame.SpecialPickup_HauntedBallBeam_TA"),
        ("Archetypes.SpecialPickups.SpecialPickup_Rugby", "TAGame.SpecialPickup_Rugby_TA"),
        ("Archetypes.Teams.Team0", "TAGame.Team_Soccar_TA"),
        ("Archetypes.Teams.Team1", "TAGame.Team_Soccar_TA"),
        ("GameInfo_Basketball.GameInfo.GameInfo_Basketball:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Breakout.GameInfo.GameInfo_Breakout:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("Gameinfo_Hockey.GameInfo.Gameinfo_Hockey:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Items.GameInfo.GameInfo_Items:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Season.GameInfo.GameInfo_Season:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Soccar.GameInfo.GameInfo_Soccar:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Tutorial.GameInfo.GameInfo_Tutorial:GameReplicationInfoArchetype", "TAGame.GRI_TA"),
        ("GameInfo_Tutorial.GameEvent.GameEvent_Tutorial_Aerial", "TAGame.GameEvent_Tutorial_TA"),
        ("TAGame.Default__CameraSettingsActor_TA", "TAGame.CameraSettingsActor_TA"),
        ("TAGame.Default__PRI_TA", "TAGame.PRI_TA"),
        ("TAGame.Default__Car_TA", "TAGame.Car_TA"),
        ("TheWorld:PersistentLevel.BreakOutActor_Platform_TA", "TAGame.BreakOutActor_Platform_TA"),
        ("TheWorld:PersistentLevel.CrowdActor_TA", "TAGame.CrowdActor_TA"),
        ("TheWorld:PersistentLevel.CrowdManager_TA", "TAGame.CrowdManager_TA"),
        ("TheWorld:PersistentLevel.InMapScoreboard_TA", "TAGame.InMapScoreboard_TA"),
        ("TheWorld:PersistentLevel.VehiclePickup_Boost_TA", "TAGame.VehiclePickup_Boost_TA"),
        ("Haunted_TrainStation_P.TheWorld:PersistentLevel.HauntedBallTrapTrigger_TA_1", "TAGame.HauntedBallTrapTrigger_TA"),
        ("Haunted_TrainStation_P.TheWorld:PersistentLevel.HauntedBallTrapTrigger_TA_0", "TAGame.HauntedBallTrapTrigger_TA"),
        ("Archetypes.Tutorial.Cannon", "TAGame.Cannon_TA")
    ]
}

pub (crate) static PARENT_CLASSES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Engine.Actor" => "Core.Object",
    "Engine.GameReplicationInfo" => "Engine.ReplicationInfo",
    "Engine.Info" => "Engine.Actor",
    "Engine.Pawn" => "Engine.Actor",
    "Engine.PlayerReplicationInfo" => "Engine.ReplicationInfo",
    "Engine.ReplicationInfo" => "Engine.Info",
    "Engine.TeamInfo" => "Engine.ReplicationInfo",
    "ProjectX.GRI_X" => "Engine.GameReplicationInfo",
    "ProjectX.Pawn_X" => "Engine.Pawn",
    "ProjectX.PRI_X" => "Engine.PlayerReplicationInfo",
    "TAGame.Ball_TA" => "TAGame.RBActor_TA",
    "TAGame.CameraSettingsActor_TA" => "Engine.ReplicationInfo",
    "TAGame.Car_Season_TA" => "TAGame.PRI_TA",
    "TAGame.Car_TA" => "TAGame.Vehicle_TA",
    "TAGame.CarComponent_Boost_TA" => "TAGame.CarComponent_TA",
    "TAGame.CarComponent_Dodge_TA" => "TAGame.CarComponent_TA",
    "TAGame.CarComponent_DoubleJump_TA" => "TAGame.CarComponent_TA",
    "TAGame.CarComponent_FlipCar_TA" => "TAGame.CarComponent_TA",
    "TAGame.CarComponent_Jump_TA" => "TAGame.CarComponent_TA",
    "TAGame.CarComponent_TA" => "Engine.ReplicationInfo",
    "TAGame.CrowdActor_TA" => "Engine.ReplicationInfo",
    "TAGame.CrowdManager_TA" => "Engine.ReplicationInfo",
    "TAGame.GameEvent_Season_TA" => "TAGame.GameEvent_Soccar_TA",
    "TAGame.GameEvent_Soccar_TA" => "TAGame.GameEvent_Team_TA",
    "TAGame.GameEvent_SoccarPrivate_TA" => "TAGame.GameEvent_Soccar_TA",
    "TAGame.GameEvent_SoccarSplitscreen_TA" => "TAGame.GameEvent_SoccarPrivate_TA",
    "TAGame.GameEvent_Tutorial_TA" => "TAGame.GameEvent_Soccar_TA",
    "TAGame.GameEvent_TA" => "Engine.ReplicationInfo",
    "TAGame.GameEvent_Team_TA" => "TAGame.GameEvent_TA",
    "TAGame.GRI_TA" => "ProjectX.GRI_X",
    "TAGame.InMapScoreboard_TA" => "Engine.Actor",
    "TAGame.PRI_TA" => "ProjectX.PRI_X",
    "TAGame.RBActor_TA" => "ProjectX.Pawn_X",
    "TAGame.SpecialPickup_BallCarSpring_TA" => "TAGame.SpecialPickup_Spring_TA",
    "TAGame.SpecialPickup_BallFreeze_TA" => "TAGame.SpecialPickup_Targeted_TA",
    "TAGame.SpecialPickup_BallGravity_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.SpecialPickup_BallLasso_TA" => "TAGame.SpecialPickup_GrapplingHook_TA",
    "TAGame.SpecialPickup_BallVelcro_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.SpecialPickup_Batarang_TA" => "TAGame.SpecialPickup_BallLasso_TA",
    "TAGame.SpecialPickup_BoostOverride_TA" => "TAGame.SpecialPickup_Targeted_TA",
    "TAGame.SpecialPickup_GrapplingHook_TA" => "TAGame.SpecialPickup_Targeted_TA",
    "TAGame.SpecialPickup_HitForce_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.SpecialPickup_Spring_TA" => "TAGame.SpecialPickup_Targeted_TA",
    "TAGame.SpecialPickup_Swapper_TA" => "TAGame.SpecialPickup_Targeted_TA",
    "TAGame.SpecialPickup_TA" => "TAGame.CarComponent_TA",
    "TAGame.SpecialPickup_Targeted_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.SpecialPickup_Tornado_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.SpecialPickup_Rugby_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.Team_Soccar_TA" => "TAGame.Team_TA",
    "TAGame.Team_TA" => "Engine.TeamInfo",
    "TAGame.Vehicle_TA" => "TAGame.RBActor_TA",
    "TAGame.VehiclePickup_TA" => "Engine.ReplicationInfo",
    "TAGame.VehiclePickup_Boost_TA" => "TAGame.VehiclePickup_TA",
    "TAGame.SpecialPickup_HauntedBallBeam_TA" => "TAGame.SpecialPickup_TA",
    "TAGame.Cannon_TA" => "Engine.Actor",
};
