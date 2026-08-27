use super::{Hidden, Map, Position, Renderable, TileType, colors::*};
use bracket_lib::{
    geometry::Point,
    terminal::{BTerm, FontCharType, RGB, to_cp437},
};
use specs::prelude::*;

const SHOW_BOUNDARIES: bool = true;

pub fn get_screen_bounds(ecs: &World, ctx: &mut BTerm) -> (i32, i32, i32, i32) {
    let player_pos = ecs.fetch::<Point>();
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    (min_x, max_x, min_y, max_y)
}

pub fn render_camera(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let (min_x, max_x, min_y, max_y) = get_screen_bounds(ecs, ctx);

    let map_width = map.width - 1;
    let map_height = map.height - 1;

    let mut y = 0;
    for ty in min_y..max_y {
        let mut x = 0;
        for tx in min_x..max_x {
            if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                let idx = map.xy_idx(tx, ty);
                if map.revealed_tiles[idx] {
                    let (glyph, fg, bg) = get_tile_glyph(idx, &*map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES {
                ctx.set(
                    x,
                    y,
                    RGB::named(BOUNDARY_FG),
                    RGB::named(DEFAULT_BG),
                    to_cp437('·'),
                );
            }
            x += 1;
        }
        y += 1;
    }

    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let hidden = ecs.read_storage::<Hidden>();
    let map = ecs.fetch::<Map>();

    let mut data = (&positions, &renderables, !&hidden)
        .join()
        .collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render, _hidden) in data.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            let entity_screen_x = pos.x - min_x;
            let entity_screen_y = pos.y - min_y;
            if entity_screen_x > 0
                && entity_screen_x < map_width
                && entity_screen_y > 0
                && entity_screen_y < map_height
            {
                ctx.set(
                    entity_screen_x,
                    entity_screen_y,
                    render.fg,
                    render.bg,
                    render.glyph,
                );
            }
        }
    }
}

pub fn render_debug_map(map: &Map, ctx: &mut BTerm) {
    let player_pos = Point::new(map.width / 2, map.height / 2);
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    let map_width = map.width - 1;
    let map_height = map.height - 1;

    let mut y = 0;
    for ty in min_y..max_y {
        let mut x = 0;
        for tx in min_x..max_x {
            if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                let idx = map.xy_idx(tx, ty);
                if map.revealed_tiles[idx] {
                    let (glyph, fg, bg) = get_tile_glyph(idx, &*map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES {
                ctx.set(
                    x,
                    y,
                    RGB::named(BOUNDARY_FG),
                    RGB::named(DEFAULT_BG),
                    to_cp437('·'),
                );
            }
            x += 1;
        }
        y += 1;
    }
}

fn get_tile_glyph(idx: usize, map: &Map) -> (FontCharType, RGB, RGB) {
    let glyph;
    let mut fg;
    let mut bg = RGB::named(LIT_BG);

    match map.tiles[idx] {
        TileType::Floor => {
            glyph = to_cp437('.');
            fg = RGB::named(FLOOR_FG);
        }
        TileType::Wall => {
            let x = idx as i32 % map.width;
            let y = idx as i32 / map.width;
            glyph = wall_glyph(&*map, x, y);
            fg = RGB::named(WALL_FG);
        }
        TileType::DownStairs => {
            glyph = to_cp437('»');
            fg = RGB::named(STAIRS_FG);
        }
    }
    if map.bloodstains.contains(&idx) {
        bg = RGB::named(BLOOD_BG);
    }
    if !map.visible_tiles[idx] {
        fg = RGB::named(MEM_FG);
        bg = RGB::named(DEFAULT_BG);
    }
    (glyph, fg, bg)
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
