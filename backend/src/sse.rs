use crate::models::StreamEvent;
use axum::response::sse::{Event, KeepAlive, Sse};
use chrono::Utc;
use futures::Stream;
use serde_json::json;
use std::{convert::Infallible, time::Duration};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

pub fn stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let interval = tokio::time::interval(Duration::from_secs(2));
    let stream = IntervalStream::new(interval).enumerate().map(|(index, _)| {
        let cpu = ((index % 11) * 9 + 17) as u64;
        let memory = ((index % 13) * 7 + 23) as u64;
        let event = StreamEvent {
            kind: "heartbeat".to_string(),
            status: "running".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            detail: json!({
                "cpu_percent": cpu,
                "memory_percent": memory,
                "network": { "rx_kbps": 120 + index as u64 * 3, "tx_kbps": 80 + index as u64 * 2 },
                "file": { "queue_depth": index % 5 },
                "security": { "risk_score": 10 + (index % 7) },
            }),
        };
        Ok(Event::default().data(serde_json::to_string(&event).unwrap()))
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("keepalive"))
}
