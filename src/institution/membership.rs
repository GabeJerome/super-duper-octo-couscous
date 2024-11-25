pub enum Membership {
    Standard,
    Premium,
    Gold,
}

impl Membership {
    pub fn fee(&self) -> f64 {
        match self {
            Membership::Standard => 10.00,
            Membership::Premium => 20.00,
            Membership::Gold => 50.00,
        }
    }
}
