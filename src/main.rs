/*****************************************************************************/
mod metro;
mod utils;

use metro::simulator::Simulator;
use structopt::StructOpt;
use std::io;

use crate::utils::file_ops::read_lines;
/*****************************************************************************/

/*****************************************************************************/

#[derive(StructOpt)]
struct SimSettings {
    stations_file: String,
    output_file: String,
    commands_file: Option<String>,
}

/*****************************************************************************/

/*****************************************************************************/
fn main() {
    let settings = SimSettings::from_args();

    let stations_file = settings.stations_file.as_str();
    let output_file = settings.output_file.as_str();
    
    let mut simulator = Simulator::new(stations_file, output_file);

    match settings.commands_file {
        Some(commands_file) => {
            println!("Command? ");
            if let Ok(lines) = read_lines(commands_file) {
                for line in lines {
                    if let Ok(input) = line {
                        parse_input(input, &mut simulator);
                    }
                }
            }
        },
        None => {
            loop {
                println!("Command? ");
                
                let stdin = io::stdin();
                let mut input = String::new();
                match stdin.read_line(&mut input) {
                    Ok(_) => {},
                    Err(_) => panic!("Error reading in user input"),
                };
                parse_input(input, &mut simulator);
            }
        }
    }
}

fn parse_input(input: String, simulator: &mut Simulator) {
    let cleaned = input
        .as_str()
        .strip_suffix("\n")
        .unwrap_or(input.as_str());

    if cleaned == "m m" {
        simulator.move_train();
        println!("{}", simulator);
    }
    else if cleaned == "m f" {
        println!("Thanks for playing MetroSim. Have a nice day!");
        std::process::exit(0);
    }
    else {
        let split = cleaned.split(" ");
        let cmd: Vec<&str> = split.collect();
        let arrival = cmd[1].parse::<u32>().unwrap();
        let departure = cmd[2].parse::<u32>().unwrap();
        simulator.add_passenger(arrival, departure);
        println!("{}", simulator);
    }
    
}
/*****************************************************************************/