use std::sync::atomic::{AtomicUsize, Ordering};


static TCP_PORT_COUNTER: AtomicUsize = AtomicUsize::new(9000);


///
/// TestSession::freeTcpPort() to get any free available TCP port
pub struct TestSession {}

impl TestSession {
    /// Returns any free available TCP port as usize
    pub fn freeTcpPortInt() -> usize {
        let port = TCP_PORT_COUNTER.load(Ordering::SeqCst);
        TCP_PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        port
    }
    /// Returns any free available TCP port as &str
    pub fn freeTcpPortStr() -> String {
        let port = TCP_PORT_COUNTER.load(Ordering::SeqCst);
        TCP_PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        port.to_string()
    }
}
