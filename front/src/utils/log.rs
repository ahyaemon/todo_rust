#[warn(dead_code)]
pub fn log(message: &str) {
    tracing::info!(message);
}
