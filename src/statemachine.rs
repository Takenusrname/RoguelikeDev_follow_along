use super::{RunState, State, components::*, gui, player_input, saveload_system};
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
            newrunstate = RunState::MonsterTurn;
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
    }

    {
        let mut runwriter = gs.ecs.write_resource::<RunState>();
        *runwriter = newrunstate;
    }
}
