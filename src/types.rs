//! Contains custom / shorthand types to simplify end-user code.

use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{errors::CbError, models::websocket::Message};

/// Used to return objects from the API to the end-user.
pub type CbResult<T> = Result<T, CbError>;

pub(crate) type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Used to define a custom callback for messages to be passed to.
pub type MessageCallbackFn = fn(CbResult<Message>);

/// The 'Read' of a split socket.
pub type WebSocketReader = SplitStream<Socket>;
