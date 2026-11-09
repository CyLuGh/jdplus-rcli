pub enum TradingDays {
    TDNone = 0,
    /// MON-FRI + SAT-SUN
    TD2 = 1,
    /// MON-SAT + SUN
    TD2C = 2,
    /// MON-FRI + SAT + SUN
    TD3 = 3,
    /// MON-THU + FRI-SAT + SUN
    TD3C = 4,
    /// MON-THU + FRI + SAT + SUN
    TD4 = 5,
    TD7 = 6,
}

impl From<i32> for TradingDays {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::TDNone,
            1 => Self::TD2,
            2 => Self::TD2C,
            3 => Self::TD3,
            4 => Self::TD3C,
            5 => Self::TD4,
            6 => Self::TD7,
            _ => unreachable!(),
        }
    }
}

pub enum TradingDaysTest {
    No = 0,
    SeparateT = 1,
    JointF = 2,
}

impl From<i32> for TradingDaysTest {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::No,
            1 => Self::SeparateT,
            2 => Self::JointF,
            _ => unreachable!(),
        }
    }
}

pub enum AutomaticTradingDays {
    No = 0,
    FTest = 1,
    Wald = 2,
    Aic = 3,
    Bic = 4,
}

impl From<i32> for AutomaticTradingDays {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::No,
            1 => Self::FTest,
            2 => Self::Wald,
            3 => Self::Aic,
            4 => Self::Bic,
            _ => unreachable!(),
        }
    }
}
