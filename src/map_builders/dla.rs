use std::collections::HashMap;

use super::{
    MapBuilder,
    common::{
        Symmetry, generate_voronoi_spawn_regions, paint,
        remove_unreachable_areas_returning_most_distant,
    },
};
use crate::{Map, Position, SHOW_MAPGEN_VISUALIZER, map::TileType, spawner};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{LineAlg::Bresenham, Point, line2d},
};

#[derive(PartialEq, Clone, Copy)]
pub enum DLAAAlgo {
    WalkInwards,
    WalkOutwards,
    CentralAttractor,
}

pub struct DLABuilder {
    map: Map,
    starting_pos: Position,
    depth: i32,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<usize>>,
    algo: DLAAAlgo,
    brush_size: i32,
    symmetry: Symmetry,
    floor_percent: f32,
    spawn_list: Vec<(usize, String)>,
}

impl MapBuilder for DLABuilder {
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

impl DLABuilder {
    #[allow(dead_code)]
    pub fn new(new_depth: i32) -> DLABuilder {
        DLABuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            algo: DLAAAlgo::WalkInwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
            spawn_list: Vec::new(),
        }
    }

    pub fn walk_inwards(new_depth: i32) -> DLABuilder {
        DLABuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            algo: DLAAAlgo::WalkInwards,
            brush_size: 1,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
            spawn_list: Vec::new(),
        }
    }

    pub fn walk_outwards(new_depth: i32) -> DLABuilder {
        DLABuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            algo: DLAAAlgo::WalkOutwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
            spawn_list: Vec::new(),
        }
    }

    pub fn central_attractor(new_depth: i32) -> DLABuilder {
        DLABuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            algo: DLAAAlgo::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
            spawn_list: Vec::new(),
        }
    }

    pub fn insectoid(new_depth: i32) -> DLABuilder {
        DLABuilder {
            map: Map::new(new_depth),
            starting_pos: Position { x: 0, y: 0 },
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            algo: DLAAAlgo::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::Horizontal,
            floor_percent: 0.25,
            spawn_list: Vec::new(),
        }
    }

    pub fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.starting_pos = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        };
        let start_idx = self.map.xy_idx(self.starting_pos.x, self.starting_pos.y);
        self.take_snapshot();

        self.map.tiles[start_idx] = TileType::Floor;
        self.map.tiles[start_idx - 1] = TileType::Floor;
        self.map.tiles[start_idx + 1] = TileType::Floor;
        self.map.tiles[start_idx - self.map.width as usize] = TileType::Floor;
        self.map.tiles[start_idx + self.map.width as usize] = TileType::Floor;

        let total_tiles = self.map.width * self.map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self
            .map
            .tiles
            .iter()
            .filter(|a| **a == TileType::Floor)
            .count();

        while floor_tile_count < desired_floor_tiles {
            match self.algo {
                DLAAAlgo::WalkInwards => {
                    let mut digger_x = rng.roll_dice(1, self.map.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, self.map.height - 3) + 1;

                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    let mut digger_idx = self.map.xy_idx(digger_x, digger_y);

                    while self.map.tiles[digger_idx] == TileType::Wall {
                        prev_x = digger_x;
                        prev_y = digger_y;

                        let stagger_dir = rng.roll_dice(1, 4);

                        match stagger_dir {
                            1 => {
                                if digger_x > 2 {
                                    digger_x -= 1;
                                }
                            }
                            2 => {
                                if digger_x < self.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < self.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = self.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut self.map,
                        self.symmetry,
                        self.brush_size,
                        prev_x,
                        prev_y,
                    );
                }
                DLAAAlgo::WalkOutwards => {
                    let mut digger_x = self.starting_pos.x;
                    let mut digger_y = self.starting_pos.y;
                    let mut digger_idx = self.map.xy_idx(digger_x, digger_y);

                    while self.map.tiles[digger_idx] == TileType::Floor {
                        let stagger_dir = rng.roll_dice(1, 4);
                        match stagger_dir {
                            1 => {
                                if digger_x > 2 {
                                    digger_x -= 1;
                                }
                            }
                            2 => {
                                if digger_x < self.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < self.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = self.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut self.map,
                        self.symmetry,
                        self.brush_size,
                        digger_x,
                        digger_y,
                    );
                }
                DLAAAlgo::CentralAttractor => {
                    let mut digger_x = rng.roll_dice(1, self.map.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, self.map.height - 3) + 1;

                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    let mut digger_idx = self.map.xy_idx(digger_x, digger_y);

                    let mut path = line2d(
                        Bresenham,
                        Point::new(digger_x, digger_y),
                        Point::new(self.starting_pos.x, self.starting_pos.y),
                    );

                    while self.map.tiles[digger_idx] == TileType::Wall && !path.is_empty() {
                        prev_x = digger_x;
                        prev_y = digger_y;

                        digger_x = path[0].x;
                        digger_y = path[0].y;

                        path.remove(0);

                        digger_idx = self.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut self.map,
                        self.symmetry,
                        self.brush_size,
                        prev_x,
                        prev_y,
                    );
                }
            }

            self.take_snapshot();
            floor_tile_count = self
                .map
                .tiles
                .iter()
                .filter(|a| **a == TileType::Floor)
                .count();
        }

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
