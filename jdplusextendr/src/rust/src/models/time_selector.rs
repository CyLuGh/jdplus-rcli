use chrono::NaiveDate;
use extendr_api::prelude::*;

#[extendr]
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
pub struct TimeSelector {
    pub selection_type: SelectionType,
    pub n0: i32,
    pub n1: i32,
    pub d0: Option<NaiveDate>,
    pub d1: Option<NaiveDate>,
}
