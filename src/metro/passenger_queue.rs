/*****************************************************************************/
use crate::metro::passenger::Passenger;
use std::{collections::VecDeque, fmt};
/*****************************************************************************/

/*****************************************************************************/
#[derive(Clone)]
pub struct PassengerQueue {
    queue: VecDeque<Passenger>,
}
/*****************************************************************************/

/*****************************************************************************/
impl PassengerQueue {
    pub fn new() -> PassengerQueue {
        PassengerQueue{
            queue: VecDeque::new(),
        }
    }

    pub fn front(&self) -> Option<&Passenger> {
        self.queue.front()
    }

    pub fn enqueue(&mut self, passenger: Passenger) {
        self.queue.push_back(passenger)
    }

    pub fn dequeue(&mut self) -> Option<Passenger> {
        self.queue.pop_front()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}

impl fmt::Display for PassengerQueue {
    fn fmt(&self, output_stream: &mut fmt::Formatter) -> fmt::Result {
        let mut output_str = String::new();
        for p in self.queue.clone() {
            output_str = output_str + format!("{}", p).as_str();
        }
        write!(
            output_stream, 
            "{}", 
            output_str
        )
    }
}
/*****************************************************************************/

/*****************************************************************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let passenger = Passenger::new(0, 1);

        let mut passengers = PassengerQueue::new();
        passengers.enqueue(passenger);
        assert_eq!(passengers.size(), 1);
    }

    #[test]
    fn test_front() {
        let passenger = Passenger::new(0, 1);

        let mut passengers = PassengerQueue::new();
        passengers.enqueue(passenger);
        match passengers.front() {
            Some(p) => {
                let expected_passenger = format!("{}", passenger);
                let recieved_passenger = format!("{}", p);
                assert_eq!(expected_passenger, recieved_passenger);
            },
            None => {
                panic!("call to get front of passenger call failed")
            }
        }
    }

    #[test]
    fn test_dequeue() {
        let passenger = Passenger::new(0, 1);

        let mut passengers = PassengerQueue::new();
        passengers.enqueue(passenger);
        passengers.dequeue();
        assert_eq!(passengers.size(), 0);
    }

    #[test]
    fn test_display() {
        let passenger_1 = Passenger::new(0, 1);
        let passenger_2 = Passenger::new(3, 4);
        let passenger_3 = Passenger::new(6, 7);

        let mut passengers = PassengerQueue::new();
        passengers.enqueue(passenger_1);
        passengers.enqueue(passenger_2);
        passengers.enqueue(passenger_3);

        let expected_queue = format!(
            "{}{}{}", 
            passenger_1, 
            passenger_2, 
            passenger_3,
        );
        let recieved_queue = format!("{}", passengers);
        print!("{}", recieved_queue);
        assert_eq!(expected_queue, recieved_queue);
    }
}
/*****************************************************************************/