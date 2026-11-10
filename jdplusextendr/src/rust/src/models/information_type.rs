#[derive(Debug)]
pub enum InformationType
{
    Info = 0,
    Warning = 1,
    Error = 2
}

impl From<i32> for InformationType {
    fn from(item: i32) -> Self {
        match item {
            0 => InformationType::Info,
            1 => InformationType::Warning,
            2 => InformationType::Error,
            _ => unreachable!()
        }
    }
}