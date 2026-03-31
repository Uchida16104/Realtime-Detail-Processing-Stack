use axum::response::sse::{Event, Sse};
use std::convert::Infallible;
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;
use futures::StreamExt;

pub async fn sse_handler() -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let interval = interval(Duration::from_secs(1));

    let stream = IntervalStream::new(interval)
        .enumerate()
        .map(|(index, _)| {
            let data = format!("{{\"count\": {}}}", index);
            Ok(Event::default().data(data))
        });

    Sse::new(stream)
}
