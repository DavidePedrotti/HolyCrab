use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;

// list of tools available
// use robotics_lib::world::tile::{Content, Tile};
// use bessie::bessie::road_paving_machine; // pave road
// use bob_lib::tracker::*; // goal tracker
// use OwnerSheeps_Sound_Tool::functions::*; // sound
// use pmp_collect_all::CollectAll;
// collect-detect items
// use rust_and_furious_dynamo::dynamo::Dynamo; // recharge energy
// use spyglass::spyglass::{SpyglassBuilder, Spyglass, SpyglassResult}; // tile search

// tools necessary
use robotics_lib::world::world_generator::Generator;
use robotics_lib::runner::Runner;
use worldgen_unwrap::public::WorldgeneratorUnwrap; // world generator

use holy_crab_davide_ai::MinerRobot;

fn main() {
    let mut wg = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("world/smallworld.bin")));
    let (map, position, _environmental_conditions, _score, _score_table) = wg.gen();

    let _world_dim = map.len();
    let (_robot_row, _robot_col) = position;

    let robot = MinerRobot::new();
    let run = Runner::new( Box::new(robot), &mut wg);

    match run {
        Ok(mut running) => {
            loop{
                let _ = running.game_tick();
                sleep(Duration::from_millis(1000))
            };
        }
        Err(e) => {
            println!("Error in runnable - main");
            println!("{:?}", e);
        }
    }

    //println!("{:#?}", map);
}