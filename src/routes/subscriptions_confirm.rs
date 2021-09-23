use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Parameters {
    
    subscription_token: String

}

#[tracing::instrument(
name = "Confirm a pending subscriber",
)]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {

    HttpResponse::Ok().finish()

}