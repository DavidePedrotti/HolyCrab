pub mod debug {
    use crate::MinerRobot;
    use colored::Colorize;

    // robotics lib
    use std::fmt::Debug;
    use std::usize;
    use robotics_lib::interface::{put};
    use robotics_lib::interface::Direction;
    use robotics_lib::world::tile::Content;
    use robotics_lib::world::World;

    // tools
    use OwnerSheeps_Sound_Tool::functions::put_sounds::play_sound_rock_in_g_h_s_s;

    impl MinerRobot {
        pub fn pave_bridge(&mut self, world: &mut World, coordinate: (i32,i32)) {
            let starting = self.get_coordinates();
            let (row,col) = coordinate;


            println!("{} {:?} to {:?}", "Starting to build from: ".red(), starting, coordinate);
            self.build_along_row_and_col(world, row,col);

            println!("{} {:?} to {:?}", "Starting to build from: ".red(), self.get_coordinates(), coordinate);
        }
        fn build_along_row_and_col(&mut self, world: &mut World, row: i32, col: i32) {
            let (robot_row,robot_col) = self.get_coordinates();
            let row_distance = robot_row as i32 - row;
            let col_distance = robot_col as i32 - col;

            if row_distance < 0 {
                self.build_to_direction(world,row_distance,Direction::Down);
            } else if row_distance > 0 {
                self.build_to_direction(world,row_distance,Direction::Up);
            }
            if col_distance < 0 {
                self.build_to_direction(world,col_distance,Direction::Right);
            } else if col_distance > 0 {
                self.build_to_direction(world,col_distance,Direction::Left);
            }
        }
        fn build_to_direction(&mut self, world: &mut World, distance: i32, direction: Direction) {
            let mut distance_left = distance.abs();
            while distance_left > 0 {
                let error = put(self, world, Content::Rock(0), 1, direction.clone());
                let _ = match error {
                    Ok(_) => {
                        play_sound_rock_in_g_h_s_s();
                        println!("{} {:?}", "Direction: ".blue(), direction);
                        Ok(())
                    },
                    Err(e) => {
                        println!("An error occurred while building the bridge: {:?}", e);
                        Err(())
                    }
                };

                distance_left -= 1;
            }
        }
        pub fn get_paving_cost(&mut self, (row,col): (i32,i32)) -> usize {
            let (robot_row,robot_col) = self.get_coordinates();
            ((robot_row as i32 - row).abs() + (robot_col as i32 - col).abs()) as usize
        }
    }
}