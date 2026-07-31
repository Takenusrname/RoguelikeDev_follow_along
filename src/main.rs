use bracket_lib::{
    geometry::Point,
    prelude::BError,
    random::RandomNumberGenerator,
    terminal::{BTerm, EMBED, GameState, embedded_resource, link_resource, main_loop},
};
use specs::{prelude::*, saveload::SimpleMarkerAllocator};

mod colors;
pub use colors::*;
mod components;
pub use components::*;
mod damage_system;
use damage_system::DamageSystem;
mod gamelog;
mod gui;
mod inventory_system;
use inventory_system::{ItemCollectionSystem, ItemDropSystem, ItemUseSystem};
pub mod map;
use map::*;
pub mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod render;
use render::render_world;
mod saveload_system;
mod spawner;
mod statemachine;
use statemachine::current_state;
mod visibility_system;
use visibility_system::VisibilitySystem;

/// RUNSTATES
///
/// PRERUN -> MAINMENU -> CHARGEN -> MAPGEN -> AwaitingInput -> PlayerTurn <-> MonsterTurn
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    MonsterTurn,
    PlayerTurn,
    PreRun,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
    MainMenu { menu_sel: gui::MainMenuSelection },
    SaveGame,
}

struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let newrunstate;

        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu { .. } => {}
            _ => {
                render_world(&mut self.ecs, ctx);
            }
        }

        current_state(self, ctx, newrunstate);

        damage_system::delete_the_dead(&mut self.ecs);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut itemuse = ItemUseSystem {};
        itemuse.run_now(&self.ecs);
        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

embedded_resource!(FONT_FILE, "../resources/cp437_16x16_mod.png");

fn main() -> BError {
    use bracket_lib::terminal::BTermBuilder;

    link_resource!(FONT_FILE, "resources/cp437_16x16_mod.png");

    let mut ctx = BTermBuilder::simple(80, 50)
        .unwrap()
        .with_title("mq")
        .with_font("cp437_16x16_mod.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .build()?;

    ctx.set_active_font(1, false);
    ctx.with_mouse_visibility(false);
    ctx.with_post_scanlines(false);
    let mut gs = State { ecs: World::new() };

    component_registration(&mut gs.ecs);

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(RandomNumberGenerator::new());

    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MainMenu {
        menu_sel: gui::MainMenuSelection::NewGame,
    });
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to MQ".to_string()],
    });

    main_loop(ctx, gs)
}
