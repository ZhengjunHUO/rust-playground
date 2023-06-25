use log::{debug, error, info, log_enabled, trace, warn, Level};

fn main() {
    env_logger::init();

    if log_enabled!(Level::Trace) {
        trace!("Only see me if level == Trace !");
    }
    debug!("env_logger inited successfully !");
    info!("Testing the log level !");
    warn!("There's an error ahead !");
    error!("Unrecoverable error, terminated !");
}
