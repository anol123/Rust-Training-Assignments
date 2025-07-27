use crate::model::Seat;
use crate::errors::BookingError;

pub struct Plane {
    seats: Vec<Option<Seat>>,
}

impl Plane {
    pub fn new(total_seats: usize) -> Self {
        Plane {
            seats: vec![None; total_seats],
        }
    }

    pub fn book_seat(&mut self, seat_number: usize, passenger_name: String) -> Result<(), BookingError> {
        if seat_number >= self.seats.len() {
            return Err(BookingError::SeatOutOfRange(seat_number));
        }

        match &self.seats[seat_number] {
            Some(_) => Err(BookingError::SeatAlreadyTaken(seat_number)),
            None => {
                self.seats[seat_number] = Some(Seat {
                    seat_number,
                    passenger_name,
                });
                Ok(())
            }
        }
    }

    pub fn cancel_booking(&mut self, seat_number: usize) -> Result<(), BookingError> {
        if seat_number >= self.seats.len() {
            return Err(BookingError::SeatOutOfRange(seat_number));
        }

        self.seats[seat_number] = None;
        Ok(())
    }

    pub fn auto_book(&mut self, passenger_name: String) -> Result<usize, BookingError> {
        for (i, seat_option) in self.seats.iter_mut().enumerate() {
            if seat_option.is_none() {
                *seat_option = Some(Seat {
                    seat_number: i,
                    passenger_name,
                });
                return Ok(i);
            }
        }

        panic!("Overbooking detected! System will now recover.");
    }

    pub fn display(&self) {
        println!("Seat Layout:");
        for (i, seat) in self.seats.iter().enumerate() {
            if let Some(seat) = seat {
                println!("Seat {}: Booked by {}", i, seat.passenger_name);
            } else {
                println!("Seat {}: Available", i);
            }
        }
    }
}
