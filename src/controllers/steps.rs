use tide::convert::json;
use tide::{Body, Response, StatusCode};

use crate::db::steps;
use crate::state;
use crate::controllers::helpers;

pub async fn get(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    // let cache = req.state().cache.clone();
    let command_id = req.param("command_id")?.to_string().parse::<i64>()?;
    let step_id = req.param("step_id")?.to_string().parse::<i64>()?;
    let maybe_step = steps::get(command_id, step_id, &db_pool).await;
    match maybe_step {
        Ok(step) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&step)?);
            Ok(res)
        }
        Err(error) => {
            let mut res = Response::new(StatusCode::NotFound);
            let err_json = json!({ "data": {}, "error": format!("Error getting step {}, err: {:?}", step_id, error)});
            res.set_body(Body::from_json(&err_json)?);
            Ok(res)
        }
    }
}

pub async fn get_all(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    // let cache = req.state().cache.clone();
    let command_id = req.param("command_id")?.to_string().parse::<i64>()?;
    let bounds: helpers::QueryBounds = helpers::get_limit_offset(&req);
    let maybe_steps = steps::get_all(&db_pool, command_id, bounds.limit, bounds.offset).await;
    match maybe_steps {
        Ok(steps) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&steps)?);
            Ok(res)
        }
        Err(error) => {
            let mut res = Response::new(StatusCode::NotFound);
            let err_json =
                json!({ "data": {}, "error": format!("Error getting steps, err: {:?}", error)});
            res.set_body(Body::from_json(&err_json)?);
            Ok(res)
        }
    }
}
