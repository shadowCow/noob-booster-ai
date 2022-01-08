pub enum D6 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl D6 {
    pub fn value_of(&self) -> u8 {
        match self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
        }
    }
}

pub struct TwoD6 {
    d1: D6,
    d2: D6,
}

impl TwoD6 {
    pub fn probability(v: &u8) -> f64 {
        match v {
            2 => 1.0/36.0,
            3 => 2.0/36.0,
            4 => 3.0/36.0,
            5 => 4.0/36.0,
            6 => 5.0/36.0,
            7 => 6.0/36.0,
            8 => 5.0/36.0,
            9 => 4.0/36.0,
            10 => 3.0/36.0,
            11 => 2.0/36.0,
            12 => 1.0/36.0,
            _ => 0.0
        }
    }
}