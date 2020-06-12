use std::fmt;


pub struct Stats{
    pub sent: u32,
    pub received: u32,
}

impl fmt::Debug for Stats{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        f.debug_struct("Stats")
            .field("sent", &self.sent)
            .field("received", &self.received)
            .finish()
    }
}

impl fmt::Display for Stats{
    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{
        f.debug_struct("Stats")
            .field("sent", &self.sent)
            .field("received", &self.received)
            .finish()
    }

}

