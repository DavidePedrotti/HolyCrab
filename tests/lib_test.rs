#[cfg(test)]
mod lib_test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use holy_crab_davide_ai::{MinerRobot};
    use holy_crab_davide_ai::SCAN_DISTANCE;

    #[test]
    fn test_new() {
        let miner_robot = MinerRobot::new();

        // Assert
        assert_eq!(miner_robot.name,"The default miner");
        assert_eq!(miner_robot.rocks_collected,0);
        assert_eq!(miner_robot.scan_distance,SCAN_DISTANCE);
        assert_eq!(miner_robot.world_scanned,false);
    }
    #[test]
    fn test_new_name() {
        let miner_robot = MinerRobot::new_name(String::from("new robot's name"));

        // Assert
        assert_eq!(miner_robot.name,"new robot's name");
        assert_eq!(miner_robot.rocks_collected,0);
        assert_eq!(miner_robot.scan_distance,SCAN_DISTANCE);
        assert_eq!(miner_robot.world_scanned,false);

        let miner_robot = MinerRobot::new_name(String::from(""));

        // Assert name
        assert_eq!(miner_robot.name,"");
    }
    #[test]
    fn test_game_is_over() {
        let mut miner_robot = MinerRobot::new();
        miner_robot.game_is_over();
        assert_eq!(miner_robot.game_over,Rc::new(RefCell::new(true)));
    }
}

