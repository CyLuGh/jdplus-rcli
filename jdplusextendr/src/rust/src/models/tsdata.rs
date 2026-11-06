use std::collections::HashMap;

use chrono::{Months, NaiveDate};

use crate::models::tsperiod::TsPeriod;
use crate::jdplus_main_ws_v1::TsDataDto;
use crate::jdplus_main_ws_v1::TsPeriodDto;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone, PartialEq)]
pub struct TsData {
    pub start: TsPeriod,
    pub values: Vec<f64>,
}

impl Default for TsData {
    fn default() -> Self {
        Self {
            start: TsPeriod::default(),
            values: Vec::new(),
        }
    }
}

impl TsData {
    pub fn get_date_values(&self) -> HashMap<NaiveDate, f64> {
        let start = self
            .start
            .to_date()
            .expect("invalid start period");

        let months_per_occurrence =
            self.start.monthly_occurrences_per_year() as u32;

        self.values
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let date = start
                    .checked_add_months(Months::new(
                        idx as u32 * months_per_occurrence,
                    ))
                    .expect("date overflow");

                (date, *value)
            })
            .collect()
    }

    pub fn end(&self) -> Option<NaiveDate> {
        let start = self.start.to_date()?;

        start.checked_add_months(Months::new(
            self.values.len() as u32
                * self.start.monthly_occurrences_per_year() as u32,
        ))
    }

    pub fn monthly_occurrences_per_year(&self) -> i32 {
        self.start.monthly_occurrences_per_year()
    }

    pub fn map_data<T, F>(&self, mapper: F, context: &T) -> Self
    where
        F: Fn(f64, &T) -> f64,
    {
        Self {
            start: self.start,
            values: self
                .values
                .iter()
                .map(|v| mapper(*v, context))
                .collect(),
        }
    }
}

impl TryFrom<TsDataDto> for TsData {
    type Error = String;

    fn try_from(dto: TsDataDto) -> Result<Self, Self::Error> {
        Ok(Self {
            start: dto.start
                .ok_or("Missing start period")?
                .into(),
            values: dto.values,
        })
    }
}

impl From<TsData> for TsDataDto {
    fn from(ts: TsData) -> Self {
        Self {
            start: Some(TsPeriodDto::from(ts.start)),
            values: ts.values,
        }
    }
}