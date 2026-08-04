use super::{Map, colors::*, components::*, gamelog::GameLog, particle_system::ParticleBuilder};
use bracket_lib::{color::RGB, terminal::to_cp437};
use specs::prelude::*;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, EntryTrigger>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, InflictsDamage>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, SingleActivation>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut entity_moved,
            entry_trigger,
            mut log,
            mut hidden,
            inflicts_damage,
            map,
            names,
            mut particle_builder,
            position,
            single_activation,
            mut inflict_damage,
        ) = data;

        let mut remove_entities: Vec<Entity> = Vec::new();
        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id {
                    let maybe_trigger = entry_trigger.get(*entity_id);
                    match maybe_trigger {
                        None => {}
                        Some(_trigger) => {
                            let name = names.get(*entity_id);
                            if let Some(name) = name {
                                log.entries.push(format!("{} triggers!", &name.name));
                            }

                            hidden.remove(*entity_id);

                            let damage = inflicts_damage.get(*entity_id);
                            if let Some(damage) = damage {
                                particle_builder.requests(
                                    pos.x,
                                    pos.y,
                                    RGB::named(DAMAGE_FG),
                                    RGB::named(LIT_BG),
                                    to_cp437('☼'),
                                    200.0,
                                );
                                SufferDamage::new_damage(
                                    &mut inflict_damage,
                                    entity,
                                    damage.damage,
                                );
                            }

                            let sa = single_activation.get(*entity_id);
                            if let Some(_sa) = sa {
                                remove_entities.push(*entity_id);
                            }
                        }
                    }
                }
            }
        }

        for trap in remove_entities.iter() {
            entities.delete(*trap).expect("Unable to delete trap");
        }

        entity_moved.clear();
    }
}
