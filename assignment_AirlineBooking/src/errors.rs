use std::fmt;

#[derive(Debug)]
pub enum BookingError {
    SeatAlreadyTaken(usize),
    SeatOutOfRange(usize),
    NoSeatsAvailable,
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BookingError::SeatAlreadyTaken(n) => write!(f, "Seat {} is already taken.", n),
            BookingError::SeatOutOfRange(n) => write!(f, "Seat {} is out of range.", n),
            BookingError::NoSeatsAvailable => write!(f, "No available seats."),
        }
    }
}
