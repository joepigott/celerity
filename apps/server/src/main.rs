use std::path::PathBuf;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use celerity::scheduler::Scheduler;
use celerity::server::Server;
use celerity::TaskQueue;

mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config::config().expect("Unable to retrieve configuration.");

    let queue = Arc::new(Mutex::new(
        recover_queue(&config.scheduler.data_path).unwrap_or(TaskQueue::new()),
    ));
    let mut scheduler = Scheduler::with_queue(Arc::clone(&queue));
    let mut server = Server::with_queue(Arc::clone(&queue));
    let sigterm = Arc::new(AtomicBool::new(false));

    // handle termination signals
    let ctrlc_sigterm = Arc::clone(&sigterm);
    tokio::spawn(async move {
        let sigint = tokio::signal::ctrl_c();
        let mut sigterm =
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

        tokio::select! {
            _ = sigint => {
                log::info!("Received SIGINT. Signaling thread to exit...");
            },
            _ = sigterm.recv() => {
                log::info!("Received SIGTERM. Signaling thread to exit...");
            }
        }

        ctrlc_sigterm.store(true, Ordering::Relaxed);
    });

    // scheduler thread
    let sched_sigterm = Arc::clone(&sigterm);
    let sched_thread = tokio::spawn(async move {
        if let Err(e) = scheduler.run(Arc::clone(&sched_sigterm), config.scheduler).await {
            log::error!("{e}");
            return;
        }
    });

    // server thread
    let serv_thread = tokio::spawn(async move {
        if let Err(e) = server.run(config.server).await {
            log::error!("{e}");
            return;
        }
    });

    // wait on scheduler thread
    if let Err(e) = sched_thread.await {
        log::error!("{e}");
    };

    // server thread doesn't need to exit gracefully, so we can abort it
    serv_thread.abort();

    log::info!("Exited.");
}

fn recover_queue(storage: &PathBuf) -> Option<TaskQueue> {
    let data = fs::read_to_string(storage).ok()?;
    serde_json::from_str::<TaskQueue>(&data).ok()
}
