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
    spawn_list: Vec<(usize, String)>,
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

    fn get_spawn_list(&self) -> &Vec<(usize, String)> {
        &self.spawn_list
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
            spawn_list: Vec::new(),
        }
    }

    pub fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.starting_pos = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        };
        let mut start_idx = self.map.xy_idx(self.starting_pos.x, self.starting_pos.y);
        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_pos.x -= 1;
            start_idx = self.map.xy_idx(self.starting_pos.x, self.starting_pos.y);
        }
        self.take_snapshot();

        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        self.map.tiles[exit_tile] = TileType::DownStairs;
        self.take_snapshot();

        /*
        // For non room based maps
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);
        for area in self.noise_areas.iter() {
            spawner::spawn_region(&self.map, &mut rng, area.1, self.depth, &mut self.spawn_list);
        }

        // for room based maps
        for room in self.rooms.iter().skip(1) {
            spawner::spawn_room(&self.map, &mut rng, room, self.depth, &mut self.spawn_list);
        }*/
    }
}
