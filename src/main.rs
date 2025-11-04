use axum::{Router, routing::get};
use log::info;
use log4rs::{
    Config,
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

#[tokio::main]
async fn main() {
    if let Err(err) = dotenvy::dotenv() {
        eprintln!("\x1B[31mError loading env file: {}\x1B[0m", err);
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}] {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(log::LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    //TODO: Separar em um arquivo com as rotas em especifico
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let port = 3000;
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap_or_else(|e| panic!("Could not bind to port: {}. Error: {}", port, e));

    info!("Server listening on port: {}", port);
    axum::serve(listener, app).await.unwrap();
}
