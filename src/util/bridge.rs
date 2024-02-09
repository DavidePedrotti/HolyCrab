pub mod debug {
    // MinerRobot
    use crate::{MinerRobot, RobotState};

    // robotics lib
    use robotics_lib::interface::{go, put};
    use robotics_lib::interface::Direction;
    use robotics_lib::world::tile::{Content, TileType};
    use robotics_lib::world::World;
    use robotics_lib::world::tile::Tile;

    // tools
    use OwnerSheeps_Sound_Tool::functions::put_sounds::{play_sound_rock_in_g_h_s_s, play_sound_rock_in_lava, play_sound_rock_in_water};

    impl MinerRobot {
        /// Method that verifies if the robot has enough rocks to pave the bridge.
        /// If true, it builds the bridge
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        ///
        /// # Notes
        ///
        /// - at first it finds the closest_tile contained in the closest island
        /// - then it calculates the amount of rocks needed to build the bridge
        /// - if the robot has enough rocks it calls the build_along_row_and_col() method and starts building the bridge
        pub fn pave_bridge(&mut self, world: &mut World) {
            let mut islands = self.find_island_cells(&self.get_map(world));
            let closest = self.find_closest_island(&mut islands);
            let discovered_tiles = self.get_map(world);
            let closest_tile = self.find_closest_tile(&discovered_tiles,closest);
            let rocks_to_build_bridge = self.get_paving_cost(&discovered_tiles, closest_tile);
            println!("Closest tile: {:?}", closest_tile);
            println!("Cost: {:?}", rocks_to_build_bridge);
            if self.rocks_collected > rocks_to_build_bridge {
                self.state = RobotState::PavingBridge;
                self.build_along_row_and_col(world, &discovered_tiles, closest_tile.0, closest_tile.1);
                self.rocks_collected -= rocks_to_build_bridge;
            }
        }
        /// Calls the method to build a bridge on both rows and columns
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `map` - the known world
        /// * `row` - the target's row
        /// * `col` - the target's column
        ///
        /// # Notes:
        ///
        /// Given the distance between the robot's row coordinate and the target's it calls the build_to_direction() method.
        /// The same goes for the column.
        fn build_along_row_and_col(&mut self, world: &mut World, map: &Vec<Vec<Tile>>, row: i32, col: i32) {
            let (robot_row,robot_col) = self.get_coordinates();
            let row_distance = robot_row as i32 - row;
            let col_distance = robot_col as i32 - col;
            let mut row_direction = Direction::Up;
            let mut col_direction = Direction::Right;

            if row_distance < 0 {
                row_direction = Direction::Down;
            } else if row_distance > 0 {
                row_direction = Direction::Up;
            }
            if col_distance < 0 {
                col_direction = Direction::Right;
            } else if col_distance > 0 {
                col_direction = Direction::Left;
            }

            // building following rows
            self.build_to_direction(world,map,row_distance,row_direction);

            // building following columns
            self.build_to_direction(world,map,col_distance,col_direction);
        }
        /// Builds a bridge given a direction and a distance
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `map` - the known world
        /// * `distance` - the amount of blocks that are getting paved
        /// * `direction` - the direction that the robot will pave on
        fn build_to_direction(&mut self, world: &mut World, map: &Vec<Vec<Tile>>, distance: i32, direction: Direction) {
            let mut distance_left = distance.abs();

            // iterating through all the tiles that need to connect the robot to the target
            while distance_left > 0 {
                let (mut row,mut col) = self.get_coordinates();

                // calculating the offset in order to find the next tile's coordinates
                let (offset_row,offset_col) = self.direction_to_offset(&direction);
                row = ((row as i32) + offset_row) as usize;
                col = ((col as i32) + offset_col) as usize;

                // calculating the amount of rocks needed to build the bridge
                let quantity = self.get_tile_cost(map[row][col].tile_type);

                // calling put to pave the bridge
                let error = put(self, world, Content::Rock(0), quantity, direction.clone());
                let _ = match error {
                    Ok(_) => {
                        self.play_sound_paving( map[row][col].tile_type);
                        // in case of error a message is returned
                        let msg = format!("Failed to move {:?}", direction);
                        go(self,world,direction.clone()).expect(msg.as_str());
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
        /// Calculates the total cost of building a bridge from the robot's coordinates to the given ones
        ///
        /// # Arguments
        ///
        /// * `map` - the known map
        /// * `(row,col)` - the target's coordinates
        ///
        /// # Returns
        ///
        /// The cost of building a bridge from the robot's coordinates to the target's
        pub fn get_paving_cost(&mut self, map: &Vec<Vec<Tile>>, (row,col): (i32,i32)) -> usize {
            let (robot_row,robot_col) = self.get_coordinates();
            let mut cost = 0;
            let (mut curr_row, mut curr_col) = (robot_row,robot_col);

            while (row,col) != (curr_row as i32,curr_col as i32) {
                // calculating the next row and next column in order to find the cost to pave the next Tile
                let next_row = if (curr_row as i32) < row {
                    curr_row + 1
                } else if (curr_row as i32) > row {
                    curr_row - 1
                } else {
                    curr_row
                };

                let next_col = if (curr_col as i32) < col {
                    curr_col + 1
                } else if (curr_col as i32) > col {
                    curr_col - 1
                } else {
                    curr_col
                };

                cost += self.get_tile_cost(map[next_row][next_col].tile_type);

                curr_row = next_row;
                curr_col = next_col;
            }
            cost
        }
        /// Calls the sound tool based on the tile_type
        fn play_sound_paving(&self, tile_type: TileType) {
            match tile_type {
                TileType::DeepWater => play_sound_rock_in_water(),
                TileType::ShallowWater => play_sound_rock_in_water(),
                TileType::Lava => play_sound_rock_in_lava(),
                _ => play_sound_rock_in_g_h_s_s(),
            }
        }
        /// Returns the cost of paving a certain tile
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the TileType that we are looking at
        ///
        /// # Returns
        ///
        /// The cost of paving a tile with the given TileType
        fn get_tile_cost(&self, tile_type: TileType) -> usize {
            match tile_type {
                TileType::DeepWater => 3,
                TileType::Lava => 3,
                TileType::ShallowWater => 2,
                TileType::Mountain => 0,
                _ => 1
            }
        }
        /// Converts the direction into an offset
        ///
        /// # Arguments
        ///
        /// * `direction` - the direction
        ///
        /// # Returns
        ///
        /// An i32 tuple representing the offset
        fn direction_to_offset(&self, direction: &Direction) -> (i32,i32){
            match direction {
                Direction::Up => (-1,0),
                Direction::Left => (0,-1),
                Direction::Down => (1,0),
                Direction::Right => (0,1),
            }
        }
    }
}