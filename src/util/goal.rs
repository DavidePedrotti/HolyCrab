pub mod goal {
    use bob_lib::tracker::{Goal, GoalType};
    use robotics_lib::world::tile::Content;
    use crate::MinerRobot;

    impl MinerRobot {
        /// Creates a new goal for the robot and adds it to the robot's goal tracker
        ///
        /// # Arguments
        ///
        /// * `goal_type` - the type of goal that will get created
        /// * `content` - the content associated to the goal
        /// * `goal_quantity` - the quantity necessary to complete the goal
        pub fn set_goal_by_content(&mut self, goal_type: GoalType, content: Content, goal_quantity: u32) {
            self.goal_tracker.add_goal(Goal::new(
                Self::get_goal_name_by_content(&content),
                Self::get_goal_description_by_content(&content),
                goal_type,
                Some(content),
                goal_quantity
            ));
        }
        /// Returns the goal's name given a content
        ///
        /// # Arguments
        ///
        /// * `content` - the content associated to the goal
        ///
        /// # Returns
        ///
        /// A string that represents the goal's name
        fn get_goal_name_by_content(content: &Content) -> String {
            String::from(match content {
                Content::Bank(_) => { "Looking for Bank" }
                Content::Bin(_) => { "Looking for Bin" }
                Content::Building => { "Looking for Building" }
                Content::Bush(_) => { "Looking for Bush" }
                Content::Crate(_) => { "Looking for Crate" }
                Content::Coin(_) => { "Looking for Coin" }
                Content::Fire => { "Looking for Fire" }
                Content::Fish(_) => { "Looking for Fish" }
                Content::Garbage(_) => { "Looking for Garbage" }
                Content::JollyBlock(_) => { "Looking for JollyBlock" }
                Content::Market(_) => { "Looking for Market" }
                Content::Rock(_) => { "Looking for Rocks" }
                Content::Scarecrow => { "Looking for Scarecrow" }
                Content::Tree(_) => { "Looking for Tree" }
                Content::Water(_) => { "Looking for Water" }
                Content::None => { "Looking for None" }
            })
        }
        /// Returns the goal's description given a content
        ///
        /// # Arguments
        ///
        /// * `content` - the content associated to the goal
        ///
        /// # Returns
        ///
        /// A string that represents the goal's description
        fn get_goal_description_by_content(content: &Content) -> String {
            String::from(match content {
                Content::Bank(_) => { "Going to the Bank" }
                Content::Bin(_) => { "Using the bin" }
                Content::Building => { "Building?" }
                Content::Bush(_) => { "Hiding in the bush" }
                Content::Crate(_) => { "Crate!" }
                Content::Coin(_) => { "Making money" }
                Content::Fire => { "Looking for fire" }
                Content::Fish(_) => { "Fishing" }
                Content::Garbage(_) => { "Garbage collecting" }
                Content::JollyBlock(_) => { "JollyBlocking" }
                Content::Market(_) => { "Going to the market" }
                Content::Rock(_) => { "Collecting Rocks" }
                Content::Scarecrow => { "Scarecrowing" }
                Content::Tree(_) => { "Looking for a forest" }
                Content::Water(_) => { "In need of Water" }
                Content::None => { "None" }
            })
        }
    }
}