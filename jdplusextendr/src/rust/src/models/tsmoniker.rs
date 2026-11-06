use extendr_api::{extendr, list, List};
use crate::jdplus_main_ws_v1::{
    TsMonikerDto,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsMoniker {
    pub source: String,
    pub id: String,
}

#[extendr]
impl TsMoniker {
    fn get_moniker(&self) -> List {
        list!(
            source = self.source.clone(),
            id = self.id.clone(),
        )
    }
}

impl Default for TsMoniker {
    fn default() -> Self {
        Self {
            source: String::from("Default"),
            id: String::from("Default"),
        }
    }
}

impl From<TsMonikerDto> for TsMoniker {
    fn from(dto: TsMonikerDto) -> Self {
        Self {
            source: dto.source,
            id: dto.id,
        }
    }
}

impl From<TsMoniker> for TsMonikerDto {
    fn from(moniker: TsMoniker) -> Self {
        Self {
            source: moniker.source,
            id: moniker.id,
        }
    }
}