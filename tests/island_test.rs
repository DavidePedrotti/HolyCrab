
#[cfg(test)]
mod island_test {
    use holy_crab_davide_ai::MinerRobot;
    use robotics_lib::world::tile::{Tile, TileType, Content};

    #[test]
    fn test_get_robot_island() {
        let mut miner_robot = MinerRobot::new();
        let (robot_row,robot_col) = miner_robot.get_coordinates();
        let islands = vec![vec![(0,0),(robot_row as i32,robot_col as i32)],vec![(1,1)]];
        assert_eq!(Some(vec![(0,0),(robot_row as i32,robot_col as i32)]),miner_robot.get_robot_island(&islands));
    }

    #[test]
    fn test_get_closest_points() {
        let map = vec![
            vec![Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Sand, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Mountain, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::Hill, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Street, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Mountain, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::ShallowWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }],
        ];

        let miner_robot = MinerRobot::new();
        let robot_island = vec![(1, 1)];
        let target_island = vec![(0, 0), (2, 2)];
        let result = miner_robot.get_closest_points(&map,robot_island,target_island);

        assert_eq!(result, Some(((0, 0), (1, 1))));
    }

    #[test]
    fn test_get_islands() {
        let map = vec![
            vec![Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Street, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Mountain, content: Content::None, elevation: 0 }],
            vec![Tile { tile_type: TileType::DeepWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::ShallowWater, content: Content::None, elevation: 0 }, Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }],
        ];
        let result = vec![
            vec![(0,0)],
            vec![(1, 1), (1, 2), (2, 2)]
        ];
        let miner_robot = MinerRobot::new();
        let islands = miner_robot.get_islands(&map);
        assert_eq!(result,islands);
    }
}