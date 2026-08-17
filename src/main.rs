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
mod hunger_system;
mod inventory_system;
use inventory_system::{ItemCollectionSystem, ItemDropSystem, ItemRemoveSystem, ItemUseSystem};
pub mod map;
use map::*;
pub mod map_builders;
pub mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod particle_system;
mod player;
use player::*;
pub mod random_table;
mod rect;
pub use rect::Rect;
mod render;
use render::render_world;
mod rex_assets;
pub mod saveload_system;
mod spawner;
mod state_machine;
use state_machine::current_state;
mod trigger_system;
mod visibility_system;
use visibility_system::VisibilitySystem;

const SHOW_MAPGEN_VISUALIZER: bool = true;

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
    NextLevel,
    ShowRemoveItem,
    GameOver,
    MagicMapReveal { row: i32 },
    MapGeneration,
    Screenshot,
}

struct State {
    pub ecs: World,
    mapgen_next_state: Option<RunState>,
    mapgen_history: Vec<Map>,
    mapgen_index: usize,
    mapgen_timer: f32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let newrunstate;

        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();
        particle_system::cull_dead_particles(&mut self.ecs, ctx);

        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::GameOver { .. } => {}
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
        let mut triggers = trigger_system::TriggerSystem {};
        triggers.run_now(&self.ecs);
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
        let mut item_remove = ItemRemoveSystem {};
        item_remove.run_now(&self.ecs);
        let mut hunger = hunger_system::HungerSystem {};
        hunger.run_now(&self.ecs);
        let mut particles = particle_system::ParticleSpawnSystem {};
        particles.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let player_entity = self.ecs.fetch::<Entity>();
        let equipped = self.ecs.read_storage::<Equipped>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            let p = player.get(entity);
            if let Some(_p) = p {
                should_delete = false;
            }

            let bp = backpack.get(entity);
            if let Some(bp) = bp {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }

            let eq = equipped.get(entity);
            if let Some(eq) = eq {
                if eq.owner == *player_entity {
                    should_delete = false;
                }
            }

            if should_delete {
                to_delete.push(entity);
            }
        }
        to_delete
    }

    fn goto_next_level(&mut self) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs
                .delete_entity(target)
                .expect("Unable to delete entity");
        }

        let current_depth;
        {
            let worldmap_res = self.ecs.write_resource::<Map>();
            current_depth = worldmap_res.depth;
        }

        self.generate_world_map(current_depth + 1);

        let player_entity = self.ecs.fetch::<Entity>();

        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
        gamelog
            .entries
            .push("You descend to the next level, and take a moment to heal.".to_string());

        let mut player_health_store = self.ecs.write_storage::<CombatStats>();
        let player_health = player_health_store.get_mut(*player_entity);
        if let Some(player_health) = player_health {
            player_health.hp = i32::max(player_health.hp, player_health.max_hp / 2);
        }
    }

    fn game_over_cleanup(&mut self) {
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }

        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }
        saveload_system::delete_save();

        {
            let player_entity = spawner::player(&mut self.ecs, 0, 0);
            let mut player_entity_writer = self.ecs.write_resource::<Entity>();
            *player_entity_writer = player_entity;
        }

        self.generate_world_map(1);
    }

    fn generate_world_map(&mut self, new_depth: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();

        let mut rng = self.ecs.write_resource::<RandomNumberGenerator>();
        
        let mut builder = map_builders::random_builder(new_depth, &mut rng);
        builder.build_map(&mut rng);
        
        std::mem::drop(rng);
        
        self.mapgen_history = builder.build_data.history.clone();

        let player_start;
        {
            let mut worldmap_res = self.ecs.write_resource::<Map>();
            *worldmap_res = builder.build_data.map.clone();
            player_start = builder.build_data.starting_pos.as_mut().unwrap().clone();
        }

        builder.spawn_entities(&mut self.ecs);

        let (player_x, player_y) = (player_start.x, player_start.y);
        let mut player_pos = self.ecs.write_resource::<Point>();
        *player_pos = Point::new(player_x, player_y);

        let mut position_comps = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.fetch::<Entity>();
        let player_pos_comp = position_comps.get_mut(*player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        let mut viewshed_comps = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_comps.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
    }
}

embedded_resource!(FONT_S, "../resources/cp437_8x8_mod.png");
embedded_resource!(FONT_M, "../resources/cp437_12x12_mod.png");
embedded_resource!(FONT_L, "../resources/cp437_16x16_mod.png");

fn main() -> BError {
    use bracket_lib::terminal::BTermBuilder;

    link_resource!(FONT_S, "resources/cp437_8x8_mod.png");
    link_resource!(FONT_M, "resources/cp437_12x12_mod.png");
    link_resource!(FONT_L, "resources/cp437_16x16_mod.png");

    let mut ctx = BTermBuilder::simple(80, 50)
        .unwrap()
        .with_title("McGuffin Quest")
        .with_font("cp437_8x8_mod.png", 8, 8)
        .with_font("cp437_12x12_mod.png", 12, 12)
        .with_font("cp437_16x16_mod.png", 16, 16)
        .with_tile_dimensions(16, 16)
        .build()?;

    ctx.set_active_font(3, false); // 1 = 8x8 / 2 = 12x12 / 3 = 16x16
    ctx.with_mouse_visibility(false);
    ctx.with_post_scanlines(false);
    let mut gs = State {
        ecs: World::new(),
        mapgen_next_state: Some(RunState::PreRun),
        mapgen_index: 0,
        mapgen_history: Vec::new(),
        mapgen_timer: 0.0,
    };

    component_registration(&mut gs.ecs);

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    gs.ecs.insert(Map::new(1));
    gs.ecs.insert(Point::new(0, 0));
    gs.ecs.insert(RandomNumberGenerator::new());

    //let save = saveload_system::does_save_exist();

    let player_entity = spawner::player(&mut gs.ecs, 0, 0);
    gs.ecs.insert(player_entity);

    /*
    if save {
        gs.ecs.insert(RunState::MainMenu {
            menu_sel: gui::MainMenuSelection::LoadGame,
        });
    } else {
        gs.ecs.insert(RunState::MainMenu {
            menu_sel: gui::MainMenuSelection::NewGame,
        });
    }// */

    gs.ecs.insert(RunState::MapGeneration {});
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to MQ".to_string()],
    });

    gs.ecs.insert(particle_system::ParticleBuilder::new());
    gs.ecs.insert(rex_assets::RexAssets::new());

    gs.generate_world_map(1);

    main_loop(ctx, gs)
}
