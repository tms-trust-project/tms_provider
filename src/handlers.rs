use std::sync::Arc;

use crate::{
    auth::UserClaims,
    sources::{Resource, User},
};
use axum::extract::State;
use axum_responses::JsonResponse;
use serde::{Deserialize, Serialize};

use crate::{errors::ServiceError, state::AppState};

#[derive(Serialize, Deserialize)]
struct ResourceForUser {
    resource: Resource,
    user: User,
}

pub async fn resources(
    State(state): State<Arc<AppState>>,
    UserClaims(claims): UserClaims,
) -> Result<JsonResponse, ServiceError> {
    let Some(account_id) = claims.subject else {
        return Err(ServiceError::MissingSubject);
    };
    let resources = state.source.get_resources(&account_id).await?;
    let mut result = vec![];
    for resource in resources.resources {
        let maybe_user = state
            .source
            .get_user_for_resource(&account_id, &resource.resource_id).await?;
        if let Some(user) = maybe_user {
            result.push(ResourceForUser {
                resource, user
            })
        }
    }
    Ok(JsonResponse::Ok().message("success").data(result))
}
