use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Undefined = 0,
    Yearly = 1,
    HalfYearly = 2,
    QuadriMonthly = 3,
    Quarterly = 4,
    BiMonthly = 6,
    Monthly = 12,
}

impl From<i32> for Frequency {
    fn from(value: i32) -> Self {
        match value {
            1 => Frequency::Yearly,
            2 => Frequency::HalfYearly,
            3 => Frequency::QuadriMonthly,
            4 => Frequency::Quarterly,
            6 => Frequency::BiMonthly,
            12 => Frequency::Monthly,
            _ => Frequency::Undefined,
        }
    }
}

impl TryFrom<&Robj> for Frequency {
    type Error = Error;

    fn try_from(value: &Robj) -> Result<Self, Self::Error> {
        let x: i32 = value.try_into()?;
        Ok(Frequency::from(x))
    }
}