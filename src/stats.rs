use std::fmt;

#[derive(Clone)]
pub struct Measurement{
    pub value: u32,
    pub timestamp: std::time::Instant,
}

pub struct Statistics{
    measurements: std::collections::VecDeque<Measurement>,
}

impl Statistics{
    pub fn new()->Statistics{
        Statistics{
            measurements: std::collections::VecDeque::new()
        }
    }

    pub fn add_value(&mut self, added_value: u32){
        if self.measurements.len() == 10{
            self.measurements.pop_front();
        }

        let value = if self.measurements.is_empty() {
            added_value  
        }
        else {
            self.measurements.back().unwrap().value + added_value
        };

        let m = Measurement{value: value,
            timestamp: std::time::Instant::now()};

        self.measurements.push_back(m);
    }
    
    pub fn get_mean_value(&self) -> Option<u32>{
        if self.measurements.len() < 2 {
            return None;
        }

        let b = self.measurements.back().unwrap();
        let f = self.measurements.front().unwrap();

        let s = b.timestamp.duration_since(f.timestamp);
        let value: u128 = ( b.value - f.value ) as u128;
        let result = (1_000 * value  / s.as_millis()) as u32;
        Some( result )
    }
}

pub struct Stats{
    pub sending: Statistics,
    pub receiving: Statistics,
}

impl fmt::Debug for Stats{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        f.debug_struct("Stats")
            .field("sending", &self.sending.get_mean_value())
            .field("receiving", &self.receiving.get_mean_value())
            .finish()
    }
}

impl fmt::Display for Stats{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        f.debug_struct("Stats")
            .field("sending", &self.sending.get_mean_value())
            .field("receiving", &self.receiving.get_mean_value())
            .finish()
    }
}