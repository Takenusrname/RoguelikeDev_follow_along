use super::{
    CombatStats, DefenseBonus, Equipped, MeleePowerBonus, Name, SufferDamage, WantsToMelee,
    gamelog::GameLog,
};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, MeleePowerBonus>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            combat_stats,
            defense_bonus,
            equipped,
            entities,
            mut log,
            melee_power_bonus,
            names,
            mut inflict_damage,
            mut wants_melee,
        ) = data;

        for (stats, entity, name, wants_melee) in
            (&combat_stats, &entities, &names, &wants_melee).join()
        {
            if stats.hp > 0 {
                let mut offenseive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in (&entities, &melee_power_bonus, &equipped).join() {
                    if equipped_by.owner == entity {
                        offenseive_bonus += power_bonus.power;
                    }
                }
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    
                    let mut defensive_bonus = 0;
                    for (_item_entity, defense_bonus, equipped_by) in (&entities, &defense_bonus, &equipped).join() {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defense_bonus.defense;
                        }
                    }
                    let damage = i32::max(0, (stats.power + offenseive_bonus) - (target_stats.defense + defensive_bonus));

                    if damage == 0 {
                        log.entries.push(format!(
                            "{} is unable to hurt {}",
                            &name.name, &target_name.name
                        ));
                    } else {
                        log.entries.push(format!(
                            "{} hits {}, for {} hp.",
                            &name.name, &target_name.name, damage
                        ));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
