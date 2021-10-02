use actix_web::get;

#[get("")]
async fn index() -> String {
    format!("404, invalid!")
}

#[get("/health")]
async fn health() -> String {
    format!("healthy!") // <- response with app_name
}
