use super::{
    Confusion, colors::*, Map, Monster, Position, RunState, Viewshed, WantsToMelee,
    particle_system::ParticleBuilder,
};
use bracket_lib::{
    geometry::{DistanceAlg::Pythagoras, Point},
    pathfinding::a_star_search,
    terminal::{RGB, to_cp437},
};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteStorage<'a, Confusion>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        WriteExpect<'a, ParticleBuilder>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, RunState>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut confused,
            player_entity,
            entities,
            mut map,
            monster,
            mut particle_builder,
            player_pos,
            mut position,
            runstate,
            mut viewshed,
            mut wants_to_melee,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, _monster, pos, viewshed) in
            (&entities, &monster, &mut position, &mut viewshed).join()
        {
            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;

                particle_builder.requests(
                    pos.x,
                    pos.y,
                    RGB::named(CONFUSION_FG),
                    RGB::named(LIT_BG),
                    to_cp437('?'),
                    200.0,
                );
            }

            if can_act {
                let distance = Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *player_entity,
                            },
                        )
                        .expect("Unable to insert attack");
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = a_star_search(
                        map.xy_idx(pos.x, pos.y),
                        map.xy_idx(player_pos.x, player_pos.y),
                        &mut *map,
                    );

                    if path.success && path.steps.len() > 1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}
