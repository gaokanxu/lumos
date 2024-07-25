#[deprecated(
    since = "1.15.0",
    note = "Please use `lumos_connection_cache::client_connection::ClientConnection` instead."
)]
pub use lumos_connection_cache::client_connection::ClientConnection as TpuConnection;
pub use lumos_connection_cache::client_connection::ClientStats;
