use bracket_lib::terminal::BTerm;
use specs::prelude::*;

use super::{camera, gui};

pub fn render_world(ecs: &mut World, ctx: &mut BTerm) {
    camera::render_camera(&ecs, ctx);
    gui::draw_ui(&ecs, ctx);
}
