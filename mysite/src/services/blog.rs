use actix_web::{get, web};

#[get("")]
pub async fn get_blogs() -> String {
    format!("blogsList!") // <- response with app_name
}

#[get("/{id}")]
pub async fn get_blog_by_id(id: web::Path<(u32,)>) -> String {
    format!("get blog with id = {}", id.into_inner().0) // <- response with app_name
}
