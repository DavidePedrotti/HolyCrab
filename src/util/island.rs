pub mod island {
    use crate::{MinerRobot};
    use std::fmt::Debug;
    use robotics_lib::world::tile::{Tile, TileType};

    impl MinerRobot {
        /// Verifies if a position is valid or not
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        /// * `row` - the row of the coordinate that we are analyzing
        /// * `col` - the column of the coordinate that we are analyzing
        /// * `visited` - a matrix that keeps track of the visited Coordinates
        ///
        /// # Returns
        ///
        /// A boolean corresponding to whether moving to that tile is possible or not
        fn is_valid_move(&self, map: &Vec<Vec<Tile>>, row: i32, col: i32, visited: &Vec<Vec<bool>>) -> bool {
            let rows = map.len() as i32;
            let cols = map[0].len() as i32;
            row >= 0 && col >= 0 && row < rows && col < cols && map[row as usize][col as usize].tile_type != TileType::DeepWater && !visited[row as usize][col as usize]
        }

        /// Implementation of the Depth-First Search algorithm
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        /// * `row` - the row of the coordinate that we are analyzing
        /// * `col` - the column of the coordinate that we are analyzing
        /// * `visited` - a matrix that keeps track of the visited Coordinates
        /// * `island_cells` - the vector containing the cells that are part of an island
        fn dfs(&self, map: &Vec<Vec<Tile>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>, island_cells: &mut Vec<(i32, i32)>) {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            visited[row as usize][col as usize] = true;
            island_cells.push((row, col));

            for (dr, dc) in directions {
                let new_row = row + dr;
                let new_col = col + dc;
                if self.is_valid_move(map, new_row, new_col, visited) {
                    self.dfs(map, new_row, new_col, visited, island_cells);
                }
            }
        }

        /// Implementation of the Depth-First Search algorithm
        ///
        /// # Arguments
        ///
        /// * `map` - the known world
        ///
        /// # Returns
        ///
        /// A vector of vector of tuples which contains all the islands represented as vectors of (i32,i32)
        pub fn find_island_cells(&self, map: &Vec<Vec<Tile>>) -> Vec<Vec<(i32, i32)>> {
            let rows = map.len() as i32;
            let cols = map[0].len() as i32;
            let mut visited = vec![vec![false; cols as usize]; rows as usize];
            let mut islands_cells = Vec::new();

            for i in 0..rows {
                for j in 0..cols {
                    if map[i as usize][j as usize].tile_type != TileType::DeepWater && !visited[i as usize][j as usize] {
                        let mut island_cells = Vec::new();
                        self.dfs(map, i, j, &mut visited, &mut island_cells);
                        islands_cells.push(island_cells);
                    }
                }
            }

            islands_cells
        }
        /// Finds the closest island to the robot's location
        ///
        /// # Arguments
        ///
        /// * `islands` - A vector containing all the islands represented as vectors of (i32,i32)
        ///
        /// # Returns
        ///
        /// An Option of Vec. The vector represents the closest island to the robot
        pub fn find_closest_island(&mut self, islands:  &mut Vec<Vec<(i32, i32)>>) -> Option<Vec<(i32, i32)>> {
            let (robot_row, robot_col) = self.get_coordinates();
            // looking for the island where the robot is located
            let robot_island = islands
                .iter()
                .find(|island| island.contains(&(robot_row as i32, robot_col as i32)))
                .cloned();

            // keeping all the islands but the robot's
            islands.retain(|island| island != &robot_island.clone().unwrap());

            if let Some(_robot_island) = robot_island {
                // the closest island is calculated as the absolute difference between coordinates
                let closest_island = islands.iter().min_by_key(|island| {
                    island.iter().map(|(row, col)| (row - robot_row as i32).abs() + (col - robot_col as i32).abs()).sum::<i32>()
                });
                closest_island.cloned()
            } else {
                None
            }
        }
        /// Finds the closest tile to the robot
        ///
        /// # Arguments
        ///
        /// * `vec` - the target island
        ///
        /// # Returns
        ///
        /// A tuple containing the coordinate of the closest tile to the robot's coordinates
        pub fn find_closest_tile(&mut self, vec: Option<Vec<(i32, i32)>>) -> (i32,i32) {
            let mut min_cost = usize::MAX;
            let mut closest= (0,0);
            match vec {
                None => {
                    println!("The vector in find_closest_tile was None")
                }
                Some(vector) => {
                    for (row,col) in vector.iter() {
                        let cost = self.get_paving_cost((*row,*col));
                        if cost < min_cost {
                            min_cost = cost;
                            closest = (*row,*col);
                        }
                    }
                }
            }
            closest
        }
    }
}

