use crate::models::tsdata::TsData;

use std::collections::HashMap;

use crate::jdplus_main_ws_v1::{
    TsDataDto,
    TsDto,
    TsMonikerDto};
use crate::models::tsmoniker::TsMoniker;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone, PartialEq)]
struct Ts {
    pub name: String,
    pub moniker: TsMoniker,
    pub data: TsData,
    pub metadata: HashMap<String, String>
}

impl Default for Ts {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            moniker: TsMoniker::default(),
            data: TsData::default(),
            metadata: HashMap::new(),
        }
    }
}

impl TryFrom<TsDto> for Ts {
    type Error = String;

    fn try_from(dto: TsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            moniker: dto.moniker
                .ok_or("moniker missing")?
                .into(),
            data: dto.data
                .ok_or("data missing")?
                .try_into()?,
            metadata: dto.metadata,
        })
    }
}

impl From<Ts> for TsDto {
    fn from(ts: Ts) -> Self {
        Self {
            name: ts.name,
            moniker: Some(TsMonikerDto::from(ts.moniker)),
            data: Some(TsDataDto::from(ts.data)),
            metadata: ts.metadata
        }
    }
}