use super::colors::*;
use bracket_lib::{
    algorithm_traits::{Algorithm2D, BaseMap},
    color::RGB,
    geometry::{DistanceAlg::Pythagoras, Point},
    prelude::SmallVec,
    terminal::{BTerm, FontCharType, to_cp437},
};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::collections::HashSet;

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,

    #[serde(skip)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }
}

impl Map {
    pub fn new(new_depth: i32) -> Map {
        Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            depth: new_depth,
            bloodstains: HashSet::new(),
            tile_content: vec![Vec::new(); MAPCOUNT],
        }
    }
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

pub fn draw_map(map: &Map, ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg: RGB;
            let mut bg: RGB;
            match tile {
                TileType::Floor => {
                    glyph = to_cp437('.');
                    fg = RGB::named(FLOOR_FG);
                    bg = RGB::named(FLOOR_BG);
                }
                TileType::Wall => {
                    glyph = wall_glyph(&*map, x, y);
                    fg = RGB::named(WALL_FG);
                    bg = RGB::named(WALL_BG);
                }
                TileType::DownStairs => {
                    glyph = to_cp437('»');
                    fg = RGB::named(STAIRS_FG);
                    bg = RGB::named(LIT_BG);
                }
            }
            if map.bloodstains.contains(&idx) {
                bg = RGB::named(BLOOD_BG);
            }
            if !map.visible_tiles[idx] {
                fg = RGB::named(MEM_FG);
                bg = RGB::named(DEFAULT_BG);
            }
            ctx.set(x, y, fg, bg, glyph)
        } else {
            let glyph = to_cp437('≈');
            let fg = RGB::named(AETHER);
            let bg = RGB::named(DEFAULT_BG);
            ctx.set(x, y, fg, bg, glyph);
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

/// Checks if the tile is in bounds.
/// if the tile is inbounds it returns true and
/// if it is out of bounds it returns false
pub fn is_inbounds(map: &Map, x: i32, y: i32) -> bool {
    if x < 0 || x > map.width - 1 || y < 0 || y > map.height - 1 {
        return false;
    } else {
        return true;
    }
}

/// Checks if the tile is a wall and it's revealed
fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

/// Checks if the wall tile has other wall tiles if it does
/// it grabs the glyph tied to the mask depending how many
/// other walls are next to it.
fn wall_glyph(map: &Map, x: i32, y: i32) -> FontCharType {
    /*
    8-bit mask values
    1, 2, 4, 8

      1
    4 # 8
      2
    */

    let mut mask: u8 = 0;

    // Check North
    if is_inbounds(map, x, y - 1) && is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    // Check South
    if is_inbounds(map, x, y + 1) && is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    // Check West
    if is_inbounds(map, x - 1, y) && is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    // Check East
    if is_inbounds(map, x + 1, y) && is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // ○ pillar
        1 => 208,  // ╨ wall only to north
        2 => 210,  // ╥ wall only to south
        3 => 186,  // ║ wall to north and south
        4 => 181,  // ╡ wall only to west
        5 => 188,  // ╝ wall to north and west
        6 => 187,  // ╗ Wall to south and west
        7 => 185,  // ╣ wall to north, south and west
        8 => 198,  // ╞ wall to the east
        9 => 200,  // ╚ wall to north and east
        10 => 201, // ╔ wall to south and east
        11 => 204, // ╠ wall to north, south and east
        12 => 205, // ═ wall to east and west
        13 => 202, // ╩ wall to east, west and north
        14 => 203, // ╦ wall to east west and south
        15 => 206, // ╬ wall to north, east, south and west
        _ => 35,   // # missed one?
    }
}
