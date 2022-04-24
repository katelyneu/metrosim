/*****************************************************************************/
use std::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
};
/*****************************************************************************/

/*****************************************************************************/
static PASSENGER_COUNTER: AtomicU32 = AtomicU32::new(1);
/*****************************************************************************/

/*****************************************************************************/
#[derive(Clone, Copy)]
pub struct Passenger {
    pub id: u32,
    pub arrival: u32,
    pub departure: u32,
}
/*****************************************************************************/

/*****************************************************************************/
impl Passenger {
    pub fn new(arrival: u32, departure: u32) -> Passenger {
        Passenger{
            id: PASSENGER_COUNTER.fetch_add(1, Ordering::SeqCst), 
            arrival: arrival, 
            departure: departure
        }
    }
}

impl fmt::Display for Passenger {
    fn fmt(&self, output_stream: &mut fmt::Formatter) -> fmt::Result {
        write!(
            output_stream, 
            "[{}, {}->{}]", 
            self.id, 
            self.arrival, 
            self.departure,
        )
    }
}
/*****************************************************************************/

/*****************************************************************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let passenger = Passenger::new(0, 1);
        let output_string = format!("{}", passenger);
        let expected_string = format!(
            "[{}, {}->{}]",
            passenger.id,
            passenger.arrival,
            passenger.departure,
        );
        assert_eq!(output_string, expected_string);
    }
}
/*****************************************************************************/