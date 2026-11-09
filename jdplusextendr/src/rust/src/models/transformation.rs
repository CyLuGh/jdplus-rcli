use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transformation
{
    Level = 0,
    Log = 1,
    Auto = 10
}

impl From<i32> for Transformation {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Level,
            1 => Self::Log,
            10 => Self::Auto,
            _ => unreachable!(),
        }
    }
}