use std::{ops::Deref, pin::Pin};

use futures::channel::mpsc;
use futures::{Stream, StreamExt, task};

use crate::base::SubscriberId;
use crate::base::query_result::{FunctionResult, QueryResults};

use super::worker::{ClientRequest, UnsubscribeRequest};

/// This structure represents a single subscription to a query with args.
/// For convenience, [`QuerySubscription`] also implements
/// [`Stream`]<[`FunctionResult`]>, giving a stream of results to the query.
///
/// It is returned by [`ConvexClient::subscribe`]. The subscription lives
/// in the active query set for as long as this token stays in scope.
///
/// For a consistent [`QueryResults`] of all your queries, use
/// [`ConvexClient::watch_all()`] instead.
pub struct QuerySubscription {
    pub(super) subscriber_id: SubscriberId,
    pub(super) request_sender: mpsc::UnboundedSender<ClientRequest>,
    pub(super) watch: async_broadcast::Receiver<QueryResults>,
    pub(super) initial: Option<FunctionResult>,
}
impl QuerySubscription {
    /// Returns an identifier for this subscription based on its query and args.
    /// This identifier can be used to find the result within a
    /// [`QuerySetSubscription`] as returned by [`ConvexClient::watch_all()`]
    pub fn id(&self) -> &SubscriberId {
        &self.subscriber_id
    }
}
impl std::fmt::Debug for QuerySubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuerySubscription")
            .field("subscriber_id", &self.subscriber_id)
            .finish()
    }
}
impl Deref for QuerySubscription {
    type Target = SubscriberId;

    fn deref(&self) -> &SubscriberId {
        &self.subscriber_id
    }
}
impl Drop for QuerySubscription {
    fn drop(&mut self) {
        let _ = self
            .request_sender
            .unbounded_send(ClientRequest::Unsubscribe(UnsubscribeRequest {
                subscriber_id: self.subscriber_id,
            }));
    }
}
impl Stream for QuerySubscription {
    type Item = FunctionResult;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        if let Some(initial) = self.initial.take() {
            return task::Poll::Ready(Some(initial));
        }
        loop {
            return match self.watch.poll_next_unpin(cx) {
                task::Poll::Ready(Some(map)) => {
                    let Some(value) = map.get(self.id()) else {
                        continue;
                    };
                    task::Poll::Ready(Some(value.clone()))
                }
                task::Poll::Ready(None) => task::Poll::Ready(None),
                task::Poll::Pending => task::Poll::Pending,
            };
        }
    }
}

/// A subscription to a consistent view of multiple queries.
///
/// [`QuerySetSubscription`]
/// implements [`Stream`]<[`QueryResults`]>.
/// Each item in the stream contains a consistent view
/// of the results of all the queries in the query set.
///
/// Queries can be added to the query set via [`ConvexClient::subscribe`].
/// Queries can be removed from the query set via dropping the
/// [`QuerySubscription`] token returned by [`ConvexClient::subscribe`].
///
///
/// [`QueryResults`] is a copy-on-write mapping from [`SubscriberId`] to
/// its latest result [`Value`].
pub struct QuerySetSubscription {
    watch: async_broadcast::Receiver<QueryResults>,
}
impl QuerySetSubscription {
    pub(super) fn new(watch: async_broadcast::Receiver<QueryResults>) -> Self {
        Self { watch }
    }
}
impl Stream for QuerySetSubscription {
    type Item = QueryResults;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        match self.watch.poll_next_unpin(cx) {
            // Ok to be lagged (skip intermediate values) - since Convex
            // only guarantees a newer value than the previous value.
            task::Poll::Ready(Some(map)) => task::Poll::Ready(Some(map)),
            task::Poll::Ready(None) => task::Poll::Ready(None),
            task::Poll::Pending => task::Poll::Pending,
        }
    }
}
