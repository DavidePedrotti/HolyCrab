pub mod debug {
    use crate::MinerRobot;
    use robotics_lib::world::World;
    use robotics_lib::world::tile::Content;
    use robotics_lib::interface::robot_map;
    impl MinerRobot {
        /// Prints all the discovered tiles
        ///
        /// # Arguments
        ///
        /// * `world` - the world
        pub fn print_discovered_tiles(&self, world: &World) {
            for (i, row) in robot_map(world).unwrap().iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        None => {
                            print!("-")
                        }
                        Some(t) => {
                            Self::print_content(t.clone().content)
                        }
                    };
                    if i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col() {
                        print!("!");
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
        fn print_content(content: Content){
            print!("{}", match content {
                Content::Bank(_) => { "A" }
                Content::Bin(_) => { "I" }
                Content::Building => { "B" }
                Content::Bush(_) => { "H" }
                Content::Crate(_) => { "C" }
                Content::Coin(_) => { "O" }
                Content::Fire => { "F" }
                Content::Fish(_) => { "P" }
                Content::Garbage(_) => { "G" }
                Content::JollyBlock(_) => { "J" }
                Content::Market(_) => { "M" }
                Content::Rock(_) => { "R" }
                Content::Scarecrow => { "S" }
                Content::Tree(_) => { "T" }
                Content::Water(_) => { "W" }
                Content::None => { "+" }
            })
        }
    }
}