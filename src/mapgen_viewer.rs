use bracket_lib::terminal::{BError, BTerm, BTermBuilder, EMBED, GameState, embedded_resource, link_resource, main_loop};
use specs::prelude::*;

embedded_resource!(FONT_S, "../resources/cp437_8x8_mod.png");

struct State {
    pub ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        ctx.cls();

        ctx.print(2, 2, "Test Write");
    } 
}

fn main() -> BError {
    link_resource!(FONT_S, "resources/cp437_8x8_mod.png");

    let mut ctx_main = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_title("Map Generator Viewer")
        .with_font("cp437_8x8_mod.png", 8, 8)
        .build()?;

    ctx_main.set_active_font(1, false);

    let mut gs = State {
        ecs: World::new()
    };

    main_loop(ctx_main, gs)

}