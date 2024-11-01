use crate::api::types::{
    organization::{
        CreateOrganizationInput, ListOrganizationsQuery, OrganizationResponse,
        UpdateOrganizationInput,
    },
    pagination::{PaginatedResponse, PaginationParams},
    responses::ApiResponse,
};
use crate::db::{get_connection, models::Organization, DbPool};

use crate::errors::AppResult;
use crate::services::organization::OrganizationService;
use actix_web::{web, HttpResponse};
use log::{debug, info};
use uuid::Uuid;

pub mod read {
    use super::*;

    #[utoipa::path(
        get,
        path = "/v1/organizations/{id}",
        responses(
            (status = 200, description = "Organization found", body = OrganizationResponse),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn get_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to retrieve organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        let organization = OrganizationService::find_by_id(&mut conn, org_id)?;

        info!("Retrieved organization: {}", organization.id);
        Ok(HttpResponse::Ok().json(ApiResponse::new(
            OrganizationResponse {
                organization: organization.into(),
            },
            None,
            "success",
        )))
    }

    #[utoipa::path(
        get,
        path = "/v1/organizations",
        responses(
            (status = 200, description = "List of organizations", body = PaginatedResponse<Organization>),
            (status = 400, description = "Bad request"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("limit" = Option<i64>, Query, description = "Limit the number of organizations"),
            ("offset" = Option<i64>, Query, description = "Offset for pagination")
        )
    )]
    pub async fn list_organizations(
        pool: web::Data<DbPool>,
        query: web::Query<ListOrganizationsQuery>,
    ) -> AppResult<HttpResponse> {
        let limit = query.limit.unwrap_or(10);
        let offset = query.offset.unwrap_or(0);
        let page = (offset / limit) + 1;

        let mut conn = get_connection(&pool)?;

        let pagination = PaginationParams {
            page,
            per_page: limit,
        };

        let organizations = OrganizationService::list(&mut conn, &pagination)?;
        let total = organizations.len() as i64;

        info!("Retrieved {} organizations", organizations.len());
        Ok(HttpResponse::Ok().json(ApiResponse::new(
            PaginatedResponse::new(organizations, total, &pagination),
            None,
            "success",
        )))
    }
}

pub mod create {
    use super::*;

    #[utoipa::path(
        post,
        path = "/v1/organizations",
        request_body = CreateOrganizationInput,
        responses(
            (status = 201, description = "Organization created", body = OrganizationResponse),
            (status = 400, description = "Bad request"),
            (status = 500, description = "Internal server error")
        )
    )]
    pub async fn create_organization(
        pool: web::Data<DbPool>,
        new_organization: web::Json<CreateOrganizationInput>,
    ) -> AppResult<HttpResponse> {
        let mut conn = get_connection(&pool)?;

        let organization = OrganizationService::create(&mut conn, new_organization.into_inner())?;

        Ok(HttpResponse::Created().json(ApiResponse::new(
            OrganizationResponse {
                organization: organization.into(),
            },
            None,
            "success",
        )))
    }
}

pub mod update {
    use super::*;

    #[utoipa::path(
        put,
        path = "/v1/organizations/{id}",
        request_body = UpdateOrganizationInput,
        responses(
            (status = 200, description = "Organization updated", body = OrganizationResponse),
            (status = 400, description = "Bad request"),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn update_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
        updated_organization: web::Json<UpdateOrganizationInput>,
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to update organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        let organization =
            OrganizationService::update(&mut conn, org_id, updated_organization.into_inner())?;

        info!("Updated organization: {}", organization.id);
        Ok(HttpResponse::Ok().json(ApiResponse::new(
            OrganizationResponse {
                organization: organization.into(),
            },
            None,
            "success",
        )))
    }
}

pub mod delete {
    use super::*;

    #[utoipa::path(
        delete,
        path = "/v1/organizations/{id}",
        responses(
            (status = 204, description = "Organization deleted"),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn delete_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to delete organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        OrganizationService::delete(&mut conn, org_id)?;

        info!("Deleted organization: {}", org_id);
        Ok(HttpResponse::NoContent().finish())
    }
}
