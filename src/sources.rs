use crate::types::{AccountId, ResourceId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::errors::SourceError;

pub mod file;
pub mod null;

#[derive(Serialize, Deserialize, Default)]
pub struct Resources {
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Resource {
    pub resource_id: ResourceId,
    pub name: String,
    pub url: Url,
    pub description: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct User {
    pub username: String,
}

#[async_trait]
pub trait Source {
    async fn get_user_for_resource(
        &self,
        account_id: &AccountId,
        resource_id: &ResourceId,
    ) -> Result<Option<User>, SourceError>;
    async fn get_resources(
        &self,
        account_id: &AccountId,
    ) -> Result<Resources, SourceError>;
}
