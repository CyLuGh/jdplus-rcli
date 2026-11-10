#[derive(Debug)]
pub enum EasterType {
    EasterUnused = 0,
    EasterStandard = 1,
    EasterIncludeEaster = 2,
    EasterIncludeEasterMonday = 3
}

impl From<i32> for EasterType {
    fn from(i: i32) -> Self {
        match i {
            0 => EasterType::EasterUnused,
            1 => EasterType::EasterStandard,
            2 => EasterType::EasterIncludeEaster,
            3 => EasterType::EasterIncludeEasterMonday,
            _ => unreachable!()
        }
    }
}