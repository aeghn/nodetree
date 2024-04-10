use std::path::PathBuf;

use axum::{
    body::{self, Bytes},
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{Stream, TryStreamExt};

use tokio::{
    fs::File,
    io::{self, BufWriter},
};
use tokio_util::io::ReaderStream;
use tracing::{error, info};

use crate::{controller::print_and_trans_to_response, utils::split_uuid_to_file_name};

use super::WebAppState;

fn get_filepath(state: &State<WebAppState>, id: &str) -> PathBuf {
    let filename_parts = split_uuid_to_file_name(&id);

    let save_filepath = std::path::Path::new(&state.config.common.asset_base_dir)
        .join(filename_parts.0)
        .join(filename_parts.1)
        .join(filename_parts.2);
    save_filepath
}

async fn upload(state: State<WebAppState>, mut multipart: Multipart) -> impl IntoResponse {
    let mut result = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        let content_type = field.content_type().unwrap().to_string();

        let mapper = &state.mapper;

        let id = mapper.generate_asset_id();

        let save_filepath = get_filepath(&state, &id);
        let save_dir = save_filepath.parent().unwrap();

        if !tokio::fs::metadata(&save_dir).await.is_ok() {
            match tokio::fs::create_dir_all(&save_dir).await {
                Ok(_) => {
                    error!("created directory {:?}", &save_dir);
                }
                err => {
                    error!("unable to create {:?}", &save_dir);
                    return print_and_trans_to_response(
                        err.map_err(|err| anyhow::anyhow!(err.to_string())),
                    );
                }
            }
        }

        match stream_to_file(field, &save_filepath).await {
            Ok(()) => {
                match mapper
                    .insert_asset(&filename, id.clone(), content_type, Some("".to_string()))
                    .await
                {
                    Ok(asset) => {
                        result.push(asset);
                    }
                    err => {
                        return print_and_trans_to_response(
                            err.map_err(|err| anyhow::anyhow!(err.to_string())),
                        )
                    }
                }
                info!("Saved {} to {:?}", filename, save_filepath);
            }
            err => {
                return print_and_trans_to_response(
                    err.map_err(|err| anyhow::anyhow!(err.to_string())),
                )
            }
        }
    }
    print_and_trans_to_response(Ok(result))
}

async fn stream_to_file<S, E>(stream: S, save_file: &PathBuf) -> Result<(), std::io::Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<axum::BoxError>,
{
    async {
        let body_with_io_error =
            stream.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let body_reader = tokio_util::io::StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let mut file = BufWriter::new(File::create(save_file).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
}

// https://github.com/tokio-rs/axum/discussions/608
pub async fn download(state: State<WebAppState>, Path(id): Path<String>) -> impl IntoResponse {
    info!("download id: {}", id);
    let asset = match state.mapper.query_asset_by_id(id.as_str()).await {
        Ok(asset) => asset,
        Err(err) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Record not found: {}, uuid: {}", err, id),
            ))
        }
    };

    let save_filepath = get_filepath(&state, &id);

    let file = match tokio::fs::File::open(&save_filepath).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("File not found: {:?}, {}", &save_filepath, err),
            ))
        }
    };

    let stream = ReaderStream::new(file);
    let body = body::Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, asset.content_type),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{:?}\"", &asset.ori_file_name),
        ),
    ];
    Ok((headers, body))
}

pub fn routes() -> Router<WebAppState> {
    Router::new()
        .route("/api/upload", post(upload))
        .route("/api/download/:id", get(download))
}
