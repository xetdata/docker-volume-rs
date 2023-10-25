use std::collections::HashMap;
use std::sync::Arc;

use crate::errors::VolumeResponse;
use async_trait::async_trait;
use axum::extract::State;
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, Value};

const ACTIVATE_RESPONSE: &str = r#"{"Implements": ["VolumeDriver"]}"#;

#[async_trait]
pub trait VolumeDriver: Send + Sync + 'static {
    async fn activate() -> Json<Value> {
        Json(from_str(ACTIVATE_RESPONSE).unwrap())
    }
    async fn create(
        driver: State<Arc<Self>>,
        request: Json<CreateRequest>,
    ) -> VolumeResponse<Json<NullResponse>>;
    async fn remove(
        driver: State<Arc<Self>>,
        request: Json<RemoveRequest>,
    ) -> VolumeResponse<Json<NullResponse>>;
    async fn mount(
        driver: State<Arc<Self>>,
        request: Json<MountRequest>,
    ) -> VolumeResponse<Json<MountResponse>>;
    async fn unmount(
        driver: State<Arc<Self>>,
        request: Json<UnmountRequest>,
    ) -> VolumeResponse<Json<NullResponse>>;
    async fn path(
        driver: State<Arc<Self>>,
        request: Json<PathRequest>,
    ) -> VolumeResponse<Json<PathResponse>>;
    async fn get(
        driver: State<Arc<Self>>,
        request: Json<GetRequest>,
    ) -> VolumeResponse<Json<GetResponse>>;
    async fn list(driver: State<Arc<Self>>) -> VolumeResponse<Json<ListResponse>>;
    async fn capabilities(driver: State<Arc<Self>>) -> VolumeResponse<Json<CapabilitiesResponse>>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateRequest {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Opts")]
    pub options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemoveRequest {
    #[serde(alias = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MountRequest {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MountResponse {
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnmountRequest {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathRequest {
    #[serde(alias = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathResponse {
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRequest {
    #[serde(alias = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetResponse {
    #[serde(rename = "Volume")]
    pub volume: Option<Volume>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListResponse {
    #[serde(rename = "Volumes")]
    pub volumes: Vec<Volume>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Volume {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    #[serde(rename = "Status")]
    pub status: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CapabilitiesResponse {
    #[serde(rename = "Capabilities")]
    pub capabilities: Capability,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Capability {
    #[serde(rename = "Scope")]
    pub scope: Scope,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Local,
    Global,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NullResponse {}
