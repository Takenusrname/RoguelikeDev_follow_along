use super::{BuilderMap, MetaMapBuilder};
use crate::map::TileType;
use bracket_lib::random::RandomNumberGenerator;

pub struct MapBorder {}

impl MetaMapBuilder for MapBorder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.border(rng, build_data);
    }
}

impl MapBorder {
    #[allow(dead_code)]
    pub fn new() -> Box<MapBorder> {
        Box::new(MapBorder {})
    }

    fn border(&mut self, _rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        for x in 0..build_data.map.width {
            let t_idx = build_data.map.xy_idx(x, 0);
            let b_idx = build_data.map.xy_idx(x, build_data.map.height - 1);
            build_data.map.tiles[t_idx] = TileType::Wall;
            build_data.map.tiles[b_idx] = TileType::Wall;
        }
        for y in 0..build_data.map.height {
            let l_idx = build_data.map.xy_idx(0, y);
            let r_idx = build_data.map.xy_idx(build_data.map.width - 1, y);
            build_data.map.tiles[l_idx] = TileType::Wall;
            build_data.map.tiles[r_idx] = TileType::Wall;
        }
        build_data.map.populate_blocked();
        build_data.take_snapshot();
    }
}
