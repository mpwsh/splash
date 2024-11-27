use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Metrics {
    peers: Arc<AtomicUsize>,
    messages_broadcasted: Arc<AtomicUsize>,
    messages_received: Arc<AtomicUsize>,
    total_connections: Arc<AtomicUsize>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(AtomicUsize::new(0)),
            messages_broadcasted: Arc::new(AtomicUsize::new(0)),
            messages_received: Arc::new(AtomicUsize::new(0)),
            total_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn increment_peers(&self) -> usize {
        self.peers.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn decrement_peers(&self) -> usize {
        self.peers.fetch_sub(1, Ordering::SeqCst) - 1
    }

    pub fn increment_messages_received(&self) {
        self.messages_received.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_messages_broadcasted(&self) {
        self.messages_broadcasted.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_metrics(&self) -> MetricsData {
        MetricsData {
            peers: self.peers.load(Ordering::SeqCst),
            messages_broadcasted: self.messages_broadcasted.load(Ordering::SeqCst),
            messages_received: self.messages_received.load(Ordering::SeqCst),
            total_connections: self.total_connections.load(Ordering::SeqCst),
        }
    }
}

#[derive(Serialize)]
pub struct MetricsData {
    pub peers: usize,
    pub messages_broadcasted: usize,
    pub messages_received: usize,
    pub total_connections: usize,
}
