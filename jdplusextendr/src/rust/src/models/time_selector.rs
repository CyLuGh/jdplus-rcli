use crate::jdplus_main_ws_v1::TimeSelectorDto;
use chrono::NaiveDate;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug)]
pub enum SelectionType {
    SpanAll = 0,
    SpanFrom = 1,
    SpanTo = 2,
    SpanBetween = 3,
    SpanLast = 4,
    SpanFirst = 5,
    SpanExcluding = 6,
    SpanNone = 7,
}

impl From<i32> for SelectionType {
    fn from(value: i32) -> Self {
        match value {
            0 => SelectionType::SpanAll,
            1 => SelectionType::SpanFrom,
            2 => SelectionType::SpanTo,
            3 => SelectionType::SpanBetween,
            4 => SelectionType::SpanLast,
            5 => SelectionType::SpanFirst,
            6 => SelectionType::SpanExcluding,
            7 => SelectionType::SpanNone,
            _ => unreachable!(),
        }
    }
}

#[extendr]
#[derive(Debug)]
pub struct TimeSelector {
    pub selection_type: SelectionType,
    pub n0: i32,
    pub n1: i32,
    pub d0: Option<NaiveDate>,
    pub d1: Option<NaiveDate>,
}
impl TryFrom<TimeSelectorDto> for TimeSelector {
    type Error = String;

    fn try_from(dto: TimeSelectorDto) -> Result<Self, Self::Error> {
        Ok(Self {
            selection_type: SelectionType::from(dto.r#type),
            n0: dto.n0,
            n1: dto.n1,
            d0: match dto.d0 {
                Some(date) => Some(NaiveDate::from(date)),
                None => None,
            },
            d1: match dto.d1 {
                Some(date) => Some(NaiveDate::from(date)),
                None => None,
            },
        })
    }
}
