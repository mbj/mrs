//! Event streaming for GitHub repository events.

use std::collections::VecDeque;
use std::time::Duration;

use futures_util::{Stream, StreamExt};

use crate::github::{Client, Event, EventsPage, ListRepositoryEvents, Repository};

/// Error during event collection.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Pagination error.
    #[error("Pagination error: {0}")]
    Paginate(#[from] mhttp::link::PaginateError<crate::github::Error>),
}

/// Result of collecting events until a stop condition.
pub struct CollectResult {
    /// Collected events in chronological order (oldest first).
    pub events: VecDeque<Event>,
    /// Server-recommended polling interval from the first page.
    pub poll_interval: Duration,
    /// ETag from the first page for conditional requests.
    pub etag: String,
    /// Whether the stop condition was met (false = desynced, pagination exhausted without finding last seen event).
    pub stopped: bool,
}

/// Collects events from the paginated API until the stop condition is met.
///
/// Iterates through pages of events (newest first from the API) and collects
/// events until the stop function returns true for an event.
///
/// If `etag` is provided, sends conditional request; returns `None` if not modified.
///
/// Returns collected events with metadata. Check `stopped` to determine if
/// the stop condition was met or if we desynced (pagination exhausted without finding last seen event).
pub async fn collect_until<F>(
    client: &mut Client,
    repository: &Repository,
    etag: Option<&str>,
    stop: F,
) -> Result<Option<CollectResult>, Error>
where
    F: Fn(&Event) -> bool,
{
    let mut collected: VecDeque<Event> = VecDeque::new();
    let mut poll_interval: Option<Duration> = None;
    let mut result_etag: Option<String> = None;

    let request = ListRepositoryEvents {
        repository: repository.clone(),
        etag: etag.map(String::from),
    };

    let mut stream = std::pin::pin!(mhttp::link::paginate(client, request));

    while let Some(result) = stream.next().await {
        let page: EventsPage = result?;

        match page {
            EventsPage::NotModified => return Ok(None),
            EventsPage::Content {
                events,
                poll_interval: page_poll_interval,
                etag: page_etag,
            } => {
                // Capture poll_interval and etag from the first page
                if poll_interval.is_none() {
                    poll_interval = Some(Duration::from_secs(page_poll_interval));
                    result_etag = Some(page_etag);
                }

                for event in events {
                    if stop(&event) {
                        return Ok(Some(CollectResult {
                            events: collected,
                            poll_interval: poll_interval.unwrap_or(Duration::from_secs(60)),
                            etag: result_etag.unwrap_or_default(),
                            stopped: true,
                        }));
                    }
                    collected.push_front(event);
                }
            }
        }
    }

    Ok(Some(CollectResult {
        events: collected,
        poll_interval: poll_interval.unwrap_or(Duration::from_secs(60)),
        etag: result_etag.unwrap_or_default(),
        stopped: false,
    }))
}

/// Creates a stream that yields newly discovered repository events.
///
/// The stream polls the GitHub Events API, tracks the last seen event ID,
/// and only yields events newer than that. Respects the server-recommended
/// polling interval from the X-Poll-Interval header. Uses ETags for conditional
/// requests to reduce rate limit consumption.
///
/// If we desync (pagination exhausted without finding the last seen event,
/// e.g., too many events occurred between polls), the stream terminates.
/// The caller should then restart with a full evaluation.
pub fn stream_new_events(
    client: Client,
    repository: Repository,
) -> impl Stream<Item = Result<Event, Error>> {
    async_stream::try_stream! {
        let mut client = client;
        let mut last_seen_id: Option<String> = None;
        let mut last_etag: Option<String> = None;
        let mut poll_interval = Duration::from_secs(60);

        loop {
            let stop_id = last_seen_id.clone();
            let is_first_poll = stop_id.is_none();

            let result = collect_until(
                &mut client,
                &repository,
                last_etag.as_deref(),
                |event| stop_id.as_ref() == Some(&event.id),
            )
            .await?;

            let poll_interval = match result {
                None => poll_interval,
                Some(result) => {
                    // On first poll, exhausting pagination is expected (all events are new)
                    // On subsequent polls, exhausting pagination means we lost track
                    if !result.stopped && !is_first_poll {
                        break;
                    }

                    // Update tracking state
                    poll_interval = result.poll_interval;
                    last_etag = Some(result.etag);

                    if let Some(newest) = result.events.back() {
                        last_seen_id = Some(newest.id.clone());
                    }

                    for event in result.events {
                        yield event;
                    }

                    poll_interval
                }
            };

            log::info!("Sleeping for {poll_interval:?} before next poll");
            tokio::time::sleep(poll_interval).await;
        }
    }
}
