use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;
use colored::Colorize;

// tools necessary
use robotics_lib::world::world_generator::Generator;
use robotics_lib::runner::Runner;
use worldgen_unwrap::public::WorldgeneratorUnwrap; // world generator

use holy_crab_davide_ai::MinerRobot;

fn main() {
    let mut wg = WorldgeneratorUnwrap::init(false, Some(PathBuf::from("world/bridge.bin")));
    wg.gen();

    let robot = MinerRobot::new();
    let game_over = robot.game_over.clone();

    let run = Runner::new( Box::new(robot), &mut wg);

    match run {
        Ok(mut running) => {
            loop{
                let _ = running.game_tick();
                let game_over_ref = game_over.borrow();
                // if the game_over value is true then the game ends
                if *game_over_ref {
                    break;
                }
                sleep(Duration::from_millis(1000))
            };
        }
        Err(e) => {
            println!("Error in runnable - main");
            println!("{:?}", e);
        }
    }
    println!("{}", "THE GAME ENDED!".green());
}