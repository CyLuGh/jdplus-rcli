use crate::models::Frequency;
use chrono::{Months, NaiveDate, Datelike};

use crate::jdplus_main_ws_v1::{
    TsPeriodDto, DateDto
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsPeriod {
    pub frequency: Frequency,
    pub year: i32,

    /// Position in the year (0-based)
    pub position: i32,
}

impl Default for TsPeriod {
    fn default() -> Self {
        Self {
            frequency: Frequency::Undefined,
            year: 1900,
            position: 0,
        }
    }
}

impl TsPeriod {
    pub fn monthly_occurrences_per_year(&self) -> i32 {
        match self.frequency {
            Frequency::Undefined => 0,
            _ => 12 / self.frequency as i32,
        }
    }

    pub fn to_date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, 1, 1)?
            .checked_add_months(Months::new(
                (self.position * self.monthly_occurrences_per_year()) as u32,
            ))
    }
}

impl From<TsPeriod> for TsPeriodDto {
    fn from(model: TsPeriod) -> Self {
        Self {
            frequency: model.frequency as i32,
            year: model.year,
            pos: model.position,
        }
    }
}

impl From<TsPeriodDto> for TsPeriod {
    fn from(dto: TsPeriodDto) -> Self {
        Self {
            frequency: Frequency::from(dto.frequency),
            year: dto.year,
            position: dto.pos,
        }
    }
}

impl From<NaiveDate> for DateDto {
    fn from(model:NaiveDate) -> Self {
        Self {
            year: model.year(),
            month: model.month() as i32,
            day: model.day() as i32,
        }
    }
}

