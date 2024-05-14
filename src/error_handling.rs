use log::{error, info};

pub fn log_error(message: &str) {
    error!("Error: {}", message); // Log the error
}

pub fn log_info(message: &str) {
    info!("Info: {}", message); // Log general information
}
