use crate::state;

pub struct QueryBounds {
    pub limit: i64,
    pub offset: i64,
}

pub fn get_limit_offset(req: &tide::Request<state::State>) -> QueryBounds {
    // let cache = req.state().cache.clone();
    let limit = req
        .param("limit")
        .unwrap_or("20")
        .to_string()
        .parse::<i64>()
        .unwrap_or(20);
    let offset = req
        .param("offset")
        .unwrap_or("0")
        .to_string()
        .parse::<i64>()
        .unwrap_or(0);
    QueryBounds { limit, offset }
}