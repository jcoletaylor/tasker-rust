use serde::Deserialize;

pub fn copy_string_or_none(r: &Option<String>) -> Option<String> {
    match r {
        Some(r) => Some(String::from(r)),
        None => None,
    }
}

#[derive(Debug, Deserialize)]
pub struct LimitOffsetBounds {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
