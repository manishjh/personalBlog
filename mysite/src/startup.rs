use crate::services;
use actix_web::{dev::Server, guard, web, App, HttpResponse, HttpServer};
use std::{io::Error, net::TcpListener};
pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(services::index)
                    .service(services::health),
            )
            .service(
                web::scope("/blog")
                    .service(services::get_blogs)
                    .service(services::get_blog_by_id),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
