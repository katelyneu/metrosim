/*****************************************************************************/
use crate::metro::passenger_queue::PassengerQueue;
use crate::utils::file_ops::read_lines;
use super::passenger::Passenger;
use std::{
    collections::HashMap,
    fmt,
    fs::OpenOptions,
    io::Write,
};
/*****************************************************************************/

/*****************************************************************************/
pub struct Simulator {
    current_station: usize,
    map: HashMap<usize, String>,
    output_file: String,
    passenger_map: Vec<PassengerQueue>,
    train: Vec<PassengerQueue>,
}
/*****************************************************************************/

/*****************************************************************************/
impl Simulator {
    pub fn new(stations: &str, output_file: &str) -> Simulator {
        let mut map: HashMap<usize, String> = HashMap::new();
        let mut passenger_map: Vec<PassengerQueue> = Vec::new();
        let mut train: Vec<PassengerQueue> = Vec::new();

        // populate the map, passenger_map, and train
        if let Ok(lines) = read_lines(stations) {
            let mut station_number = 0;
            for line in lines {
                if let Ok(station_name) = line {
                    map.insert(station_number, station_name);
                }
                passenger_map.push(PassengerQueue::new());
                train.push(PassengerQueue::new());
                station_number += 1
            }
        }

        Simulator { 
            current_station: 0,
            map: map, 
            output_file: output_file.to_string(),
            passenger_map: passenger_map,
            train: train,
        }
    }

    pub fn add_passenger(&mut self, arrival: u32, departure: u32) {
        let passenger = Passenger::new(arrival, departure);
        let passenger_queue = &mut self.passenger_map[arrival as usize];
        passenger_queue.enqueue(passenger);
    }
    
    pub fn move_train(&mut self) {
        let current_station = &mut self.current_station;
        let passenger_queue = &mut self.passenger_map[*current_station];
        let train = &mut self.train;
    
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.output_file)
            .unwrap();

        // exiting passengers exit train
        while let Some(p) = train[*current_station].dequeue() {
            // write this to output file instead
            let station = match self.map.get(current_station) {
                Some(station) => station,
                None => panic!("error finding station {}", current_station),
            };
            write!(
                file, 
                "Passenger {} left the train at station {}\n", 
                p.id, 
                station
            );
        }

        // entering passengers enter train
        while let Some(p) = passenger_queue.dequeue() {
            train[p.departure as usize].enqueue(p);
        }

        self.current_station = (self.current_station + 1 as usize) % self.map.len();
    }
}
/*****************************************************************************/

/*****************************************************************************/
impl fmt::Display for Simulator {
    fn fmt(&self, output_stream: &mut fmt::Formatter) -> fmt::Result {
        let mut passengers_on_train = String::new();

        for car in &self.train {
            passengers_on_train = passengers_on_train + format!("{}", car).as_str();
        }

        write!(
            output_stream,
            "Passengers on train: {{{}}}\n",
            passengers_on_train,
        )?;

        let mut station_index = 0;
        for station in &self.passenger_map {
            let train_fill: &str;
            if self.current_station == station_index {
                train_fill = "TRAIN:";
            }
            else {
                train_fill = " ";
            }
            write!(
                output_stream,
                "{:<7}[{}] {} {{{}}}\n",
                train_fill,
                station_index,
                self.map[&(station_index as usize)],
                station,
            )?;
            station_index += 1;
        }
        write!(output_stream, "")
    }
}
/*****************************************************************************/

/*****************************************************************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sim() {
        let simulator = Simulator::new(
            "src/resources/test_stations.txt",
            "src/resources/output.txt",
        );
        let expected_sim = "\
Passengers on train: {}
TRAIN: [0] station_0 {}
       [1] station_1 {}
       [2] station_2 {}\n".to_string();
        let recieved_sim = format!("{}", simulator);
        print!("expected: \n {}", expected_sim);
        print!("recieved: \n {}", recieved_sim);
        assert_eq!(expected_sim, recieved_sim);
    }

    #[test]
    fn test_add_passenger() {
        let mut simulator = Simulator::new(
            "src/resources/test_stations.txt",
            "src/resources/output.txt",
        );
        simulator.add_passenger(0, 2);
        let p = match simulator.passenger_map[0 as usize].front() {
            Some(p) => p,
            None => panic!("no passenger found on train with departure station 1")
        };
        let expected_sim = format!("\
Passengers on train: {{}}
TRAIN: [0] station_0 {{[{}, 0->2]}}
       [1] station_1 {{}}
       [2] station_2 {{}}\n",
            p.id
        );
        let recieved_sim = format!("{}", simulator);
        print!("expected: \n {}", expected_sim);
        print!("recieved: \n {}", recieved_sim);
        assert_eq!(expected_sim, recieved_sim);
    }

    #[test]
    fn test_move_train() {
        let mut simulator = Simulator::new(
            "src/resources/test_stations.txt",
            "src/resources/output.txt",
        );
        simulator.add_passenger(0, 2);
        simulator.move_train();
        let p = match simulator.train[2 as usize].front() {
            Some(p) => p,
            None => panic!("no passenger found on train with departure station 1")
        };
        let expected_sim = format!("\
Passengers on train: {{[{}, 0->2]}}
       [0] station_0 {{}}
TRAIN: [1] station_1 {{}}
       [2] station_2 {{}}\n",
            p.id
        );
        let recieved_sim = format!("{}", simulator);
        print!("expected: \n {}", expected_sim);
        print!("recieved: \n {}", recieved_sim);
        assert_eq!(expected_sim, recieved_sim);
    }

    #[test]
    fn test_move_train_and_leave() {
        let mut simulator = Simulator::new(
            "src/resources/test_stations.txt",
            "src/resources/output.txt",
        );
        simulator.add_passenger(0, 1);
        simulator.move_train();
        simulator.move_train();
        let expected_sim = "\
Passengers on train: {}
       [0] station_0 {}
       [1] station_1 {}
TRAIN: [2] station_2 {}\n".to_string();
        let recieved_sim = format!("{}", simulator);
        print!("expected: \n {}", expected_sim);
        print!("recieved: \n {}", recieved_sim);
        assert_eq!(expected_sim, recieved_sim);
    }

    #[test]
    fn test_move_train_around_track() {
        let mut simulator = Simulator::new(
            "src/resources/test_stations.txt",
            "src/resources/output.txt",
        );
        simulator.move_train();
        simulator.move_train();
        simulator.move_train();
        let expected_sim = "\
Passengers on train: {}
TRAIN: [0] station_0 {}
       [1] station_1 {}
       [2] station_2 {}\n".to_string();
        let recieved_sim = format!("{}", simulator);
        print!("expected: \n {}", expected_sim);
        print!("recieved: \n {}", recieved_sim);
        assert_eq!(expected_sim, recieved_sim);
    }
}
/*****************************************************************************/