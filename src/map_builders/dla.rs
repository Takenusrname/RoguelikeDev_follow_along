use super::{
    BuilderMap, InitialMapBuilder,
    common::{Symmetry, paint},
};
use crate::{Position, map::TileType};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{LineAlg::Bresenham, Point, line2d},
};

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum DLAAAlgorithm {
    WalkInwards,
    WalkOutwards,
    CentralAttractor,
}

pub struct DLABuilder {
    algo: DLAAAlgorithm,
    brush_size: i32,
    symmetry: Symmetry,
    floor_percent: f32,
}

impl InitialMapBuilder for DLABuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl DLABuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algo: DLAAAlgorithm::WalkInwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_inwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algo: DLAAAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_outwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algo: DLAAAlgorithm::WalkOutwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn central_attractor() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algo: DLAAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn insectoid() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algo: DLAAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::Horizontal,
            floor_percent: 0.25,
        })
    }

    pub fn build(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        let starting_pos = Position {
            x: build_data.map.width / 2,
            y: build_data.map.height / 2,
        };
        let start_idx = build_data.map.xy_idx(starting_pos.x, starting_pos.y);

        build_data.map.tiles[start_idx] = TileType::Floor;
        build_data.map.tiles[start_idx - 1] = TileType::Floor;
        build_data.map.tiles[start_idx + 1] = TileType::Floor;
        build_data.map.tiles[start_idx - build_data.map.width as usize] = TileType::Floor;
        build_data.map.tiles[start_idx + build_data.map.width as usize] = TileType::Floor;

        build_data.take_snapshot();

        let total_tiles = build_data.map.width * build_data.map.height;
        let desired_floor_tiles = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = build_data
            .map
            .tiles
            .iter()
            .filter(|a| **a == TileType::Floor)
            .count();

        while floor_tile_count < desired_floor_tiles {
            match self.algo {
                DLAAAlgorithm::WalkInwards => {
                    let mut digger_x = rng.roll_dice(1, build_data.map.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, build_data.map.height - 3) + 1;

                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);

                    while build_data.map.tiles[digger_idx] == TileType::Wall {
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
                                if digger_x < build_data.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < build_data.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut build_data.map,
                        self.symmetry,
                        self.brush_size,
                        prev_x,
                        prev_y,
                    );
                }
                DLAAAlgorithm::WalkOutwards => {
                    let mut digger_x = starting_pos.x;
                    let mut digger_y = starting_pos.y;
                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);

                    while build_data.map.tiles[digger_idx] == TileType::Floor {
                        let stagger_dir = rng.roll_dice(1, 4);
                        match stagger_dir {
                            1 => {
                                if digger_x > 2 {
                                    digger_x -= 1;
                                }
                            }
                            2 => {
                                if digger_x < build_data.map.width - 2 {
                                    digger_x += 1;
                                }
                            }
                            3 => {
                                if digger_y > 2 {
                                    digger_y -= 1;
                                }
                            }
                            _ => {
                                if digger_y < build_data.map.height - 2 {
                                    digger_y += 1;
                                }
                            }
                        }
                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut build_data.map,
                        self.symmetry,
                        self.brush_size,
                        digger_x,
                        digger_y,
                    );
                }
                DLAAAlgorithm::CentralAttractor => {
                    let mut digger_x = rng.roll_dice(1, build_data.map.width - 3) + 1;
                    let mut digger_y = rng.roll_dice(1, build_data.map.height - 3) + 1;

                    let mut prev_x = digger_x;
                    let mut prev_y = digger_y;

                    let mut digger_idx = build_data.map.xy_idx(digger_x, digger_y);

                    let mut path = line2d(
                        Bresenham,
                        Point::new(digger_x, digger_y),
                        Point::new(starting_pos.x, starting_pos.y),
                    );

                    while build_data.map.tiles[digger_idx] == TileType::Wall && !path.is_empty() {
                        prev_x = digger_x;
                        prev_y = digger_y;

                        digger_x = path[0].x;
                        digger_y = path[0].y;

                        path.remove(0);

                        digger_idx = build_data.map.xy_idx(digger_x, digger_y);
                    }
                    paint(
                        &mut build_data.map,
                        self.symmetry,
                        self.brush_size,
                        prev_x,
                        prev_y,
                    );
                }
            }

            build_data.take_snapshot();
            floor_tile_count = build_data
                .map
                .tiles
                .iter()
                .filter(|a| **a == TileType::Floor)
                .count();
        }
    }
}
