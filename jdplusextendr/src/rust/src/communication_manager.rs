use crate::jdplus_main_ws_v1::{
    build_ts_data_result_dto, tramo_full_process_result_dto,
    ts_functions_client::TsFunctionsClient, BuildTsDataInputDto, BuildTsDataObsDto, DateDto,
    EmptyDto, ObsGatheringDto, TramoRequestDto, TsDataDto,
};

use crate::jdplus_main_ws_v1::tramo_request_dto::Spec::DefSpec;
use crate::models::tramo_output::TramoOutput;
use crate::models::tsdata::TsData;
use crate::models::{AggregationType, Frequency};
use chrono::NaiveDate;
use std::result::Result::Ok;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq)]
pub struct CommunicationManager {
    url: String,
}

impl CommunicationManager {
    pub fn builder() -> CommunicationManagerBuilder {
        CommunicationManagerBuilder::new()
    }

    pub async fn get_version(&self) -> String {
        let Ok(mut client) = TsFunctionsClient::connect(self.url.clone()).await else {
            return String::from("Unknown version: unable to contact server");
        };

        let Ok(response) = client.get_version(EmptyDto {}).await else {
            return String::from("Unknown version: unable to retrieve version");
        };

        let version = response.into_inner();

        format!("{}.{}.{}", version.major, version.minor, version.revision)
    }

    pub async fn build_ts_data(
        &self,
        data: Vec<(NaiveDate, f64)>,
        aggregation_type: AggregationType,
        frequency: Frequency,
        allow_partial_aggregation: bool,
        include_missing_values: bool,
    ) -> Result<TsData> {
        let input = BuildTsDataInputDto {
            id: String::from(""),
            gathering: Some(ObsGatheringDto {
                aggregation_type: aggregation_type as i32,
                frequency: frequency as i32,
                allow_partial_aggregation,
                include_missing_values,
            }),
            observations: data
                .iter()
                .map(|(date, value)| BuildTsDataObsDto {
                    date: Some(DateDto::from(*date)),
                    value: *value,
                })
                .collect(),
        };

        let client = TsFunctionsClient::connect(self.url.clone()).await;

        let dto = client?
            .build_ts_data(input)
            .await
            .map_err(|e| format!("Error building ts-data: {}", e))?
            .into_inner();

        match dto.result {
            Some(build_ts_data_result_dto::Result::Output(output)) => {
                let series = output.series.ok_or("Missing series")?;
                let data: TsData = series.try_into()?;
                Ok(data)
            }

            Some(build_ts_data_result_dto::Result::Error(error)) => Err(error.message.into()),

            None => Err("Missing result".into()),
        }
    }

    pub async fn get_tramo_full_process(
        &self,
        series: TsData,
        spec: String,
    ) -> Result<TramoOutput> {
        let input = TramoRequestDto {
            n_forecasts: 0,
            series: Some(TsDataDto::from(series)),
            id: String::from("TODO"),
            modelling_context: None,
            spec: Some(DefSpec(spec)),
        };

        let client = TsFunctionsClient::connect(self.url.clone()).await;

        let response = client?
            .tramo_full_process(input)
            .await
            .map_err(|e| e.to_string())?
            .into_inner();

        match response.result {
            Some(tramo_full_process_result_dto::Result::Output(output)) => {
                Ok(TramoOutput::try_from(output)?)
            }

            Some(tramo_full_process_result_dto::Result::Error(error)) => Err(error.message.into()),

            None => Err("Missing result".into()),
        }
    }
}

#[derive(Default)]
pub struct CommunicationManagerBuilder {
    url: String,
}

impl CommunicationManagerBuilder {
    pub fn new() -> CommunicationManagerBuilder {
        CommunicationManagerBuilder {
            url: String::from("http://localhost:4566"),
        }
    }

    pub fn url(mut self, url: String) -> CommunicationManagerBuilder {
        self.url = url;
        self
    }

    pub fn build(self) -> CommunicationManager {
        CommunicationManager { url: self.url }
    }
}
