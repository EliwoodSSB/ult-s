use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::frame;
use smash::phx::{Hash40, Vector2f};
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut IS_MEGA : [bool; 8] = [false; 8];
static mut DOWNB_X : [f32; 8] = [0.0; 8];
static mut DOWNB_Y : [f32; 8] = [0.0; 8];

#[acmd_script(
    agent = "lucario",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.5)
		frame(Frame=4)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=30, FKB=0, BKB=20, Size=3.8, X=0.0, Y=-2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=367, KBG=100, FKB=50, BKB=0, Size=4.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=150, KBG=100, FKB=25, BKB=0, Size=4.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			rust {
				let speed = smash::phx::Vector3f { x: -0.85, y: 0.0, z: 0.0 };
				KineticModule::mul_speed(fighter.module_accessor, &speed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let speed = smash::phx::Vector3f { x: 0.0, y: 0.6, z: 0.0 };
				KineticModule::add_speed(fighter.module_accessor, &speed);
			}
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=5.8, X=0.0, Y=-2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=4.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "lucario",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=64, KBG=85, FKB=0, BKB=55, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("toel"), Damage=6.0, Angle=64, KBG=85, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=64, KBG=85, FKB=0, BKB=55, Size=5.0, X=0.0, Y=6.7, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=28)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "lucario",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=0.0, Y=9.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=0.0, Y=11.0, Z=-5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("shoulderr"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=1.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("shoulderl"), Damage=3.0, Angle=367, KBG=100, FKB=80, BKB=0, Size=3.6, X=1.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=0.0, Y=9.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=0.0, Y=11.0, Z=-5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("shoulderr"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=1.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("shoulderl"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.6, X=1.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=22)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "lucario",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.2)
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=33, KBG=99, FKB=0, BKB=30, Size=4.3, X=0.0, Y=6.0, Z=13.0, X2=0.0, Y2=6.0, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=33, KBG=99, FKB=0, BKB=30, Size=4.3, X=0.0, Y=6.0, Z=-15.0, X2=0.0, Y2=6.0, Z2=-10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "lucario",
    script =  "sound_attacklw4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn lucario_dsmashs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
		}
		frame(Frame=12)
		if(is_excute){
			PLAY_SE(hash40("vc_lucario_attack05"))
		}
		frame(Frame=18)
		if(is_excute){
			PLAY_SE(hash40("se_lucario_smash_l01"))
			PLAY_SE(hash40("se_lucario_smash_add04"))
		}
    });
}	
#[acmd_script(
    agent = "lucario",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.66)
		frame(Frame=9)
		FT_MOTION_RATE(FSM=0.8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=50, FKB=0, BKB=50, Size=4.0, X=0.0, Y=2.8, Z=9.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "lucario",
    scripts =  ["game_attacks3", "game_attacks3lw", "game_attacks3hi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=100, FKB=25, BKB=0, Size=4.0, X=0.0, Y=6.0, Z=8.5, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=366, KBG=100, FKB=30, BKB=0, Size=4.0, X=0.0, Y=6.0, Z=8.5, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=100, FKB=10, BKB=0, Size=4.0, X=0.0, Y=6.0, Z=8.5, X2=0.0, Y2=6.0, Z2=3.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=98, FKB=0, BKB=54, Size=3.0, X=0.0, Y=6.0, Z=14.0, X2=0.0, Y2=6.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=100, FKB=10, BKB=0, Size=3.0, X=0.0, Y=6.0, Z=14.0, X2=0.0, Y2=6.0, Z2=5.0, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_PUNCH)
			AttackModule::clear(ID=1, false)
			AttackModule::clear(ID=2, false)
		}
		frame(Frame=17)
		if(is_excute){
		AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "lucario",
    scripts =  ["game_speciallw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=22)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=70, KBG=100, FKB=113, BKB=0, Size=7.0, X=0.0, Y=3.0, Z=12.0, X2=0.0, Y2=15.0, Z2=12.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=70, KBG=100, FKB=80, BKB=0, Size=7.0, X=0.0, Y=15.0, Z=12.0, X2=0.0, Y2=27.0, Z2=12.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=27)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "lucario",
    scripts =  ["sound_speciallw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn lucario_sound_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_lucario_special_l01"))
		}
		frame(Frame=22)
		if(is_excute){
			PLAY_SE(hash40("vc_lucario_attack05"))
		}
		wait(Frames=1)
		if(is_excute){
			PLAY_SE(hash40("se_lucario_smash_h01"))
			PLAY_SE(hash40("se_lucario_smash_add02"))
		}
    });
}		
#[acmd_script(
    agent = "lucario",
    scripts =  ["effect_speciallw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn lucario_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(0x138dd24a0cu64, hash40("top"), 0, 0, 0, 0, 0, 0, 1, true)
			EFFECT(0x1925c7cacdu64, hash40("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			LANDING_EFFECT(hash40("sys_whirlwind_l"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=22)
		if(is_excute){
			EFFECT(hash40("lucario_hakkei_far"), hash40("top"), -1, 0, 12, -90, 0, 0, 0.775, 0, 0, 0, 0, 0, 0, true)
		}
    });
}			
//lucario
#[fighter_frame_callback]
pub fn lucario(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_LUCARIO {
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					if frame > 3.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_SPLIT, true);
					};
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && status_kind != *FIGHTER_STATUS_KIND_CAPTURE_JUMP {
				HAS_DOWNB[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_SPLIT {
				if frame < 3.0 {
					if ControlModule::get_stick_x(boma) > 0.5 {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 20.0, y: 0.0}, false) == 0 {
							DOWNB_X[ENTRY_ID] = PostureModule::pos_x(boma) + 20.0;
						} else {
							DOWNB_X[ENTRY_ID] = PostureModule::pos_x(boma);
						};
					} else if  ControlModule::get_stick_x(boma) < -0.5 {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: -20.0, y: 0.0}, false) == 0 {
							DOWNB_X[ENTRY_ID] = PostureModule::pos_x(boma) - 20.0;
						} else {
							DOWNB_X[ENTRY_ID] = PostureModule::pos_x(boma);
						};
					} else {
						DOWNB_X[ENTRY_ID] = PostureModule::pos_x(boma);
					};
					if ControlModule::get_stick_y(boma) > 0.5 {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: 20.0}, false) == 0 {
							DOWNB_Y[ENTRY_ID] = PostureModule::pos_y(boma);
						} else {
							DOWNB_Y[ENTRY_ID] = PostureModule::pos_y(boma);
						};
					} else if  ControlModule::get_stick_y(boma) < -0.5 {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -20.0}, false) == 0 {
							DOWNB_Y[ENTRY_ID] = PostureModule::pos_y(boma);
						} else {
							DOWNB_Y[ENTRY_ID] = PostureModule::pos_y(boma);
						};
					} else {
						DOWNB_Y[ENTRY_ID] = PostureModule::pos_y(boma);
					};
				};
				if frame < 6.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				};
				MotionModule::set_rate(boma, 0.55);
			};
			if status_kind == *FIGHTER_STATUS_KIND_CAPTURE_JUMP && HAS_DOWNB[ENTRY_ID] {
				MotionModule::set_rate(boma, 0.8);
			};
			if status_kind == *FIGHTER_STATUS_KIND_CAPTURE_PULLED && HAS_DOWNB[ENTRY_ID] {
				HAS_DOWNB[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR {
				let pos = smash::phx::Vector3f { x: DOWNB_X[ENTRY_ID], y: DOWNB_Y[ENTRY_ID], z: 0.0 };
				PostureModule::set_pos(boma, &pos);
				PostureModule::init_pos(boma, &pos, true, true);
				/*if ControlModule::get_stick_x(boma) > 0.2{
					PostureModule::set_lr(boma, -1.0);
					PostureModule::update_rot_y_lr(boma);
				} else {
					PostureModule::set_lr(boma, 1.0);
					PostureModule::update_rot_y_lr(boma);
				};*/
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && HAS_DOWNB[ENTRY_ID] == false {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					HAS_DOWNB[ENTRY_ID] = true;
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
				if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && HAS_DOWNB[ENTRY_ID] == false {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
					};
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					};
				};
			};
			/*if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
				if IS_MEGA[ENTRY_ID] == false {
					IS_MEGA[ENTRY_ID] = true;
				} else {
					IS_MEGA[ENTRY_ID] = false;
				};
			};
			if IS_MEGA[ENTRY_ID] {
				VisibilityModule::set_whole(boma, false);
				if !ArticleModule::is_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM) {
					ArticleModule::generate_article(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM, true, 0);
				};
			} else {
				VisibilityModule::set_whole(boma, true);
				if ArticleModule::is_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM) {
					ArticleModule::remove_exist(boma, *FIGHTER_LUCARIO_GENERATE_ARTICLE_LUCARIOM,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				};
			};*/
		};
    };
}			

pub fn install() {
    smashline::install_acmd_scripts!(
		lucario_dsmash,
		lucario_dsmashs,
		lucario_fair,
		lucario_dtilt,
		lucario_ftilt,
		lucario_downb,
		lucario_downb_eff,
		lucario_nair,
		lucario_dair,
		lucario_sound_downb
    );
	smashline::install_agent_frame_callbacks!(lucario);
}
