use tide::convert::json;
use tide::{Body, Response, StatusCode};

use crate::db::commands;
use crate::state;

pub async fn get(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    // let cache = req.state().cache.clone();
    let command_id_str = req.param("command_id")?;
    let command_id = command_id_str.to_string().parse::<i64>()?;
    let maybe_command = commands::get(command_id, &db_pool).await;
    match maybe_command {
        Ok(command) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&command)?);
            Ok(res)
        }
        Err(error) => {
            let mut res = Response::new(StatusCode::NotFound);
            let err_json = json!({ "data": {}, "error": format!("Error getting command {}, err: {:?}", command_id, error)});
            res.set_body(Body::from_json(&err_json)?);
            Ok(res)
        }
    }
}

pub async fn get_all(req: tide::Request<state::State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    // let cache = req.state().cache.clone();
    let maybe_commands = commands::get_all(&db_pool).await;
    match maybe_commands {
        Ok(commands) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&commands)?);
            Ok(res)
        }
        Err(error) => {
            let mut res = Response::new(StatusCode::NotFound);
            let err_json =
                json!({ "data": {}, "error": format!("Error getting commands, err: {:?}", error)});
            res.set_body(Body::from_json(&err_json)?);
            Ok(res)
        }
    }
}
