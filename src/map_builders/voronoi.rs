use std::collections::HashMap;

use super::{
    MapBuilder,
    common::{generate_voronoi_spawn_regions, remove_unreachable_areas_returning_most_distant},
};
use crate::{Map, Position, SHOW_MAPGEN_VISUALIZER, TileType, spawner};
use bracket_lib::{geometry::Point, random::RandomNumberGenerator, terminal::DistanceAlg};
use specs::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum DistanceAlgo {
    Pythagoras,
    Manhattan,
    Chebyshev,
}

pub struct VoronoiCellBuilder {
    map: Map,
    starting_pos: Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<usize>>,
    n_seeds: usize,
    distance_algo: DistanceAlgo,
    spawn_list: Vec<(usize, String)>
}

impl MapBuilder for VoronoiCellBuilder {
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

impl VoronoiCellBuilder {
    #[allow(dead_code)]
    pub fn new(new_depth: i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            n_seeds: 64,
            distance_algo: DistanceAlgo::Chebyshev,
            spawn_list: Vec::new()
        }
    }

    pub fn pythagoras(new_depth: i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            n_seeds: 64,
            distance_algo: DistanceAlgo::Pythagoras,
            spawn_list: Vec::new()
        }
    }
    
    pub fn manhattan(new_depth: i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            n_seeds: 64,
            distance_algo: DistanceAlgo::Manhattan,
            spawn_list: Vec::new()
        }
    }

    pub fn chebyshev(new_depth: i32) -> VoronoiCellBuilder {
        VoronoiCellBuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            n_seeds: 64,
            distance_algo: DistanceAlgo::Chebyshev,
            spawn_list: Vec::new()
        }
    }

    pub fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        let mut voronoi_seeds: Vec<(usize, Point)> = Vec::new();

        while voronoi_seeds.len() < self.n_seeds {
            let vx = rng.roll_dice(1, self.map.width - 1);
            let vy = rng.roll_dice(1, self.map.height - 1);
            let vidx = self.map.xy_idx(vx, vy);

            let candidate = (vidx, Point::new(vx, vy));

            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_dist = vec![(0, 0.0f32); self.n_seeds];
        let mut voronoi_membership: Vec<i32> = vec![0; self.map.width as usize * self.map.height as usize];

        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let x = i as i32 % self.map.width;
            let y = i as i32 / self.map.width;

            for (seed, pos) in voronoi_seeds.iter().enumerate() {
                let dist;
                match self.distance_algo {
                    DistanceAlgo::Pythagoras => {
                        dist = DistanceAlg::PythagorasSquared.distance2d(Point::new(x, y), pos.1);
                    }
                    DistanceAlgo::Manhattan => {
                        dist = DistanceAlg::Manhattan.distance2d(Point::new(x, y), pos.1);
                    }
                    DistanceAlgo::Chebyshev => {
                        dist = DistanceAlg::Chebyshev.distance2d(Point::new(x, y), pos.1);
                    }
                }
                voronoi_dist[seed] = (seed, dist);
            }
            voronoi_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            *vid = voronoi_dist[0].0 as i32;
        }

        for y in 1..self.map.height - 1 {
            for x in 1..self.map.width - 1 {
                let mut neighbors = 0;
                let my_idx = self.map.xy_idx(x, y);
                let my_seed = voronoi_membership[my_idx];

                if voronoi_membership[self.map.xy_idx(x - 1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x + 1, y)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y - 1)] != my_seed { neighbors += 1; }
                if voronoi_membership[self.map.xy_idx(x, y + 1)] != my_seed { neighbors += 1; }

                if neighbors < 2 {
                    self.map.tiles[my_idx] = TileType::Floor;
                }
            }
            self.take_snapshot();
        }

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

        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);

        for area in self.noise_areas.iter() {
            spawner::spawn_region(&self.map, &mut rng, area.1, self.depth, &mut self.spawn_list);
        }
    }
}
