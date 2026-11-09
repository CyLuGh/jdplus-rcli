use crate::jdplus_main_ws_v1::TramoOutputDto;
use crate::models::processing_logs::ProcessingLogs;
use crate::models::reg_arima_model::RegArimaModel;
use crate::models::specs::TramoSpec;
use extendr_api::prelude::*;

#[extendr]
pub struct TramoOutput {
    pub result: RegArimaModel,
    pub estimation_spec: TramoSpec,
    pub result_spec: Option<TramoSpec>,
    pub log: Option<ProcessingLogs>,
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
