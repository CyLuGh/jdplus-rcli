use extendr_api::prelude::*;

#[extendr]
pub enum AutomaticTradingDay {
    No = 0,
    FTest = 1,
    Wald = 2,
    Aic = 3,
    Bic = 4
}

impl From<i32> for AutomaticTradingDay {
    fn from(value: i32) -> Self {
        match value {
            0 => AutomaticTradingDay::No,
            1 => AutomaticTradingDay::FTest,
            2 => AutomaticTradingDay::Wald,
            3 => AutomaticTradingDay::Aic,
            4 => AutomaticTradingDay::Bic,
            _ => unreachable!()
        }
    }
}