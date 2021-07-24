use sqlx::{PgPool, Pool};
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::Server;

use crate::cache::RedisStore;
use crate::controllers::commands;
use crate::controllers::steps;
use crate::state;

pub fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool = Pool::connect_lazy(&db_url).unwrap();
    db_pool
}

pub async fn make_redis_store() -> RedisStore {
    let connection_info = std::env::var("REDIS_URL").unwrap();
    let store = RedisStore::new(connection_info).unwrap();
    store
}

pub async fn make_server(db_pool: PgPool, cache: RedisStore) -> Server<state::State> {
    let mut app = tide::with_state(state::State { db_pool, cache });

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.at("/v1/commands").get(commands::get_all);
    // app.at("/v1/commands").post(commands::create);
    app.at("/v1/commands/:command_id").get(commands::get);
    // app.at("/v1/commands/:command_id").put(commands::update);
    // app.at("/v1/commands/:command_id").delete(commands::delete);

    app.at("/v1/commands/:command_id/steps").get(steps::get_all);
    // app.at("/v1/commands/:command_id/steps").post(steps::create);
    app.at("/v1/commands/:command_id/steps/:step_id")
        .get(steps::get);
    // app.at("/v1/commands/:command_id/steps/:step_id").put(steps::update);
    // app.at("/v1/commands/:command_id/steps/:step_id").delete(steps::delete);

    app
}
