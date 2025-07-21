mod base;
pub mod leptos;
#[cfg(feature = "ssr")]
mod server;
mod websocket;

use serde_json::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConvexError {
    /// From any error, redacted from prod deployments.
    pub message: String,
    /// Custom application error data payload that can be passed from your
    /// function to a client.
    pub data: Value,
}
