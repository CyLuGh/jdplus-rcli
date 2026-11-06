use chrono::NaiveDate;
use extendr_api::prelude::*;

#[extendr]
pub struct TsObservation {
    /// For point observations, start should be equal to end. Otherwise, start is included in the period corresponding to the observation.
    pub start: NaiveDate,
    /// For point observations, start should be equal to end. Otherwise, end is excluded from the period corresponding to the observation.
    pub end: NaiveDate,
    pub value: f64,
}

impl TsObservation {
    pub fn builder() -> TsObservationBuilder {
        TsObservationBuilder::default()
    }
}

#[derive(Default)]
pub struct TsObservationBuilder {
    /// For point observations, start should be equal to end. Otherwise, start is included in the period corresponding to the observation.
    pub start: NaiveDate,
    /// For point observations, start should be equal to end. Otherwise, end is excluded from the period corresponding to the observation.
    pub end: Option<NaiveDate>,
    pub value: f64,
}

impl TsObservationBuilder {
    pub fn new() -> TsObservationBuilder {
        TsObservationBuilder::default()
    }

    pub fn start(mut self, start: NaiveDate) -> TsObservationBuilder {
        self.start = start;
        self
    }

    pub fn end(mut self, end: NaiveDate) -> TsObservationBuilder {
        self.end = Some(end);
        self
    }

    pub fn value(mut self, value: f64) -> TsObservationBuilder {
        self.value = value;
        self
    }

    pub fn build(self) -> TsObservation {
        TsObservation {
            start: self.start,
            end: match self.end {
                Some(end) => end,
                None => self.start,
            },
            value: self.value,
        }
    }
}
