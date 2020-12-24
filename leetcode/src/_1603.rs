struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
    total: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        
        ParkingSystem {
            big,
            medium,
            small,
            total: big + medium + small,
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {

        if self.total == 0 {
            false
        } else {
            match car_type {
                1 => {
                    if self.big == 0 {
                        false
                    } else {
                        self.big -= 1;
                        self.total -= 1;
                        true
                    }
                },
                2 => {
                    if self.medium == 0 {
                        false
                    } else {
                        self.medium -= 1;
                        self.total -= 1;
                        true
                    }
                },
                _ => {
                    if self.small == 0 {
                        false
                    } else {
                        self.small -= 1;
                        self.total -= 1;
                        true
                    }
                }
            }
        }
    }
}