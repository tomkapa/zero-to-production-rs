use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct Application {
    /// Allow test to init the app with arbitrary port.
    /// This is useful for avoiding port conflicts when running tests
    port: u16,
    server: Server,
}

impl Application {
    /// Build the application with the provided configuration.
    pub async fn build(configuration: Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        // port can be 0 to let the OS assign a free port.
        // Hence, we need to get the port assigned by the OS.
        // This is useful for tests.
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool).await?;

        Ok(Self { port, server })
    }

    /// Retrieve the port the application is running on.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Run the application until stopped.
    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await?;
        Ok(())
    }
}

// TODO: Should this function be moved to a different module?
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    // Using *lazy* to avoid create too many connections at startup
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            // web::Data as a container (smart pointer) to share the connection pool across the application
            .app_data(web::Data::new(connection_pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
