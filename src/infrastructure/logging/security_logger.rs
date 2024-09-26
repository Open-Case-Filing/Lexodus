// // src/infrastructure/logging/security_logger.rs
// use log::{info, warn, error};

// pub struct SecurityLogger;

// impl SecurityLogger {
//     pub fn log_access(user_id: i64, endpoint: &str, ip: &str) {
//         info!("Access: user_id={}, endpoint={}, ip={}", user_id, endpoint, ip);
//     }

//     pub fn log_failed_auth(ip: &str, reason: &str) {
//         warn!("Failed auth attempt: ip={}, reason={}", ip, reason);
//     }

//     pub fn log_security_event(event_type: &str, details: &str) {
//         error!("Security event: type={}, details={}", event_type, details);
//     }
// }
