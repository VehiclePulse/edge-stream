use tokio_util::sync::CancellationToken;
use tokio::signal::unix::{ SignalKind, signal};

pub async fn wait_for_shutdown_signal(
    cancellatin_token: CancellationToken
) -> std::io::Result<()> {
    
    let mut signint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = signint.recv() => {
            println!("Received SIGINT (Ctrl+C), shutting down...");
        },
        _ = sigterm.recv() => {
            println!("Received SIGTERM, shutting down...");
        }
    }

    cancellatin_token.cancel();

    Ok(())
}