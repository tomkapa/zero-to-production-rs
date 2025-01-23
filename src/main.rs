use anyhow::Result;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero_to_production_rs::configuration;
use zero_to_production_rs::startup::Application;
use zero_to_production_rs::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    // tracing setup
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // application setup
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;

    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select!(
        o = application_task => report_exit("Application", o),
    );

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
