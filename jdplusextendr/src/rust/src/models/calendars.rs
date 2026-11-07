use crate::models::basic_spec::BasicSpec;
use chrono::NaiveDate;
use extendr_api::prelude::*;

#[extendr]
pub enum CalendarEvent {
    Unspecified = 0,
    /// January, 1
    NewYear = 1,
    /// Shrove Monday (48 days before Easter)
    ShroveMonday = 2,
    /// Shrove Tuesday (47 days before Easter)
    ShroveTuesday = 3,
    /// Ash Wednesday (46 days before Easter)
    AshWednesday = 4,
    /// Easter
    Easter = 5,
    /// Julian Easter.
    JulianEaster = 6,
    /// Last Thursday before Easter
    MaundyThursday = 7,
    /// Last Friday before Easter
    GoodFriday = 8,
    /// First Monday after Easter
    EasterMonday = 9,
    /// Ascension (40 days after Easter)
    Ascension = 10,
    /// Pentecost (50 days after Easter)
    Pentecost = 11,
    /// Corpus Christi (60 days after Easter)
    CorpusChristi = 12,
    /// First Monday after Pentecost (50 days after Easter)
    WhitMonday = 13,
    /// May, 1
    MayDay = 14,
    /// August, 15
    Assumption = 15,
    /// Second Tuesday of September
    LaborDay = 16,
    /// October, 31
    Halloween = 17,
    /// November, 1
    AllSaintsDay = 18,
    /// November, 11
    Armistice = 19,
    /// Fourth Thursday of November
    Thanksgiving = 20,
    /// December, 25
    Christmas = 21,
}

impl From<i32> for CalendarEvent {
    fn from(value: i32) -> Self {
        match value {
            0 => CalendarEvent::Unspecified,
            1 => CalendarEvent::NewYear,
            2 => CalendarEvent::ShroveMonday,
            3 => CalendarEvent::ShroveTuesday,
            4 => CalendarEvent::AshWednesday,
            5 => CalendarEvent::Easter,
            6 => CalendarEvent::JulianEaster,
            7 => CalendarEvent::MaundyThursday,
            8 => CalendarEvent::GoodFriday,
            9 => CalendarEvent::EasterMonday,
            10 => CalendarEvent::Ascension,
            11 => CalendarEvent::Pentecost,
            12 => CalendarEvent::CorpusChristi,
            13 => CalendarEvent::WhitMonday,
            14 => CalendarEvent::MayDay,
            15 => CalendarEvent::Assumption,
            16 => CalendarEvent::LaborDay,
            17 => CalendarEvent::Halloween,
            18 => CalendarEvent::AllSaintsDay,
            19 => CalendarEvent::Armistice,
            20 => CalendarEvent::Thanksgiving,
            21 => CalendarEvent::Christmas,
            _ => unreachable!(),
        }
    }
}

#[extendr]
pub struct ChainedCalendar {
    pub calendar1: String,
    pub calendar2: String,
    pub break_date: NaiveDate,
}

#[extendr]
pub struct Item {
    pub calendar: String,
    pub weight: f64,
}

#[extendr]
pub struct WeightedCalendar {
    pub items: Vec<Item>,
}

#[extendr]
pub struct ValidityPeriod {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[extendr]
pub struct FixedDay {
    pub month: i32,
    pub day: i32,
    pub weight: f64,
    pub validity: ValidityPeriod,
}

#[extendr]
pub struct EasterRelatedDay {
    pub offset: i32,
    pub julian: bool,
    pub weight: f64,
    pub validity: ValidityPeriod,
}

#[extendr]
pub struct FixedWeekDay {
    pub month: i32,
    /// Corresponds to the first, second...
    pub position: i32,
    /// ISO-8601 standard, from 1 (Monday) to 7 (Sunday)
    pub week_day: i32,
    pub weight: f64,
    pub validity: ValidityPeriod,
}

#[extendr]
pub struct PrespecifiedHoliday {
    pub event: CalendarEvent,
    pub offset: i32,
    pub weight: f64,
    pub validity: ValidityPeriod,
}

#[extendr]
pub struct SingleDate{
    pub date: NaiveDate,
    pub weight: f64,
}

#[extendr]
pub struct Calendar {
    pub fixed_days: Vec<FixedDay>,
    pub easter_related_days: Vec<EasterRelatedDay>,
    pub fixed_weeks: Vec<FixedWeekDay>,
    pub prespecified_holidays: Vec<PrespecifiedHoliday>,
    pub single_dates: Vec<SingleDate>,
    pub mean_correction: bool
}

#[extendr]
pub enum CalendarDefinitionOneOf {
    Calendar(Calendar),
    WeightedCalendar(WeightedCalendar),
    ChainedCalendar(ChainedCalendar),
}

#[extendr]
pub struct CalendarDefinition {
    pub definition: CalendarDefinitionOneOf
}
