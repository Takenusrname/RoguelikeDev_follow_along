use super::{
    CombatStats, InBackpack, Map, Name, Player, Position, RunState, State, Viewshed, colors::*,
    gamelog::GameLog,
};
use bracket_lib::{
    color::RGB,
    geometry::{Point, Rect},
    pathfinding::DistanceAlg,
    terminal::{BTerm, FontCharType, VirtualKeyCode, letter_to_option, to_cp437},
};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}
#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let bg_rect = Rect::with_size(0, 43, 79, 49);
    ctx.fill_region(
        bg_rect,
        to_cp437(' '),
        RGB::named(DEFAULT_FG),
        RGB::named(DEFAULT_BG),
    );
    ctx.draw_hollow_box(0, 43, 79, 6, RGB::named(DEFAULT_FG), RGB::named(DEFAULT_BG));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(
            12,
            43,
            RGB::named(DEFAULT_FG),
            RGB::named(DEFAULT_BG),
            &health,
        );
        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(HP_FG),
            RGB::named(DEFAULT_BG),
        );
    }

    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(MOUSE_BG));
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, RGB::named(DEFAULT_FG), RGB::named(TT_BG), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(DEFAULT_FG),
                        RGB::named(TT_BG),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(DEFAULT_FG),
                RGB::named(TT_BG),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::named(DEFAULT_FG), RGB::named(TT_BG), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(DEFAULT_FG),
                        RGB::named(TT_BG),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(DEFAULT_FG),
                RGB::named(TT_BG),
                &"<-".to_string(),
            );
        }
    }
}

fn inventory_frame(ctx: &mut BTerm, count: usize, x: i32, y: i32, w: i32, fg: RGB, bg: RGB) {
    let bg_rect = Rect::with_size(x, y - 2, w, (count + 3) as i32);
    let menu_text: &str = " Inventory ";
    let esc_text: &str = " ESC to Cancel ";

    let start_char = to_cp437('┤');
    let end_char = to_cp437('├');

    ctx.fill_region(bg_rect, to_cp437(' '), fg, bg);
    ctx.draw_hollow_box(x, y - 2, w, (count + 3) as i32, fg, bg);

    ctx.set(x + 1, y - 2, fg, bg, start_char);
    ctx.print_color(x + 2, y - 2, bg, fg, menu_text);
    ctx.set(x + menu_text.len() as i32 + 2, y - 2, fg, bg, end_char);
    ctx.print_color(x + 15, y + count as i32 + 1, bg, fg, esc_text);
    ctx.set(x + 14, y + count as i32 + 1, fg, bg, start_char);
    ctx.set(x + 30, y + count as i32 + 1, fg, bg, end_char);
}

fn inventory_selection(
    ctx: &mut BTerm,
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: FontCharType,
    sel_name: &String,
) {
    ctx.set(x, y, fg, bg, to_cp437('('));
    ctx.set(x + 1, y, RGB::named(SEL_FG), bg, glyph);
    ctx.set(x + 2, y, fg, bg, to_cp437(')'));

    ctx.print(x + 4, y, sel_name);
}

pub fn show_inventory(
    gs: &mut State,
    ctx: &mut BTerm,
    action: &str,
) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let x = 15;
    let mut y = (25 - (count / 2)) as i32;
    let w = 31;

    let fg = RGB::named(DEFAULT_FG);
    let bg: RGB;
    if action == "drop" {
        bg = RGB::named(DROP_BG);
    } else if action == "use" {
        bg = RGB::named(INV_BG);
    } else {
        bg = RGB::named(DEFAULT_BG);
    }

    inventory_frame(ctx, count, x, y, w, fg, bg);

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        inventory_selection(ctx, x + 2, y, fg, bg, 65 + j, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut BTerm,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    ctx.print_color(
        5,
        0,
        RGB::named(SEL_FG),
        RGB::named(DEFAULT_BG),
        "Select Target: ",
    );

    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        for idx in visible.visible_tiles.iter() {
            let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, RGB::named(RANGE_BG));
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
            valid_target = true;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(VALID_BG));
        if ctx.left_click {
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            );
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(INVALID_BG));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }
    (ItemMenuResult::NoResponse, None)
}

pub fn main_menu(gs: &mut State, ctx: &mut BTerm) -> MainMenuResult {
    let save_exists = super::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();

    let title_txt = "TODO: THINK OF TITLE";
    let new_game_txt = "Begin New Game";
    let load_game_txt = "Load Game";
    let quit_txt = "Quit";
    let non_sel_fg = RGB::named(NONSEL_FG);
    let sel_fg = RGB::named(SEL_FG);
    let bg = RGB::named(DEFAULT_BG);

    ctx.print_color_centered(15, RGB::named(TITLE_FG), RGB::named(DEFAULT_BG), title_txt);

    if let RunState::MainMenu {
        menu_sel: selection,
    } = *runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(24, sel_fg, bg, new_game_txt);
        } else {
            ctx.print_color_centered(24, non_sel_fg, bg, new_game_txt);
        }
        if selection == MainMenuSelection::LoadGame {
            ctx.print_color_centered(25, sel_fg, bg, load_game_txt);
        } else {
            ctx.print_color_centered(25, non_sel_fg, bg, load_game_txt);
        }
        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(26, sel_fg, bg, quit_txt);
        } else {
            ctx.print_color_centered(26, non_sel_fg, bg, quit_txt);
        }

        match ctx.key {
            None => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                };
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return MainMenuResult::NoSelection {
                        selected: MainMenuSelection::Quit,
                    };
                }
                VirtualKeyCode::Up => {
                    let mut newsel;
                    match selection {
                        MainMenuSelection::NewGame => newsel = MainMenuSelection::Quit,
                        MainMenuSelection::LoadGame => newsel = MainMenuSelection::NewGame,
                        MainMenuSelection::Quit => newsel = MainMenuSelection::LoadGame,
                    }
                    if newsel == MainMenuSelection::LoadGame && !save_exists {
                        newsel = MainMenuSelection::NewGame;
                    }
                    return MainMenuResult::NoSelection { selected: newsel };
                }
                VirtualKeyCode::Down => {
                    let mut newsel: MainMenuSelection;
                    match selection {
                        MainMenuSelection::NewGame => newsel = MainMenuSelection::LoadGame,
                        MainMenuSelection::LoadGame => newsel = MainMenuSelection::Quit,
                        MainMenuSelection::Quit => newsel = MainMenuSelection::NewGame,
                    }
                    if newsel == MainMenuSelection::LoadGame && !save_exists {
                        newsel = MainMenuSelection::Quit;
                    }
                    return MainMenuResult::NoSelection { selected: newsel };
                }
                VirtualKeyCode::Return => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    };
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    };
                }
            },
        }
    }
    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}
