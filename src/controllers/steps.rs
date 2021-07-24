use tide::convert::json;
use tide::{Body, Response, StatusCode};

use crate::db::steps;
use crate::state;

pub async fn get(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    // let cache = req.state().cache.clone();
    let command_id_str = req.param("command_id")?;
    let command_id = command_id_str.to_string().parse::<i64>()?;
    let step_id_str = req.param("step_id")?;
    let step_id = step_id_str.to_string().parse::<i64>()?;
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
    let command_id_str = req.param("command_id")?;
    let command_id = command_id_str.to_string().parse::<i64>()?;
    let maybe_steps = steps::get_all(command_id, &db_pool).await;
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
