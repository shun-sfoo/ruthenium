use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from any origin
        // .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()));
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", get(get_users))
        .route("/projects", get(get_projects))
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_users() -> impl IntoResponse {
    let mut users = Vec::new();
    users.push(User {
        id: 1,
        name: "王文静".to_string(),
    });

    users.push(User {
        id: 2,
        name: "刘康".to_string(),
    });

    (StatusCode::OK, Json(users))
}

async fn get_projects() -> impl IntoResponse {
    let mut projects = Vec::new();
    projects.push(Project {
        id: 1,
        name: "骑手管理".to_string(),
        person_id: 1,
        organization: "外卖组".to_string(),
        created: 2,
    });

    projects.push(Project {
        id: 2,
        name: "骑手管".to_string(),
        person_id: 2,
        organization: "外卖".to_string(),
        created: 2,
    });

    (StatusCode::OK, Json(projects))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        name: payload.name,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    id: u64,
    name: String,
    person_id: u64,
    organization: String,
    created: u64,
}
