use extendr_api::prelude::*;

#[extendr]
pub enum LengthOfPeriod {
    None = 0,
    LeapYear = 1,
    LengthOfPeriod = 2,
}

impl From<i32> for LengthOfPeriod {
    fn from(item: i32) -> Self {
        match item {
            0 => LengthOfPeriod::None,
            1 => LengthOfPeriod::LeapYear,
            2 => LengthOfPeriod::LengthOfPeriod,
            _ => unreachable!(),
        }
    }
}
