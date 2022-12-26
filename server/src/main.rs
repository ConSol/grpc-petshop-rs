use std::sync::Arc;

use std::error::Error;
use tokio::{sync::Mutex, try_join};
use tonic::{codegen::http::header::HeaderName, transport::Server};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

mod api;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// main entry
#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

/// macro free entry-point running the server
async fn run() -> Result<()> {
    let db = create_pet_db();

    // gRPC server on `:8080`
    let grpc_server = Server::builder()
        .add_service(api::auth())
        .add_service(api::shop(db.clone()))
        .serve(
            "127.0.0.1:8080"
                .parse()
                .expect("valid address can be parsed"),
        );

    // http-gRPC bridge server on `:8081`.
    // Browser cannot use real gRPC because it's not based on HTTP.
    let web_server = Server::builder()
        .accept_http1(true)
        // because client and server doesn't have the same origin (different port):
        // 1. server has to allow origins, headers and methods explicitely
        // 2. server has to allow the client to read specific gRPC response headers.
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any)
                .expose_headers([
                    HeaderName::from_static("grpc-status"),
                    HeaderName::from_static("grpc-message"),
                ]),
        )
        .layer(GrpcWebLayer::new())
        .add_service(api::auth())
        .add_service(api::shop(db))
        .serve(
            "127.0.0.1:8081"
                .parse()
                .expect("valid address can be parsed"),
        );

    // run both servers
    try_join!(grpc_server, web_server)?;
    Ok(())
}

/// fake db with some samples
fn create_pet_db() -> PetDb {
    PetDb {
        data: Arc::new(Mutex::new(vec![
            Pet {
                id: 1,
                age: 4,
                name: "Ferris".to_owned(),
            },
            Pet {
                id: 2,
                age: 3,
                name: "Lilly".to_owned(),
            },
            Pet {
                id: 3,
                age: 8,
                name: "Ratty".to_owned(),
            },
        ])),
    }
}

#[derive(Clone)]
pub struct PetDb {
    data: Arc<Mutex<Vec<Pet>>>,
}

#[derive(Clone)]
struct Pet {
    id: i64,
    age: i32,
    name: String,
}
