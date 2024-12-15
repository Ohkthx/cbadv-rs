//! Contains custom / shorthand types to simplify end-user code.

use futures::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::errors::CbError;

/// Used to return objects from the API to the end-user.
pub type CbResult<T> = Result<T, CbError>;

pub(crate) type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// The 'Read' of a split socket.
pub type WebSocketReader = SplitStream<Socket>;
