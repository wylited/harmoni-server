
#[cfg(test)]
mod tests {
    use crate::*;

    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use std::net::{Ipv4Addr, SocketAddr};
    use futures::{StreamExt, SinkExt};
    use tokio_tungstenite::tungstenite;
    use tower::util::ServiceExt;

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

    #[tokio::test]
    async fn root_test() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, Client!");
    }
}