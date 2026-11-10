use crate::jdplus_main_ws_v1::LikelihoodStatisticsDto;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug)]
pub struct LikelihoodStatistics {
    pub n_obs: i32,
    pub n_effective_obs: i32,
    pub n_params: i32,
    pub degrees_of_freedom: i32,
    pub log_likelihood: f64,
    pub adjusted_log_likelihood: f64,
    pub aic: f64,
    pub aicc: f64,
    pub bic: f64,
    pub bicc: f64,
    pub bic2: f64,
    pub hannan_quinn: f64,
    pub ssq: f64,
}

impl From<LikelihoodStatisticsDto> for LikelihoodStatistics {
    fn from(dto: LikelihoodStatisticsDto) -> Self {
        Self {
            n_obs: dto.nobs,
            n_effective_obs: dto.neffectiveobs,
            n_params: dto.nparams,
            degrees_of_freedom: dto.degrees_of_freedom,
            log_likelihood: dto.log_likelihood,
            adjusted_log_likelihood: dto.adjusted_log_likelihood,
            aic: dto.aic,
            aicc: dto.aicc,
            bic: dto.bic,
            bicc: dto.bicc,
            bic2: dto.bic2,
            hannan_quinn: dto.hannan_quinn,
            ssq: dto.ssq,
        }
    }
}
