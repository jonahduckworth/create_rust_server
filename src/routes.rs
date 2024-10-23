use crate::api::organization_api;
use actix_web::{web, Scope};

pub fn api_routes() -> Scope {
    web::scope("/api").service(
        web::scope("/organizations")
            .route("", web::post().to(organization_api::create_organization))
            .route("/{id}", web::get().to(organization_api::get_organization))
            .route(
                "/{id}",
                web::put().to(organization_api::update_organization),
            )
            .route(
                "/{id}",
                web::delete().to(organization_api::delete_organization),
            )
            .route("", web::get().to(organization_api::list_organizations)),
    )
}
