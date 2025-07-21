use std::time::Duration;

use convex_sync_types::backoff::Backoff;
use convex_sync_types::{AuthenticationToken, UdfPath};
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::{mpsc, oneshot};
use futures::stream::Fuse; // Removed FusedStream
use futures::{FutureExt, StreamExt, pin_mut, select};
use gloo_timers::future::sleep;
use js_sys::Math::random;
use leptos::logging::log;

use crate::base::query_result::{FunctionResult, QueryResults};
use crate::base::{BaseConvexClient, SubscriberId};
use crate::websocket::{ProtocolResponse, ReconnectProtocolReason, ReconnectRequest, SyncProtocol};
use serde_json::Value;

use super::subscription::QuerySubscription;

const INITIAL_BACKOFF: Duration = Duration::from_millis(100);
const MAX_BACKOFF: Duration = Duration::from_secs(15);

pub enum ClientRequest {
    Mutation(
        MutationRequest,
        oneshot::Sender<oneshot::Receiver<FunctionResult>>,
    ),
    Action(
        ActionRequest,
        oneshot::Sender<oneshot::Receiver<FunctionResult>>,
    ),
    Subscribe(
        SubscribeRequest,
        oneshot::Sender<QuerySubscription>,
        mpsc::UnboundedSender<ClientRequest>,
    ),
    Unsubscribe(UnsubscribeRequest),
    Authenticate(Box<AuthenticateRequest>),
}

pub struct MutationRequest {
    pub udf_path: UdfPath,
    pub args: Value,
}

pub struct ActionRequest {
    pub udf_path: UdfPath,
    pub args: Value,
}

pub struct SubscribeRequest {
    pub udf_path: UdfPath,
    pub args: Value,
}

pub struct AuthenticateRequest {
    pub token: AuthenticationToken,
}

#[derive(Debug)]
pub struct UnsubscribeRequest {
    pub subscriber_id: SubscriberId,
}

pub async fn worker<T: SyncProtocol>(
    protocol_response_receiver: mpsc::Receiver<ProtocolResponse>,
    client_request_receiver: mpsc::UnboundedReceiver<ClientRequest>,
    mut watch_sender: async_broadcast::Sender<QueryResults>,
    mut base_client: BaseConvexClient,
    shutdown_listen: oneshot::Receiver<()>,
    mut protocol_manager: T,
) {
    log!("Convex worker started.");
    let mut backoff = Backoff::new(INITIAL_BACKOFF, MAX_BACKOFF);
    let mut protocol_response_stream = protocol_response_receiver.fuse();
    let mut client_request_stream = client_request_receiver.fuse();
    let mut shutdown_listen = shutdown_listen.fuse();

    loop {
        let worker_future = _worker_once(
            &mut protocol_response_stream,
            &mut client_request_stream,
            &mut watch_sender,
            &mut base_client,
            &mut protocol_manager,
        );

        select! {
            _ = shutdown_listen => {
                log!("Shutdown signal received, breaking worker loop.");
                break;
            },
            worker_result = worker_future.fuse() => {
                match worker_result {
                    Ok(()) => {
                        log!("Worker iteration completed successfully, resetting backoff.");
                        backoff.reset();
                    },
                    Err(e) => {
                        let delay = backoff.fail(random());
                        log!("Worker iteration failed: {:?}. Retrying in {:.2} seconds.", e, delay.as_secs_f32());
                        protocol_manager
                            .reconnect(ReconnectRequest {
                                reason: e,
                                max_observed_timestamp: base_client.max_observed_timestamp(),
                            })
                            .await;
                        base_client.resend_ongoing_queries_mutations();
                        flush_messages(&mut base_client, &mut protocol_manager).await;
                        sleep(delay).await;
                    }
                }
            }
        }
    }
    log!("Convex worker stopped.");
}

async fn _worker_once<T: SyncProtocol>(
    protocol_response_stream: &mut Fuse<mpsc::Receiver<ProtocolResponse>>,
    client_request_stream: &mut Fuse<UnboundedReceiver<ClientRequest>>,
    watch_sender: &mut async_broadcast::Sender<QueryResults>,
    base_client: &mut BaseConvexClient,
    protocol_manager: &mut T,
) -> Result<(), ReconnectProtocolReason> {
    log!("_worker_once entered.");
    pin_mut!(protocol_response_stream);
    pin_mut!(client_request_stream);
    select! {
        protocol_response_opt = protocol_response_stream.next() => {
            match protocol_response_opt {
                Some(protocol_response) => {
                    log!("Received protocol response: {:?}", protocol_response);
                    match protocol_response {
                        ProtocolResponse::ServerMessage(msg) => {
                            if let Some(subscriber_id_to_latest_value) = base_client.receive_message(msg)? {
                                log!("Broadcasting updated query results.");
                                let _ = watch_sender.broadcast(subscriber_id_to_latest_value).await;
                            }
                        },
                        ProtocolResponse::Failure => {
                            log!("ProtocolResponse::Failure received.");
                            return Err("ProtocolFailure".into());
                        },
                    }
                    Ok(())
                },
                None => {
                    log!("Protocol stream terminated unexpectedly.");
                    Err("ProtocolStreamTerminated".into())
                },
            }
        },
        client_request_opt = client_request_stream.next() => {
            match client_request_opt {
                Some(client_request) => {
                    match client_request {
                        ClientRequest::Subscribe(query, tx, request_sender) => {
                            let SubscribeRequest {
                                udf_path,
                                args,
                            } =  query;
                            log!("Handling ClientRequest::Subscribe for path: {:?}", udf_path);
                            let watch = watch_sender.new_receiver();
                            let subscriber_id = base_client.subscribe(udf_path, args);
                            flush_messages(base_client, protocol_manager).await;

                            let subscription = QuerySubscription {
                                subscriber_id,
                                request_sender,
                                watch,
                                initial: base_client.latest_results().get(&subscriber_id).cloned(),
                            };
                            let _ = tx.send(subscription);
                        },
                        ClientRequest::Mutation(mutation, tx) => {
                            let MutationRequest {
                                udf_path,
                                args,
                            } = mutation;
                            log!("Handling ClientRequest::Mutation for path: {:?}", udf_path);
                            let result_receiver = base_client
                                .mutation(udf_path, args);
                            flush_messages(base_client, protocol_manager).await;
                            let _ = tx.send(result_receiver);
                        },
                        ClientRequest::Action(action, tx) => {
                            let ActionRequest {
                                udf_path,
                                args,
                            } = action;
                            log!("Handling ClientRequest::Action for path: {:?}", udf_path);
                            let result_receiver = base_client
                                .action(udf_path, args);
                            flush_messages(base_client, protocol_manager).await;
                            let _ = tx.send(result_receiver);
                        },
                        ClientRequest::Unsubscribe(unsubscribe) => {
                            let UnsubscribeRequest {subscriber_id} = unsubscribe;
                            log!("Handling ClientRequest::Unsubscribe for subscriber_id: {:?}", subscriber_id);
                            base_client.unsubscribe(subscriber_id);
                            flush_messages(base_client, protocol_manager).await;
                        },
                        ClientRequest::Authenticate(authenticate) => {
                            log!("Handling ClientRequest::Authenticate.");
                            base_client.set_auth(authenticate.token);
                            flush_messages(base_client, protocol_manager).await;
                        },
                    }
                    Ok(())
                },
                None => {
                    log!("Client request stream terminated unexpectedly.");
                    Err("ClientStreamTerminated".into())
                },
            }
        },
        complete => {
            log!("All streams terminated in _worker_once.");
            Err("AllStreamsTerminated".into())
        }
    }
}

/// Flush all messages to the protocol
async fn flush_messages<P: SyncProtocol>(base_client: &mut BaseConvexClient, protocol: &mut P) {
    while let Some(modification) = base_client.pop_next_message() {
        log!("Flushing a message to protocol.");
        let _ = protocol.send(modification).await;
    }
}
