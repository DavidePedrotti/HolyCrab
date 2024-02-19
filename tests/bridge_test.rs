#[cfg(test)]
mod bridge_test{
    use holy_crab_davide_ai::MinerRobot;
    use robotics_lib::world::tile::{Tile, TileType, Content};

    #[test]
    fn test_get_paving_cost() {
        let map = vec![
            vec![Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }],
        ];
        let miner_robot = MinerRobot::new();
        let paving_cost = miner_robot.get_paving_cost(&map,(0,0),(2,2));
        assert_eq!(paving_cost,9);
    }
}