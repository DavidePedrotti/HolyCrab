pub mod debug {
    use crate::MinerRobot;
    use robotics_lib::world::World;
    use robotics_lib::world::tile::{Content, TileType};
    use robotics_lib::interface::robot_map;
    use colored::Colorize;
    impl MinerRobot {
        /// Prints all the discovered tiles content
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        pub fn print_discovered_tiles_content(&self, world: &World) {
            print!("- ");
            for (i, _row) in robot_map(world).unwrap().iter().enumerate() {
                print!("{} ", i % 10);
            }
            println!();
            for (i, row) in robot_map(world).unwrap().iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    if j == 0 {
                        print!("{} ", i % 10);
                    }
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("! ");
                    } else {
                        match tile {
                            None => {
                                print!("- ")
                            }
                            Some(t) => {
                                Self::print_content(t.clone().content)
                            }
                        };
                    }
                }
                println!();
            }
            println!();
        }
        /// Prints all the discovered tiles tile_type
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        pub fn print_discovered_tiles_tile_type(&self, world: &World) {
            print!("- ");
            for (i, _row) in robot_map(world).unwrap().iter().enumerate() {
                print!("{} ", i % 10);
            }
            println!();
            for (i, row) in robot_map(world).unwrap().iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    if j == 0 {
                        print!("{} ", i % 10);
                    }
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("! ");
                    } else {
                        match tile {
                            None => {
                                print!("- ")
                            }
                            Some(t) => {
                                Self::print_tile_type(t.clone().tile_type)
                            }
                        };
                    }
                }
                println!();
            }
            println!();
        }
        /// Prints the respective letter to the content given
        ///
        /// # Arguments
        ///
        /// * `content` - the content that is getting printed
        fn print_content(content: Content) {
            print!("{}", match content {
                Content::Bank(_) => { "A " }
                Content::Bin(_) => { "I " }
                Content::Building => { "B " }
                Content::Bush(_) => { "H " }
                Content::Crate(_) => { "C " }
                Content::Coin(_) => { "O " }
                Content::Fire => { "F " }
                Content::Fish(_) => { "P " }
                Content::Garbage(_) => { "G " }
                Content::JollyBlock(_) => { "J " }
                Content::Market(_) => { "M " }
                Content::Rock(_) => { "R " }
                Content::Scarecrow => { "S " }
                Content::Tree(_) => { "T " }
                Content::Water(_) => { "W " }
                Content::None => { "+ " }
            })
        }
        /// Prints the respective letter to the tiletype given
        ///
        /// # Arguments
        ///
        /// * `tile_type` - the tiletype that is getting printed
        fn print_tile_type(tile_type: TileType) {
            print!("{}", match tile_type {
                TileType::DeepWater => { "D ".blue() }
                TileType::Grass => { "G ".green() }
                TileType::Hill => { "H ".green() }
                TileType::Lava => { "L ".green() }
                TileType::Mountain => { "M ".green() }
                TileType::Sand => { "S ".green() }
                TileType::ShallowWater => { "o ".green() }
                TileType::Snow => { "N ".green() }
                TileType::Street => { "R ".red() }
                TileType::Teleport(_) => { "T ".green() }
                TileType::Wall => { "W ".green() }
            })
        }
    }
}