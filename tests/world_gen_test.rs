#[cfg(test)]
mod tests {
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use worldgen_unwrap::public::WorldgeneratorUnwrap;
    use robotics_lib::world::world_generator::Generator;
    #[test]
    fn test_world_creation() {
        let gui_start = true;
        let path = None;
        let mut world_generator = WorldgeneratorUnwrap::init(gui_start, path);
        world_generator.gen();
    }
}