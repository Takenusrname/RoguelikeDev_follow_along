use bracket_lib::terminal::{XpFile, EMBED, embedded_resource, link_resource};

embedded_resource!(TS, "../resources/mq_80x50.xp");

pub struct RexAssets {
    pub menu: XpFile
}

impl RexAssets {
    pub fn new() -> RexAssets {
        link_resource!(TS, "../resources/mq_80x50.xp");

        RexAssets { menu: XpFile::from_resource("../resources/mq_80x50.xp").unwrap() }
    }
}
