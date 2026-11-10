use crate::jdplus_main_ws_v1::{
    processing_detail_dto, ProcessingDetailDto, ProcessingInformationDto, ProcessingLogsDto,
};
use crate::models::doubles::DoublesSeq;
use crate::models::matrix::Matrix;
use crate::models::statistical_test::StatisticalTest;
use crate::models::tsdata::TsData;
use crate::models::information_type::InformationType;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug)]
pub struct ProcessingLogs {
    pub log: Vec<ProcessingInformation>,
}

impl TryFrom<ProcessingLogsDto> for ProcessingLogs {
    type Error = String;

    fn try_from(value: ProcessingLogsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            log: value
                .log
                .into_iter()
                .map(ProcessingInformation::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct ProcessingInformation {
    pub name: String,
    pub origin: String,
    pub message: String,
    pub information_type: InformationType,
    pub details: ProcessingDetail,
}

impl TryFrom<ProcessingInformationDto> for ProcessingInformation {
    type Error = String;

    fn try_from(dto: ProcessingInformationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            origin: dto.origin,
            message: dto.msg,
            information_type: InformationType::from(dto.r#type),
            details: ProcessingDetail::try_from(dto.details.unwrap())?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct ProcessingDetail {
    pub data: ProcessingDetailOneOf,
}

impl TryFrom<ProcessingDetailDto> for ProcessingDetail {
    type Error = String;
    fn try_from(dto: ProcessingDetailDto) -> Result<Self, Self::Error> {
        match dto.data {
            Some(processing_detail_dto::Data::Ts(ts)) => Ok(Self {
                data: ProcessingDetailOneOf::TsData(TsData::try_from(ts)?),
            }),
            Some(processing_detail_dto::Data::Array(doubles)) => Ok(Self {
                data: ProcessingDetailOneOf::Array(doubles.into()),
            }),
            Some(processing_detail_dto::Data::Matrix(matrix)) => Ok(Self {
                data: ProcessingDetailOneOf::Matrix(matrix.into()),
            }),
            Some(processing_detail_dto::Data::Test(test)) => Ok(Self {
                data: ProcessingDetailOneOf::Test(test.into()),
            }),
            Some(processing_detail_dto::Data::Message(message)) => Ok(Self {
                data: ProcessingDetailOneOf::Message(message),
            }),
            Some(processing_detail_dto::Data::Ivalue(i)) => Ok(Self {
                data: ProcessingDetailOneOf::IValue(i),
            }),
            Some(processing_detail_dto::Data::Dvalue(d)) => Ok(Self {
                data: ProcessingDetailOneOf::DValue(d),
            }),
            _ => unreachable!(),
        }
    }
}

#[extendr]
#[derive(Debug)]
pub enum ProcessingDetailOneOf {
    TsData(TsData),
    Array(DoublesSeq),
    Matrix(Matrix),
    Test(StatisticalTest),
    Message(String),
    IValue(i32),
    DValue(f64),
}
