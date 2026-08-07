use crate::{Map, Position};
use specs::prelude::*;

mod cellular_automata;
use cellular_automata::CellularAutomataBuilder;
mod drunkards;
use drunkards::DrunkardsWalkBuilder;
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;
mod bsp_interior;
use bsp_interior::BspInteriorBuilder;
mod common;
//use common::*;
mod simple_map;
use simple_map::SimpleMapBuilder;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_pos(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    //Box::new(DrunkardsWalkBuilder::new(new_depth,DrunkardSettings { spawn_mode: DrunkSpawnMode::Random, drunken_lifetime: 100, floor_percent: 0.4 }))
    //*
    let mut rng = bracket_lib::random::RandomNumberGenerator::new();
    let builder = rng.roll_dice(1, 7);

    match builder {
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth)),
    } // */
}
