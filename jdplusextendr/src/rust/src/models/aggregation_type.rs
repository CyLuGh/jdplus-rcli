use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationType {
    None = 0,
    Sum = 1,
    Average = 2,
    First = 3,
    Last = 4,
    Max = 5,
    Min = 6,
}

impl From<i32> for AggregationType {
    fn from(value: i32) -> Self {
        match value {
            0 => AggregationType::None,
            1 => AggregationType::Sum,
            2 => AggregationType::Average,
            3 => AggregationType::First,
            4 => AggregationType::Last,
            5 => AggregationType::Max,
            6 => AggregationType::Min,
            _ => AggregationType::None,
        }
    }
}


impl TryFrom<&Robj> for AggregationType {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        let x: i32 = value.try_into()?;
        Ok(AggregationType::from(x))
    }
}
