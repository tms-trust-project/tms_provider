use async_trait::async_trait;
use tracing::warn;

use super::{AccountId, Resources};
use crate::sources::Source;
use crate::{errors::SourceError, sources::User, types::ResourceId};

pub struct NullSource;

#[async_trait]
impl Source for NullSource {
    async fn get_user_for_resource(
        &self,
        _account_id: &AccountId,
        _resource_id: &ResourceId,
    ) -> Result<Option<User>, SourceError> {
        warn!("Using Null Data Source.");
        Ok(Some(User {
            username: "noname".to_string(),
        }))
    }
    async fn get_resources(
        &self,
        _account: &AccountId,
    ) -> Result<Resources, SourceError> {
        warn!(
            "Using Null Data Source. Use the option `source_kind` to specify another source"
        );
        Ok(Resources { resources: vec![] })
    }
}
