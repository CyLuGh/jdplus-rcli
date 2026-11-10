use crate::jdplus_main_ws_v1::DoublesDto;
use extendr_api::prelude::*;

#[extendr]
// Rename to DoublesSeq because Doubles already exists in extendr
#[derive(Debug)]
pub struct DoublesSeq {
    pub name: String,
    pub values: Vec<f64>,
}

impl From<DoublesDto> for DoublesSeq {
    fn from(dto: DoublesDto) -> DoublesSeq {
        Self {
            name: dto.name,
            values: dto.values,
        }
    }
}
