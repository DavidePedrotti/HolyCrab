use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;

// tools necessary
use robotics_lib::world::world_generator::Generator;
use robotics_lib::runner::Runner;
use worldgen_unwrap::public::WorldgeneratorUnwrap; // world generator

use holy_crab_davide_ai::MinerRobot;

fn main() {
    let mut wg = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("world/bridge.bin")));
    wg.gen();

    let robot = MinerRobot::new();
    let run = Runner::new( Box::new(robot), &mut wg);

    match run {
        Ok(mut running) => {
            loop{
                let _ = running.game_tick();
                sleep(Duration::from_millis(2000))
            };
        }
        Err(e) => {
            println!("Error in runnable - main");
            println!("{:?}", e);
        }
    }
}