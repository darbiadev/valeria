use poem::{get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let app = Route::new().at("/hello/:name", get(hello)).nest("/api", api_service).nest("/", ui).with(Tracing);    
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}
