mod asset;
mod service;
mod staticfiles;

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
        },
        HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
    Json, Router,
};

use ntcore::mapper::Mapper;
use serde::Serialize;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::{self, TraceLayer},
};
use tracing::{debug, error, info, Level};

use crate::config::Config;

#[derive(Clone)]
pub struct WebAppState {
    mapper: Arc<dyn Mapper>,
    config: Arc<Config>,
}

pub async fn serve(mapper: Arc<dyn Mapper>, config: Config) {
    let state = WebAppState {
        mapper,
        config: Arc::new(config.clone()),
    };

    let cors_layer = CorsLayer::new()
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods(Any)
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(|req: &axum::http::Request<axum::body::Body>, _: &_| {
            info!("request: {:?}", req);
        });

    let app = Router::new()
        .merge(service::routes())
        .merge(asset::routes())
        .merge(staticfiles::routes())
        .with_state(state)
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        ))
        .layer(DefaultBodyLimit::disable())
        // https://stackoverflow.com/questions/73498537/axum-router-rejecting-cors-options-preflight-with-405-even-with-corslayer/
        .layer(cors_layer)
        .layer(trace_layer);

    let server_url = format!("{}:{}", "0.0.0.0", &config.server.port);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    info!("server: {}", server_url);

    axum::serve(listener, app).await.unwrap();
}

fn print_and_trans_to_response<T>(result: anyhow::Result<T>) -> (StatusCode, Response)
where
    T: Serialize + Debug,
{
    match result {
        Ok(r) => {
            let j = Json(r);
            debug!("return result: {:?}", j);
            (StatusCode::OK, j.into_response())
        }
        Err(err) => {
            let err_str = err.to_string();
            error!("{}", err_str);
            (StatusCode::INTERNAL_SERVER_ERROR, err_str.into_response())
        }
    }
}
