use std::time::Duration;

use anyhow::Context;
use async_trait::async_trait;
use convex_sync_types::{ClientMessage, SessionId, Timestamp, backoff::Backoff};
use futures::{
    FutureExt, SinkExt, StreamExt,
    channel::{
        mpsc::{self, UnboundedReceiver},
        oneshot::{self, Receiver},
    },
    select_biased,
    stream::{Fuse, SplitSink, SplitStream},
};
use gloo_net::websocket::{Message, futures::WebSocket};
use gloo_timers::future::{IntervalStream, sleep};
use js_sys::Math::random;
use leptos::logging::log;
use url::Url;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_time::Instant;

use super::{ProtocolResponse, ReconnectRequest, SyncProtocol, WebSocketState};

const INITIAL_BACKOFF: Duration = Duration::from_millis(100);
const MAX_BACKOFF: Duration = Duration::from_secs(15);

#[derive(Debug)]
enum WebSocketRequest {
    SendMessage(Box<ClientMessage>, oneshot::Sender<()>),
    Reconnect(ReconnectRequest),
}

struct WebSocketInternal {
    tx: SplitSink<WebSocket, Message>,
    rx: SplitStream<WebSocket>,
    last_server_response: Instant,
}
struct WebSocketWorker {
    ws_url: Url,
    on_response: mpsc::Sender<ProtocolResponse>,
    on_state_change: Option<mpsc::Sender<WebSocketState>>,
    internal_receiver: Fuse<UnboundedReceiver<WebSocketRequest>>,
    ping_ticker: IntervalStream,
    connection_count: u32,
    backoff: Backoff,
}

pub struct WebSocketManager {
    internal_sender: mpsc::UnboundedSender<WebSocketRequest>,
    shutdown_sender: oneshot::Sender<()>,
}

#[async_trait]
impl SyncProtocol for WebSocketManager {
    async fn open(
        ws_url: Url,
        on_response: mpsc::Sender<ProtocolResponse>,
        on_state_change: Option<mpsc::Sender<WebSocketState>>,
        client_id: &str,
    ) -> anyhow::Result<Self> {
        let (internal_sender, internal_receiver) = mpsc::unbounded();
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        spawn_local(WebSocketWorker::run(
            ws_url,
            on_response,
            on_state_change,
            internal_receiver,
            client_id.to_string(),
            shutdown_receiver,
        ));
        log!("WebSocketManager: Worker spawned.");

        Ok(WebSocketManager {
            internal_sender,
            shutdown_sender,
        })
    }

    async fn send(&mut self, message: ClientMessage) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.internal_sender
            .send(WebSocketRequest::SendMessage(Box::new(message), tx))
            .await?;
        rx.await?;
        Ok(())
    }

    async fn reconnect(&mut self, request: ReconnectRequest) {
        log!(
            "WebSocketManager: Requesting reconnect with reason: {}",
            request.reason
        );
        let _ = self
            .internal_sender
            .send(WebSocketRequest::Reconnect(request))
            .await;
    }
}

impl WebSocketWorker {
    /// How often heartbeat pings are sent.
    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
    /// How long before lack of server response causes a timeout.
    const SERVER_INACTIVITY_THRESHOLD: Duration = Duration::from_secs(30);

    async fn run(
        ws_url: Url,
        on_response: mpsc::Sender<ProtocolResponse>,
        on_state_change: Option<mpsc::Sender<WebSocketState>>,
        internal_receiver: mpsc::UnboundedReceiver<WebSocketRequest>,
        client_id: String,
        shutdown_receiver: Receiver<()>,
    ) {
        let ping_ticker = IntervalStream::new(Self::HEARTBEAT_INTERVAL.as_millis() as u32);
        let backoff = Backoff::new(INITIAL_BACKOFF, MAX_BACKOFF);

        let mut worker = Self {
            ws_url,
            on_response,
            on_state_change,
            internal_receiver: internal_receiver.fuse(),
            ping_ticker,
            connection_count: 0,
            backoff,
        };

        let mut last_close_reason = "InitialConnect".to_string();
        let mut max_observed_timestamp = None;
        if let Some(mut state_change_sender) = worker.on_state_change.clone() {
            let _ = state_change_sender.try_send(WebSocketState::Connecting);
        }

        let mut shutdown_receiver = shutdown_receiver.fuse();

        loop {
            let exit_result: anyhow::Result<ReconnectRequest> = select_biased! {
                _ = shutdown_receiver => {
                    return;
                },
                result = worker.work(last_close_reason, max_observed_timestamp, &client_id).fuse() => {
                    result
                },
            };

            if let Some(mut state_change_sender) = worker.on_state_change.clone() {
                let _ = state_change_sender.try_send(WebSocketState::Connecting);
            }

            let e = match exit_result {
                Ok(reconnect) => {
                    // WS worker exited cleanly because it got a request to reconnect
                    log!(
                        "WebSocketWorker: Clean reconnect requested. Reason: {}",
                        reconnect.reason
                    );
                    last_close_reason = reconnect.reason;
                    max_observed_timestamp = reconnect.max_observed_timestamp;
                    continue;
                }
                Err(e) => e,
            };
            worker.connection_count += 1;
            last_close_reason = e.to_string();
            let delay = worker.backoff.fail(random());
            log!("Convex WebSocketWorker failed: {e:?}. Backing off for {delay:?} and retrying.",);

            let _ = worker.on_response.send(ProtocolResponse::Failure).await;

            loop {
                log!("Waiting for base client to acknowledge reconnect");
                let request = worker.internal_receiver.next().await;
                if let Some(WebSocketRequest::Reconnect(reconnect)) = request {
                    max_observed_timestamp = reconnect.max_observed_timestamp;
                    break;
                }
                log!("Base client acknowledged reconnect. Sleeping {delay:?} and reconnecting");
            }
            sleep(delay).await;
            log!("Reconnecting");
        }
    }

    async fn work(
        &mut self,
        last_close_reason: String,
        max_seen_transition: Option<Timestamp>,
        client_id: &str,
    ) -> anyhow::Result<ReconnectRequest> {
        let verb = if self.connection_count == 0 {
            "connect"
        } else {
            "reconnect"
        };
        log!("trying to {verb} to {}", self.ws_url);

        let mut internal = WebSocketInternal::new(
            self.ws_url.clone(),
            self.connection_count,
            last_close_reason,
            max_seen_transition,
            client_id,
        )
        .await?;
        log!("WebSocketWorker: Connection established.");

        if let Some(mut state_change_sender) = self.on_state_change.clone() {
            let _ = state_change_sender.try_send(WebSocketState::Connected);
        }

        let mut ws_rx = internal.rx.fuse();
        let mut ws_tx = internal.tx;

        loop {
            select_biased! {
                _ = self.ping_ticker.next().fuse() => {
                    let now = Instant::now();
                    if now - internal.last_server_response > Self::SERVER_INACTIVITY_THRESHOLD {
                        log!("WebSocketWorker: Server inactive, no response for {:?}", now - internal.last_server_response);
                        anyhow::bail!("InactiveServer");
                    }
                },
                server_msg = ws_rx.select_next_some() => {
                    internal.last_server_response = Instant::now();

                    match server_msg {
                        Ok(Message::Text(t)) => {
                            let json: serde_json::Value = serde_json::from_str(&t).context("JsonDeserializeError")?;
                            let server_message = json.try_into()?;

                            let resp = ProtocolResponse::ServerMessage(server_message);
                            let _ = self.on_response.send(resp).await;

                            // TODO: Similar to JS, we should ideally only reset backoff if we get
                            // the client gets into a correct state, where we have Connected and
                            // received a response to our pending Queries and Mutations.
                            self.backoff.reset();
                        },
                        _server_msg => {
                            log!("WebSocketWorker: Received unknown/non-text message from server: {:?}", _server_msg);
                        },
                    }
                },
                request = self.internal_receiver.select_next_some() => {
                    match request {
                        WebSocketRequest::SendMessage(message, sender) => {
                            let msg = Message::Text(serde_json::Value::try_from(*message).context("JsonSerializeError")?.to_string());
                            let _ = ws_tx.send(msg.clone()).await;
                            let _ = sender.send(());
                        },
                        WebSocketRequest::Reconnect(reason) => {
                            log!("WebSocketWorker: Reconnect request received during 'work'. Reason: {}", reason.reason);
                            return Ok(reason);
                        },
                    };
                }
            };
        }
    }
}

impl WebSocketInternal {
    async fn new(
        ws_url: Url,
        connection_count: u32,
        last_close_reason: String,
        max_observed_timestamp: Option<Timestamp>,
        _client_id: &str, // Added underscore for unused variable
    ) -> anyhow::Result<WebSocketInternal> {
        log!(
            "WebSocketInternal: Attempting to open WebSocket to {}",
            ws_url
        );
        let ws_stream = WebSocket::open(ws_url.as_ref())?;
        log!("WebSocketInternal: WebSocket opened.");

        let (tx, rx) = ws_stream.split();

        let last_server_response = Instant::now();
        let mut internal = WebSocketInternal {
            last_server_response,
            tx,
            rx,
        };

        // Send an initial connect message on the new websocket
        let session_id = Uuid::new_v4();
        let message = ClientMessage::Connect {
            session_id: SessionId::new(session_id),
            connection_count,
            last_close_reason,
            max_observed_timestamp,
        };
        log!(
            "WebSocketInternal: Sending initial Connect message: {:?}",
            message
        );
        let msg = Message::Text(
            serde_json::Value::try_from(message)
                .context("JSONSerializationErrorOnConnect")?
                .to_string(),
        );
        internal.send_worker(msg).await?;
        log!("WebSocketInternal: Initial Connect message sent.");

        Ok(internal)
    }

    async fn send_worker(&mut self, message: Message) -> anyhow::Result<()> {
        log!("WebSocketInternal: Sending message to WebSocket.");
        self.tx.send(message).await.context("WebsocketClosedOnSend")
    }
}
