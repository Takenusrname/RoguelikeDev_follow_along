#![allow(unused)]

use bracket_lib::color::RGB;

mod palette;
use palette::*;

pub fn return_u8_rgb(color_val: (f32, f32, f32)) -> RGB {
    let rgb = RGB::from_f32(color_val.0, color_val.1, color_val.2);
    return rgb;
}

pub fn return_f32_rgb(color_val: (u8, u8, u8)) -> (f32, f32, f32) {
    let rgb = RGB::from_u8(color_val.0, color_val.1, color_val.2);
    return (rgb.r, rgb.g, rgb.b);
}


// Defaults
pub const DEFAULT_FG: (u8, u8, u8) = DB32_COLOR11;
pub const DEFAULT_BG: (u8, u8, u8) = DB32_COLOR32;

// SCREENBURN
pub const SB_COLOR: (u8, u8,u8) = DB32_COLOR27;

// Player
pub const PLAYER_FG: (u8, u8, u8 ) = DB32_COLOR32;
pub const PLAYER_BG: (u8, u8, u8 ) = DB32_COLOR23;
pub const LIT_BG: (u8, u8, u8) = DB32_COLOR31; 

// MONSTERS
pub const GOBLIN_FG: (u8, u8, u8) = DB32_COLOR21;
pub const ORC_FG: (u8, u8, u8) = DB32_COLOR10;

// Dungeon
pub const WALL_FG: (u8, u8, u8) = DB32_COLOR10;
pub const WALL_BG: (u8, u8, u8) = DB32_COLOR31;
pub const FLOOR_FG: (u8, u8, u8) = DB32_COLOR28;
pub const FLOOR_BG: (u8, u8, u8) = DB32_COLOR31;
pub const MEM_FG: (u8, u8, u8) = DB32_COLOR07;
pub const AETHER: (u8, u8, u8) = DB32_COLOR18;
pub const STAIRS_FG: (u8, u8, u8) = DB32_COLOR13;
pub const BLOOD_BG: (u8, u8, u8) = DB32_COLOR05;
pub const DOOR_FG: (u8, u8, u8) = DB32_COLOR28;

// Items
pub const POT_HEALTH_FG: (u8, u8, u8) = DB32_COLOR04;
pub const SCROLL_MM_FG:(u8, u8,u8) = DB32_COLOR13;
pub const SCROLL_FB_FG: (u8, u8, u8) = DB32_COLOR27;
pub const SCROLL_CON_FG: (u8, u8, u8) = DB32_COLOR24;
pub const DAGGER_FG: (u8, u8, u8) = DB32_COLOR13;
pub const SHIELD_FG: (u8, u8, u8) = DB32_COLOR12;
pub const LSWORD_FG: (u8, u8, u8) = DB32_COLOR10;
pub const TSHIELD_FG: (u8, u8, u8) = DB32_COLOR09;
pub const RATION_FG: (u8, u8, u8) = DB32_COLOR26;
pub const MAP_SCROLL_FG: (u8, u8, u8) = DB32_COLOR06;
pub const BEARTRAP_FG: (u8, u8,u8) = DB32_COLOR27;

// Particles
pub const ATTACK_FG: (u8, u8, u8) = DB32_COLOR27;
pub const HEAL_FG: (u8, u8, u8) = DB32_COLOR23;
pub const CONFUSION_FG: (u8, u8, u8) = DB32_COLOR03;
pub const DAMAGE_FG: (u8, u8, u8) = DB32_COLOR05;
pub const AOE_FG: (u8, u8,u8) = DB32_COLOR27;

// UI
pub const HP_FG: (u8, u8, u8) = DB32_COLOR05;
pub const INV_BG: (u8, u8, u8) = DB32_COLOR14;
pub const DROP_BG: (u8, u8, u8) = DB32_COLOR04;
pub const NONSEL_FG: (u8, u8, u8) = DB32_COLOR07;
pub const SEL_FG: (u8, u8, u8) = DB32_COLOR24;
pub const MOUSE_BG: (u8, u8, u8) = DB32_COLOR03;
pub const TT_BG: (u8, u8, u8) = DB32_COLOR09;
pub const RANGE_BG: (u8, u8, u8) = DB32_COLOR15;
pub const VALID_BG: (u8, u8, u8) = DB32_COLOR13;
pub const INVALID_BG: (u8, u8, u8) = DB32_COLOR05;
pub const TITLE_FG: (u8, u8, u8) = DB32_COLOR24;
pub const ERROR_BG: (u8, u8, u8) = DB32_COLOR03;
pub const UNEQUIP_BG: (u8, u8, u8) = DB32_COLOR27;
pub const DEATH_FG: (u8, u8, u8) = DB32_COLOR05;

// Hunger
pub const WF_FG: (u8, u8, u8) = DB32_COLOR23;
pub const FED_FG: (u8, u8, u8) = DB32_COLOR22;
pub const HUNGRY_FG: (u8, u8, u8) = DB32_COLOR27;
pub const STARVE_FG: (u8, u8, u8) = DB32_COLOR05;