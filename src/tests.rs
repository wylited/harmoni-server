
#[cfg(test)]
mod tests {
    use crate::*;

    use std::net::{Ipv4Addr, SocketAddr};
    use futures::{StreamExt, SinkExt};
    use tokio_tungstenite::tungstenite;

    // We can integration test one handler by running the server in a background task and
    // connecting to it like any other client would.
    #[tokio::test]
    async fn echo_test() {
        let listener = tokio::net::TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
            .await
            .unwrap();

        let addr = listener.local_addr().unwrap();
        tokio::spawn(axum::serve(listener, app()));

        let (mut socket, _response) =
            tokio_tungstenite::connect_async(format!("ws://{addr}/echo"))
                .await
                .unwrap();

        socket
            .send(tungstenite::Message::text("foo"))
            .await
            .unwrap();

        let msg = match socket.next().await.unwrap().unwrap() {
            tungstenite::Message::Text(msg) => msg,
            other => panic!("expected a text message but got {other:?}"),
        };

        assert_eq!(msg, "You said: foo");

        socket.close(None).await.unwrap();
    }
}