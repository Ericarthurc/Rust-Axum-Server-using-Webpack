use askama::Template;
use axum::{
    body::{self, Full},
    extract,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                ))))
                .unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api", get(get_data).delete(delete_data))
        .route("/about", get(about))
        .nest(
            "/public",
            get_service(ServeDir::new("./public/")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> Response {
    let template = IndexTemplate {};
    HtmlTemplate(template).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    id: i32,
    name: String,
    age: i8,
}

async fn get_data() -> Response {
    let payload: Vec<Package> = vec![
        Package {
            id: 79,
            name: String::from("Tim"),
            age: 40,
        },
        Package {
            id: 2,
            name: String::from("John"),
            age: 25,
        },
        Package {
            id: 3,
            name: String::from("Kim"),
            age: 12,
        },
    ];
    Json(payload).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageId {
    id: i32,
}

async fn delete_data(extract::Json(payload): extract::Json<PackageId>) -> Response {
    println!("{:?}", payload);

    ().into_response()
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

async fn about() -> Response {
    let template = AboutTemplate {};
    HtmlTemplate(template).into_response()
}
