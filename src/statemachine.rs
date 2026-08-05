use super::{
    MAPHEIGHT, MAPWIDTH, Map, RunState, SHOW_MAPGEN_VISUALIZER, State, components::*, draw_map,
    gui, player_input, saveload_system,
};
use bracket_lib::terminal::BTerm;
use specs::prelude::*;

pub fn current_state(gs: &mut State, ctx: &mut BTerm, rs: RunState) {
    let mut newrunstate = rs;
    match newrunstate {
        RunState::PreRun => {
            gs.run_systems();
            gs.ecs.maintain();
            newrunstate = RunState::AwaitingInput;
        }

        RunState::AwaitingInput => {
            newrunstate = player_input(gs, ctx);
        }

        RunState::PlayerTurn => {
            gs.run_systems();
            gs.ecs.maintain();
            match *gs.ecs.fetch::<RunState>() {
                RunState::MagicMapReveal { .. } => {
                    newrunstate = RunState::MagicMapReveal { row: 0 }
                }
                _ => newrunstate = RunState::MonsterTurn,
            }
        }

        RunState::MonsterTurn => {
            gs.run_systems();
            gs.ecs.maintain();
            newrunstate = RunState::AwaitingInput;
        }

        RunState::ShowInventory => {
            let result = gui::show_inventory(gs, ctx, "use");
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    let item_entity = result.1.unwrap();
                    let is_ranged = gs.ecs.read_storage::<Ranged>();
                    let is_item_ranged = is_ranged.get(item_entity);
                    if let Some(is_item_ranged) = is_item_ranged {
                        newrunstate = RunState::ShowTargeting {
                            range: is_item_ranged.range,
                            item: item_entity,
                        }
                    } else {
                        let mut intent = gs.ecs.write_storage::<WantsToUseItem>();
                        intent
                            .insert(
                                *gs.ecs.fetch::<Entity>(),
                                WantsToUseItem {
                                    item: item_entity,
                                    target: None,
                                },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
        }

        RunState::ShowDropItem => {
            let result = gui::show_inventory(gs, ctx, "drop");
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    let item_entity = result.1.unwrap();
                    let mut intent = gs.ecs.write_storage::<WantsToDropItem>();
                    intent
                        .insert(
                            *gs.ecs.fetch::<Entity>(),
                            WantsToDropItem { item: item_entity },
                        )
                        .expect("Unable to insert intent");
                    newrunstate = RunState::PlayerTurn;
                }
            }
        }

        RunState::ShowRemoveItem => {
            let result = gui::show_inventory(gs, ctx, "unequip");
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    let item_entity = result.1.unwrap();
                    let mut intent = gs.ecs.write_storage::<WantsToRemoveItem>();
                    intent
                        .insert(
                            *gs.ecs.fetch::<Entity>(),
                            WantsToRemoveItem { item: item_entity },
                        )
                        .expect("Unable to insert intent");
                    newrunstate = RunState::PlayerTurn;
                }
            }
        }

        RunState::ShowTargeting { range, item } => {
            let result = gui::ranged_target(gs, ctx, range);
            match result.0 {
                gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                gui::ItemMenuResult::NoResponse => {}
                gui::ItemMenuResult::Selected => {
                    let mut intent = gs.ecs.write_storage::<WantsToUseItem>();
                    intent
                        .insert(
                            *gs.ecs.fetch::<Entity>(),
                            WantsToUseItem {
                                item,
                                target: result.1,
                            },
                        )
                        .expect("Unable to insert intent");
                    newrunstate = RunState::PlayerTurn;
                }
            }
        }

        RunState::MainMenu { .. } => {
            let result = gui::main_menu(gs, ctx);
            match result {
                gui::MainMenuResult::NoSelection { selected } => {
                    newrunstate = RunState::MainMenu { menu_sel: selected }
                }

                gui::MainMenuResult::Selected { selected } => match selected {
                    gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                    gui::MainMenuSelection::LoadGame => {
                        saveload_system::load_game(&mut gs.ecs);
                        newrunstate = RunState::AwaitingInput;
                        saveload_system::delete_save();
                    }
                    gui::MainMenuSelection::Quit => {
                        ::std::process::exit(0);
                    }
                },
            }
        }

        RunState::GameOver => {
            let result = gui::game_over(ctx);
            match result {
                gui::GameOverResult::NoSelection => {}
                gui::GameOverResult::QuitToMenu => {
                    gs.game_over_cleanup();
                    newrunstate = RunState::MainMenu {
                        menu_sel: gui::MainMenuSelection::NewGame,
                    };
                }
            }
        }

        RunState::SaveGame => {
            saveload_system::save_game(&mut gs.ecs);
            newrunstate = RunState::MainMenu {
                menu_sel: gui::MainMenuSelection::LoadGame,
            }
        }

        RunState::NextLevel => {
            gs.goto_next_level();
            newrunstate = RunState::PreRun;
        }

        RunState::MagicMapReveal { row } => {
            let mut map = gs.ecs.fetch_mut::<Map>();
            for x in 0..MAPWIDTH {
                let idx = map.xy_idx(x as i32, row);
                map.revealed_tiles[idx] = true;
            }
            if row as usize == MAPHEIGHT - 1 {
                newrunstate = RunState::MonsterTurn;
            } else {
                newrunstate = RunState::MagicMapReveal { row: row + 1 }
            }
        }
        RunState::MapGeneration => {
            if !SHOW_MAPGEN_VISUALIZER {
                newrunstate = gs.mapgen_next_state.unwrap();
            }
            ctx.cls();
            draw_map(&gs.mapgen_history[gs.mapgen_index], ctx);

            gs.mapgen_timer += ctx.frame_time_ms;
            if gs.mapgen_timer > 300.0 {
                gs.mapgen_timer = 0.0;
                gs.mapgen_index += 1;
                if gs.mapgen_index >= gs.mapgen_history.len() {
                    newrunstate = gs.mapgen_next_state.unwrap();
                }
            }
        }
    }

    {
        let mut runwriter = gs.ecs.write_resource::<RunState>();
        *runwriter = newrunstate;
    }
}
