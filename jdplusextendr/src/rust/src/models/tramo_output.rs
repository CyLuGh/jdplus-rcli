use crate::jdplus_main_ws_v1::TramoOutputDto;
use crate::models::parameter::Parameter;
use crate::models::processing_logs::ProcessingLogs;
use crate::models::reg_arima_model::RegArimaModel;
use crate::models::specs::TramoSpec;
use chrono::NaiveDate;
use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
#[derive(Debug)]
pub struct TramoOutput {
    pub result: RegArimaModel,
    pub estimation_spec: TramoSpec,
    pub result_spec: Option<TramoSpec>,
    pub log: Option<ProcessingLogs>,
}

impl TramoOutput {
    pub fn get_outliers(&self) -> Vec<OutlierReport> {
        let series_values = self.result.description.series.get_date_values();

        let outliers = match &self.result_spec {
            Some(rs) => rs
                .regression
                .outliers
                .iter()
                .filter_map(|o| {
                    series_values.get(&o.position).map(|val| OutlierReport {
                        period: o.position,
                        value: val.clone(),
                        code: o.code.clone(),
                        coefficient: o.coefficient.clone(),
                        metadata: o.metadata.clone(),
                    })
                })
                .collect::<Vec<OutlierReport>>(),
            None => Vec::<OutlierReport>::new(),
        };

        outliers
    }
}

impl TryFrom<TramoOutputDto> for TramoOutput {
    type Error = String;

    fn try_from(dto: TramoOutputDto) -> Result<Self, Self::Error> {
        Ok(Self {
            result: RegArimaModel::try_from(dto.result.unwrap())?,
            estimation_spec: TramoSpec::try_from(dto.estimation_spec.unwrap())?,
            result_spec: dto.result_spec.map(TramoSpec::try_from).transpose()?,
            log: dto.log.map(ProcessingLogs::try_from).transpose()?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct OutlierReport {
    pub period: NaiveDate,
    pub value: f64,
    pub code: String,
    pub coefficient: Parameter,
    pub metadata: HashMap<String, String>,
}
