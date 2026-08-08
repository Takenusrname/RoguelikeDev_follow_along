use std::collections::HashMap;

use super::{
    MapBuilder,
    common::{generate_voronoi_spawn_regions, remove_unreachable_areas_returning_most_distant},
};
use crate::{Map, Position, Rect, SHOW_MAPGEN_VISUALIZER, map::TileType, spawner};
use bracket_lib::random::RandomNumberGenerator;
use specs::prelude::*;

pub struct MapBoiler {
    map: Map,
    starting_pos: Position,
    depth: i32,
    rooms: Vec<Rect>,
    history: Vec<Map>,
}

impl MapBuilder for MapBoiler {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_pos(&self) -> Position {
        self.starting_pos.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self) {
        self.build();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for room in self.rooms.iter().skip(1) {
            spawner::spawn_room(ecs, room, self.depth);
        }
    }

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

impl MapBoiler {
    pub fn new(new_depth: i32) -> MapBoiler {
        MapBoiler {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            rooms: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn build(&mut self) {}
}
