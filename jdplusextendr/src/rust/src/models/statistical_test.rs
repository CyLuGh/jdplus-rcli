use crate::jdplus_main_ws_v1::StatisticalTestDto;
use extendr_api::prelude::*;

#[extendr]
pub struct StatisticalTest {
    pub value: f64,
    pub p_value: f64,
    pub description: String,
}

impl From<StatisticalTestDto> for StatisticalTest {
    fn from(dto: StatisticalTestDto) -> Self {
        Self {
            value: dto.value,
            p_value: dto.p_value,
            description: dto.description,
        }
    }
}
