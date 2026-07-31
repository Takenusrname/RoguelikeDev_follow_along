use specs::prelude::*;
use bracket_lib::terminal::BTerm;

use super::{Position, Renderable, Map, gui, draw_map};


pub fn render_world(ecs: &mut World, ctx: &mut BTerm) {
    draw_map(&ecs, ctx);

    {
        let positions = ecs.read_storage::<Position>();
        let renderables = ecs.read_storage::<Renderable>();
        let map = ecs.fetch::<Map>();

        let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
        for (pos, render) in data.iter() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }

        gui::draw_ui(&ecs, ctx);
    }
}