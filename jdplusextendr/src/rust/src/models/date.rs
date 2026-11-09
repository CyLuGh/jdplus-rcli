use chrono::NaiveDate;
use crate::jdplus_main_ws_v1::DateDto;

impl From<DateDto> for NaiveDate {
    fn from(date: DateDto) -> Self {
        Self::from_ymd_opt(date.year, date.month as u32, date.day as u32).unwrap_or_default()
    }
}