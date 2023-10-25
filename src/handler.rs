use anyhow::anyhow;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

use crate::driver::VolumeDriver;
use axum::body::Bytes;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{middleware, routing::post, Router};
use hyper::{Body, Server};
use hyperlocal::UnixServerExt;
use tokio::fs;
use tracing::{debug, info};

pub struct VolumeHandler<T: VolumeDriver> {
    driver: Arc<T>,
}

impl<T: VolumeDriver> VolumeHandler<T> {
    pub fn new(driver: T) -> Self {
        Self {
            driver: Arc::new(driver),
        }
    }

    pub async fn run_tcp(&self, port: u16) -> Result<(), anyhow::Error> {
        info!("Starting Volume handler on port: {}", port);
        let app = self.build_router();

        let addr = format!("0.0.0.0:{port}").parse()?;
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }

    pub async fn run_unix_socket(&self, socket_path: PathBuf) -> Result<(), anyhow::Error> {
        info!("Starting Volume handler on unix socket: {:?}", socket_path);
        // setup socket file
        if socket_path.exists() {
            fs::remove_file(&socket_path).await?;
        }
        fs::create_dir_all(socket_path.parent().ok_or(anyhow!("no parent dir"))?).await?;

        let app = self.build_router();
        Server::bind_unix(socket_path)?
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    fn build_router(&self) -> Router {
        Router::new()
            .route("/Plugin.Activate", post(T::activate))
            .route("/VolumeDriver.Create", post(T::create))
            .route("/VolumeDriver.Remove", post(T::remove))
            .route("/VolumeDriver.Mount", post(T::mount))
            .route("/VolumeDriver.Unmount", post(T::unmount))
            .route("/VolumeDriver.Get", post(T::get))
            .route("/VolumeDriver.List", post(T::list))
            .route("/VolumeDriver.Path", post(T::path))
            .route("/VolumeDriver.Capabilities", post(T::capabilities))
            .with_state(self.driver.clone())
            .layer(middleware::from_fn(print_request_response))
    }
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (mut parts, body) = req.into_parts();
    let (mut headers, uri) = (parts.headers.clone(), parts.uri.clone());
    debug!("handling request for: {:?}", uri);
    let bytes = buffer_and_print("request", &headers, body).await?;
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    parts.headers = headers;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let headers = &parts.headers;
    let bytes = buffer_and_print("response", headers, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B, T: Debug>(
    direction: &str,
    headers: &HeaderMap<T>,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction}, err: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        debug!("{} headers = {:?}, body = {:?}", direction, headers, body);
    }

    Ok(bytes)
}
