pub mod debug {
    use crate::{MinerRobot};
    use sense_and_find_by_rustafariani::Action;
    use std::fmt::Debug;
    use robotics_lib::interface::destroy;
    use robotics_lib::interface::Direction;
    use robotics_lib::interface::go;
    use robotics_lib::world::tile::Content;
    use robotics_lib::world::World;
    use bob_lib::tracker::*;

    impl MinerRobot {
        // function that generates and returns the vector that associates coordinates containing Content, with the cost to reach them
        pub fn get_cost_vector_to_content(&mut self, world: &World, content: Content) -> Vec<(usize,(usize,usize))>{
            let mut cost_vector: Vec<(usize,(usize,usize))> = Vec::new();

            let map = self.get_map_option(world);
            let (x,y) = self.get_coordinates();

            // updating both map and costs
            self.update_lssf_map_cost(world,&map,x,y);

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

        // moves to a target tile and destroys its content
        pub fn move_to_tile_destroy_content(&mut self, world: &mut World, vec: Vec<(usize, (usize, usize))>) {
            let (cost,(x,y));
            if vec.len() > 0 {
                (cost,(x,y)) = vec[0];
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

            for action in &action_vec {
                println!("Action: {:?}", action);
            }

            // printing the action vec:
            for (i,action) in action_vec.iter().enumerate() {
                let direction = match action {
                    Action::North => Direction::Up,
                    Action::East => Direction::Right,
                    Action::South => Direction::Down,
                    Action::West => Direction::Left,
                    Action::Teleport(_x,_y) => Direction::Up,
                };
                // calling the destroy if the robot is facing the tile containting Content
                if i == action_vec.len() - 1 {
                    match destroy(self, world, direction.clone()) {
                        Ok(quantity) => {
                            println!("{:?} quantity of the content has been added to your backpack", quantity);
                            self.update_rock_count();
                            self.goal_tracker.update_manual(GoalType::GetItems,Some(Content::Rock(0)),quantity);
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
    }
}