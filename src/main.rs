#![deny(warnings)]
use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use async_once::AsyncOnce;
use db::Database;
use handlers::login::{self};
use juniper::{EmptySubscription, RootNode};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use lazy_static::lazy_static;
use log::{error, warn, Level};
use mutation::Mutation;
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::trace::{SpanKind, Status};
use opentelemetry::{global, sdk::propagation::TraceContextPropagator, trace::Tracer};
use opentelemetry_stdout::SpanExporter;

use query::Query;

use crate::data_loaders::get_loader;

mod data_loaders;
mod db;
mod handlers;
mod models;
mod mutation;
mod query;
mod utils;

impl juniper::Context for Database {}

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Database>::new())
}

async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}
async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn login_route(req: actix_web::HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(login::login(req.uri().path())))
}
async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let tracer = global::tracer("gql_server");

    let mut span = tracer
        .span_builder(format!("{} {}", req.method(), req.uri().path()))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    let context = DATABASE.get().await;
    error!(target: "gottem", "hello from {}. My price is {}", "apple", 2.99);
    match graphql_handler(&schema, context, req, payload).await {
        Ok(response) => {
            span.set_status(Status::Ok);
            Ok(response)
        }
        Err(e) => {
            span.set_status(Status::error(e.to_string()));
            Err(e)
        }
    }
}

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(async {
        let sport_loader = get_loader();
        Database::new(sport_loader).await.unwrap()
    });
}

fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);
}

fn init_logger() {
    env::set_var("RUST_LOG", "warn");
    env_logger::init();

    // let exporter = opentelemetry_stdout::LogExporterBuilder::default()
    //     // .with_encoder(|writer, data| Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
    //     .build();

    // let logger_provider = LoggerProvider::builder()
    //     .with_config(
    //         Config::default().with_resource(Resource::new(vec![KeyValue::new(
    //             "gql_server",
    //             "logs-basic-example",
    //         )])),
    //     )
    //     .with_simple_exporter(exporter)
    //     .build();

    // Setup Log Appender for the log crate.
    // let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);

    // match log::set_boxed_logger(Box::new(otel_log_appender)) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         warn!("Failed to set logger: {}", e);
    //     }
    // }
    log::set_max_level(Level::Warn.to_level_filter());
    warn!(target: "my-target", "hello from {}. My price is {}", "apple", 2.99);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_tracer();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
            .service(web::resource("/login").route(web::get().to(login_route)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
