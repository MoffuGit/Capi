use std::{collections::BTreeMap, fmt::Display, ops::Deref};

use derive_more::{Deref, FromStr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{Timestamp, UdfPath};

#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Hash,
)]
pub struct QueryId(u32);

impl QueryId {
    pub fn new(id: u32) -> Self {
        QueryId(id)
    }

    pub fn get_id(&self) -> u32 {
        self.0
    }
}

impl Display for QueryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type QuerySetVersion = u32;
pub type IdentityVersion = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    pub query_id: QueryId,
    pub udf_path: UdfPath,
    pub args: Vec<JsonValue>,

    /// Query journals are only specified on reconnect. Also old clients
    /// (<=0.2.1) don't send them.
    pub journal: Option<SerializedQueryJournal>,

    /// For internal use by Convex dashboard. Only works with admin auth.
    /// Allows calling a query within a component directly.
    pub component_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySetModification {
    Add(Query),
    Remove { query_id: QueryId },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientMessage {
    Connect {
        session_id: SessionId,
        connection_count: u32,
        last_close_reason: String,
        max_observed_timestamp: Option<Timestamp>,
    },
    ModifyQuerySet {
        base_version: QuerySetVersion,
        new_version: QuerySetVersion,
        modifications: Vec<QuerySetModification>,
    },
    Mutation {
        request_id: SessionRequestSeqNumber,
        udf_path: UdfPath,
        args: Vec<JsonValue>,
        /// For internal use by Convex dashboard. Only works with admin auth.
        /// Allows calling a mutation within a component directly.
        component_path: Option<String>,
    },
    Action {
        request_id: SessionRequestSeqNumber,
        udf_path: UdfPath,
        args: Vec<JsonValue>,
        /// For internal use by Convex dashboard. Only works with admin auth.
        /// Allows calling an action within a component directly.
        component_path: Option<String>,
    },
    Authenticate {
        base_version: IdentityVersion,
        token: AuthenticationToken,
    },
    Event(ClientEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientEvent {
    pub event_type: String,
    pub event: JsonValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserIdentifier(pub String);
impl UserIdentifier {
    pub fn construct(issuer_name: &str, subject: &str) -> Self {
        Self(format!("{}|{}", issuer_name, subject))
    }
}

impl Deref for UserIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Make issuer and subject not optional to match TypeScript
// type and runtime behavior. Requires all FunctionTesters
// to require them.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserIdentityAttributes {
    pub token_identifier: UserIdentifier,
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub nickname: Option<String>,
    pub preferred_username: Option<String>,
    pub profile_url: Option<String>,
    pub picture_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub gender: Option<String>,
    pub birthday: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub address: Option<String>,
    /// Stored as RFC3339 string
    pub updated_at: Option<String>,

    pub custom_claims: BTreeMap<String, String>,
}

impl Default for UserIdentityAttributes {
    fn default() -> Self {
        UserIdentityAttributes {
            token_identifier: UserIdentifier::construct("convex", "fake_user"),
            subject: None,
            issuer: None,
            name: None,
            email: None,
            given_name: None,
            family_name: None,
            nickname: None,
            preferred_username: None,
            profile_url: None,
            picture_url: None,
            website_url: None,
            email_verified: None,
            gender: None,
            birthday: None,
            timezone: None,
            language: None,
            phone_number: None,
            phone_number_verified: None,
            address: None,
            updated_at: None,
            custom_claims: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum AuthenticationToken {
    /// Admin key issued by a KeyBroker, potentially acting as a user.
    Admin(String, Option<UserIdentityAttributes>),
    /// OpenID Connect JWT
    User(String),
    #[default]
    /// Logged out.
    None,
}

/// The serialized representation of the query journal for pagination.
pub type SerializedQueryJournal = Option<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateModification {
    QueryUpdated {
        query_id: QueryId,
        value: Value,
        log_lines: LogLinesMessage,
        journal: SerializedQueryJournal,
    },
    QueryFailed {
        query_id: QueryId,
        error_message: String,
        log_lines: LogLinesMessage,
        journal: SerializedQueryJournal,
        error_data: Option<Value>,
    },
    QueryRemoved {
        query_id: QueryId,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StateVersion {
    pub query_set: QuerySetVersion,
    pub identity: IdentityVersion,
    pub ts: Timestamp,
}

impl StateVersion {
    pub fn initial() -> Self {
        Self {
            query_set: 0,
            identity: 0,
            ts: Timestamp::MIN,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerMessage {
    Transition {
        start_version: StateVersion,
        end_version: StateVersion,
        modifications: Vec<StateModification>,
    },
    MutationResponse {
        request_id: SessionRequestSeqNumber,
        result: Result<Value, ErrorPayload<Value>>,
        ts: Option<Timestamp>,
        log_lines: LogLinesMessage,
    },
    ActionResponse {
        request_id: SessionRequestSeqNumber,
        result: Result<Value, ErrorPayload<Value>>,
        log_lines: LogLinesMessage,
    },
    AuthError {
        error_message: String,
        base_version: Option<IdentityVersion>,
        // We want to differentiate between "updating auth starting at version `base_version`
        // failed" and "auth at version `base_version` is invalid" (e.g. it expired)
        auth_update_attempted: Option<bool>,
    },
    FatalError {
        error_message: String,
    },
    Ping,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorPayload<V: 'static> {
    /// From any error, redacted from prod deployments.
    Message(String),
    /// From ConvexError, never redacted.
    /// `message` is generic, partly for backwards compatibility.
    ErrorData { message: String, data: V },
}

impl<V: 'static> ErrorPayload<V> {
    pub fn get_message(&self) -> &str {
        match self {
            ErrorPayload::Message(message) => message,
            ErrorPayload::ErrorData { message, .. } => message,
        }
    }

    pub fn get_data(&self) -> Option<&V> {
        match self {
            ErrorPayload::Message(..) => None,
            ErrorPayload::ErrorData { message: _, data } => Some(data),
        }
    }
}

/// List of log lines from a Convex function execution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LogLinesMessage(pub Vec<String>);

#[derive(Copy, Clone, Debug, Deref, Eq, FromStr, PartialEq)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<SessionId> for Uuid {
    fn from(id: SessionId) -> Self {
        id.0
    }
}

// The seq number of a request with a session. Uniquely identifies a
// modification request within a session.
pub type SessionRequestSeqNumber = u32;
