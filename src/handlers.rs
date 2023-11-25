/// Handlers
///
/// At the top level, there is only 1 GET that manages the redirect
/// GET /{tiny_url}
use actix_web::{delete, get, http::header, patch, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    app_state::AppState,
    model::{ListUrlsResponse, MessageResponse, UrlRequest},
    service::ServiceError,
};

/// GET /api/{key}
/// Redirects to the url associated with the key
/// It's important to not use PermanentRedirect, or browsers will cache it.  Bad for testing!
#[get("/{key}")]
async fn redirect_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    // The path holds the "key" value
    let result = data.service.get(&path.into_inner()).await;

    match result {
        Ok(tiny) => {
            // Redirect to the associated URL
            log::info!("Redirecting to {}", &tiny.src_url);
            HttpResponse::TemporaryRedirect()
                .append_header((header::LOCATION, tiny.src_url))
                .finish()
        }
        Err(err) => {
            // The key wasn't found.  Just return NotFound
            log::info!("Error from DB: {}", err.to_string());
            HttpResponse::NotFound().json(json!({"status":"not found", "message":"URL not found"}))
        }
    }
}

/// The following are the API handlers for managing the url collection
/// GET /api/healthcheck:  Just for testing that server is up
/// PUT /api/url: enters the url provided in URLRequest
/// GET /api/url/{tiny_url}:  Returns the TinyUrl
/// POST /api/url/{tiny_url}: Updates the existing entry with the UrlRequest
/// DELETE /api/url/{tiny_url}:  Delete the entry

/// Just returns an OK status message, if server is running.
#[get("/healthcheck")]
async fn api_health_checker_handler() -> impl Responder {
    let message = MessageResponse::new("success", json!("API Server is up and running"));
    HttpResponse::Ok().json(message)
}

/// GET /api/url/{tiny_url}
/// Returns the [TinyUrl] or Not Found
#[get("/url/{tiny_url}")]
async fn get_url_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let tiny_url = path.into_inner();
    let url = data.service.get(&tiny_url).await?;
    Ok(HttpResponse::Ok().json(MessageResponse::new("ok", json!(url))))
}

/// GET /api/url
/// Returns all urls,  Not Found
#[get("/url")]
async fn get_all_url_handler(data: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let urls = data.service.all().await?;
    let list = ListUrlsResponse { urls };

    Ok(HttpResponse::Ok().json(json!({
        "status":"success",
        "message": list
    })))
}

/// PUT /api/url
/// Create a tiny url associated with the url in the [UrlRequest] body.  The provided
/// url must be valid.
/// Response: [TinyUrl]
///
/// Note: The UrlRequest is validated in the model.
#[put("/url")]
async fn put_url_handler(
    body: web::Json<UrlRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let tiny = data.service.create(&body).await?;

    Ok(HttpResponse::Created().json(json!({"status": "success","message": tiny})))
}

/// PATCH /api/url/{tiny_url}
/// Update a stored tiny url with the src_url from [UrlRequest] body
/// Response: [TinyUrl]
///
/// Note: the UrlRequest is validated in the model.
#[patch("/url/{tiny_url}")]
async fn patch_url_handler(
    body: web::Json<UrlRequest>,
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let tiny_url = path.into_inner();

    let tiny = data.service.update(&tiny_url, &body).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status":"success",
        "message": tiny
    })))
}

/// DELETE /api/url/{tiny_url}
/// Returns success or Not Found
#[delete("/url/{tiny_url}")]
async fn delete_url_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let tiny_url = path.into_inner();

    data.service.delete(&tiny_url).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status":"success",
        "message": format!("{} successfully deleted", &tiny_url)
    })))
}

/// Demonstrate chaining scopes with ServiceConfig
pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(redirect_handler)
        // API handlers
        .service(
            web::scope("/api")
                .service(api_health_checker_handler)
                .service(get_all_url_handler)
                .service(get_url_handler)
                .service(put_url_handler)
                .service(patch_url_handler)
                .service(delete_url_handler),
        );
}
