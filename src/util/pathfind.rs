pub mod pathfind {
    use crate::{MinerRobot};
    use sense_and_find_by_rustafariani::Action;
    use robotics_lib::interface::{destroy, go, Direction};
    use robotics_lib::world::{tile::Content, World};
    use bob_lib::tracker::*;
    use OwnerSheeps_Sound_Tool::functions::destroying_sound::play_sound_mining_rock;

    impl MinerRobot {
        /// Generates and returns the vector that associates coordinates containing Content, with the cost to reach them
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `content` - the content that we want to look for
        ///
        /// # Returns
        ///
        /// A vector of tuples:
        /// - the first element represents the cost to reach the tile
        /// - the second element represents the coordinates of the tile
        pub fn get_cost_vector_to_content(&mut self, world: &World, content: Content) -> Vec<(usize,(usize,usize))>{
            let mut cost_vector: Vec<(usize,(usize,usize))> = Vec::new();

            let map = self.get_map_option(world);
            let (x,y) = self.get_coordinates();

            // updating both map and costs
            self.update_lssf_map_and_cost(&map, x, y);

            // getting the vector that contains all the coordinates of tiles that contain a specific content
            let content_vec = self.get_tiles_by_content(world,content);

            // adding both cost and coordinates to the cost vector by iterating over the content vector
            for (row,col) in content_vec {
                match self.lssf.get_cost(row,col){
                    Some(cost) => {
                        if (x,y) != (row,col) {
                            cost_vector.push((cost,(row,col)));
                        }
                    },
                    None => {
                        println!("Unreachable tile")
                    }
                };
            }

            // we order the cost vector so that the first element is the one with the lesser cost
            cost_vector.sort();
            cost_vector
        }

        /// Moves towards a target tile and destroys its content
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        /// * `vec` - the vector of tuples (cost(row,col))
        ///
        /// # Notes
        ///
        /// The robot moves until it reaches the tile near the target and then it destroys the target's content
        pub fn move_to_tile_destroy_content(&mut self, world: &mut World, vec: Vec<(usize, (usize, usize))>) {
            let (_cost,(x,y));
            if vec.len() > 0 {
                (_cost,(x,y)) = vec[0];
            } else {
                println!("The content vector is empty");
                return;
            }

            let coords = self.get_coordinates();

            println!("\nStarted {:?}. Goal: {:?}", coords, (x,y));

            let action_vec = match self.lssf.get_action_vec(x,y){
                Ok(vec) => vec,
                Err(e) => {
                    println!("Error while getting the action vector: {:?}", e);
                    return ();
                }
            };

            // printing the action vec:
            for (i,action) in action_vec.iter().enumerate() {
                let direction = self.action_to_direction(action);
                // calling the destroy if the robot is facing the tile containting Content
                if i == action_vec.len() - 1 {
                    match destroy(self, world, direction.clone()) {
                        Ok(quantity) => {
                            play_sound_mining_rock();
                            println!("{:?} quantity of the content has been added to your backpack", quantity);
                            self.update_rock_count();
                            self.goal_tracker.update_manual(GoalType::GetItems,Some(Content::Rock(0)),quantity);
                            self.goal_tracker.clean_completed_goals();
                        }
                        Err(e) => {
                            println!("Error while destroying content: {:?}", e);
                        }
                    } ;
                }
                let msg = format!("Failed to move {:?}", direction);
                go(self,world,direction.clone()).expect(msg.as_str());
            }
        }
        /// Converts an action into a direction
        ///
        /// # Arguments
        ///
        /// `action`: the action to convert
        ///
        /// # Returns
        ///
        /// The corresponding direction
        fn action_to_direction(&self, action: &Action) -> Direction {
            match action {
                Action::North => Direction::Up,
                Action::East => Direction::Right,
                Action::South => Direction::Down,
                Action::West => Direction::Left,
                Action::Teleport(_x,_y) => Direction::Up,
            }
        }
    }
}