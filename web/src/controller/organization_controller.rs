use crate::{AppState, Error};
use axum::{async_trait, Extension};
use axum::extract::{FromRequestParts, Path, State};
use axum::http::{Method, request::Parts, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use chrono::{serde::ts_seconds, DateTime, Utc};
use entity::{organization, Id};
use entity_api::organization as OrganizationApi;
use sea_orm::prelude::DateTimeUtc;
use serde::{Serialize, Deserialize};
use serde_json::json;

extern crate log;
use log::*;

#[derive(Debug, Clone)]
pub struct Authorized(pub Claims);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

pub struct AuthorizationMiddleware;

#[async_trait]
impl<S> FromRequestParts<S> for AuthorizationMiddleware
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        debug!("from_request_parts!");
        if parts.method == Method::OPTIONS {
            // For options requests browsers will not send the authorization header.
            return Ok(Self);
        }

        let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await
        else {
            error!("Could not get Authorization header from the request");
            return Err(StatusCode::UNAUTHORIZED);
        };

        match check_auth(bearer) {
            Ok(auth) => {
                // Set `auth` as a request extension so it can be accessed by other
                // services down the stack.
                parts.extensions.insert(auth);

                Ok(Self)
            }
            Err(error) => {
                error!("{error:?}");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

fn check_auth(bearer: Bearer) -> Result<Authorized, String> {
    debug!("check_auth(), bearer: {:?}", bearer);
    Ok(Authorized(Claims { email: "james.hodapp@gmail.com".to_string(), exp: DateTimeUtc::default() }))
}

pub struct OrganizationController {}

impl OrganizationController {
    /// GET all Organizations
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations
    pub async fn index(State(app_state): State<AppState>, Extension(claims): Extension<Authorized>) -> Result<impl IntoResponse, Error> {
        let email = claims.0.email;
        let exp = claims.0.exp;

        debug!("Provided bearer claims email: {email}");
        debug!("Provided bearer claims expiry: {exp}");

        let organizations = OrganizationApi::find_all(app_state.db_conn_ref().unwrap()).await?;

        Ok(Json(organizations))
    }

    /// GET a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request GET \
    /// http://localhost:4000/organizations/<id>
    pub async fn read(
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("GET Organization by id: {}", id);

        let organization: Option<organization::Model> =
            OrganizationApi::find_by_id(app_state.db_conn_ref().unwrap(), id).await?;

        Ok(Json(organization))
    }

    /// CREATE a new Organization entity
    /// Test this with curl: curl --header "Content-Type: application/json" \
    /// --request POST \
    /// --data '{"name":"My New Organization"}' \
    /// http://localhost:4000/organizations
    pub async fn create(
        State(app_state): State<AppState>,
        Json(organization_model): Json<organization::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("CREATE new Organization: {}", organization_model.name);

        let organization: organization::Model =
            OrganizationApi::create(app_state.db_conn_ref().unwrap(), organization_model).await?;

        debug!("Newly Created Organization: {:?}", &organization);

        Ok(Json(organization))
    }

    /// UPDATE a particular Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request PUT  http://localhost:4000/organizations/<id> \
    /// --data '{"name":"My Updated Organization"}'
    pub async fn update(
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
        Json(organization_model): Json<organization::Model>,
    ) -> Result<impl IntoResponse, Error> {
        debug!(
            "UPDATE the entire Organization by id: {}, new name: {}",
            id, organization_model.name
        );

        let updated_organization: organization::Model =
            OrganizationApi::update(app_state.db_conn_ref().unwrap(), id, organization_model)
                .await?;

        Ok(Json(updated_organization))
    }

    /// DELETE an Organization entity specified by its primary key
    /// Test this with curl: curl --header "Content-Type: application/json" \                                                                                         in zsh at 12:03:06
    /// --request DELETE \
    /// http://localhost:4000/organizations/<id>
    pub async fn delete(
        State(app_state): State<AppState>,
        Path(id): Path<Id>,
    ) -> Result<impl IntoResponse, Error> {
        debug!("DELETE Organization by id: {}", id);

        OrganizationApi::delete_by_id(app_state.db_conn_ref().unwrap(), id).await?;
        Ok(Json(json!({"id": id})))
    }
}
