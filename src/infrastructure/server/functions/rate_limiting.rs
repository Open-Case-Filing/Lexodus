// // src/server/middleware/rate_limit.rs
// use std::collections::HashMap;
// use std::sync::Mutex;
// use std::time::{Duration, Instant};
// use leptos::*;

// struct RateLimiter {
//     requests: HashMap<String, Vec<Instant>>,
//     max_requests: usize,
//     window: Duration,
// }

// impl RateLimiter {
//     fn new(max_requests: usize, window: Duration) -> Self {
//         Self {
//             requests: HashMap::new(),
//             max_requests,
//             window,
//         }
//     }

//     fn is_rate_limited(&mut self, key: &str) -> bool {
//         let now = Instant::now();
//         let requests = self.requests.entry(key.to_string()).or_insert_with(Vec::new);

//         requests.retain(|&time| now.duration_since(time) < self.window);

//         if requests.len() >= self.max_requests {
//             true
//         } else {
//             requests.push(now);
//             false
//         }
//     }
// }

// pub fn rate_limit(max_requests: usize, window: Duration) -> impl Fn(leptos::Request) -> Result<leptos::Request, ServerFnError> {
//     let limiter = Mutex::new(RateLimiter::new(max_requests, window));

//     move |req: leptos::Request| {
//         let ip = req.headers().get("X-Forwarded-For")
//             .and_then(|value| value.to_str().ok())
//             .unwrap_or("unknown");

//         let mut limiter = limiter.lock().unwrap();
//         if limiter.is_rate_limited(ip) {
//             Err(ServerFnError::ServerError("Rate limit exceeded".into()))
//         } else {
//             Ok(req)
//         }
//     }
// }
