use crate::jdplus_main_ws_v1::{
    BuildTsDataInputDto, BuildTsDataObsDto, DateDto, EmptyDto, ObsGatheringDto,
    build_ts_data_result_dto, ts_functions_client::TsFunctionsClient,
};

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