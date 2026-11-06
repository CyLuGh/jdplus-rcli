use crate::jdplus_main_ws_v1::{MatrixDto, TsMatrixDto};
use crate::models::TsPeriod;

pub struct Matrix {
    pub nrows: i32,
    pub ncols: i32,
    /// The number of values should be nrows*ncols. Values are organized by columns (1st column, 2nd column...)
    pub values: Vec<f64>,
}

impl From<MatrixDto> for Matrix {
    fn from(dto: MatrixDto) -> Self {
        Self {
            nrows: dto.nrows,
            ncols: dto.ncols,
            values: dto.values,
        }
    }
}

pub struct TsMatrix {
    pub start: TsPeriod,
    pub matrix: Matrix,
}

impl TryFrom<TsMatrixDto> for TsMatrix {
    type Error = String;

    fn try_from(dto: TsMatrixDto) -> Result<Self, Self::Error> {
        Ok(Self {
            start: dto.start.ok_or("Missing start period")?.into(),
            matrix: dto.values.ok_or("Missing values")?.into(),
        })
    }
}
