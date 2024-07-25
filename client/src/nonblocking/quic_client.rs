#[deprecated(
    since = "1.15.0",
    note = "Please use `lumos_quic_client::nonblocking::quic_client::QuicClientConnection` instead."
)]
pub use lumos_quic_client::nonblocking::quic_client::QuicClientConnection as QuicTpuConnection;
pub use lumos_quic_client::nonblocking::quic_client::{
    QuicClient, QuicClientCertificate, QuicLazyInitializedEndpoint,
};
