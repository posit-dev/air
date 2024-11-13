//
// lsp_client.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use futures::{executor::block_on, StreamExt};
use futures_util::sink::SinkExt;
use std::future::Future;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tokio_util::codec::{FramedRead, FramedWrite};
use tower_lsp::{jsonrpc, lsp_types};

use crate::codec::LanguageServerCodec;
use crate::request::Request;

pub struct TestClient {
    pub rx: FramedRead<ReadHalf<SimplexStream>, LanguageServerCodec<jsonrpc::Response>>,
    pub tx: FramedWrite<WriteHalf<SimplexStream>, LanguageServerCodec<Request>>,

    server_handle: Option<tokio::task::JoinHandle<()>>,
    id_counter: i64,
}

impl TestClient {
    pub fn new<F, Fut>(start: F) -> Self
    where
        F: FnOnce(Box<dyn AsyncRead + Unpin + Send>, Box<dyn AsyncWrite + Unpin + Send>) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let (client_rx, client_tx) = tokio::io::simplex(1024);
        let (server_rx, server_tx) = tokio::io::simplex(1024);

        let server_handle = tokio::spawn(start(Box::new(server_rx), Box::new(client_tx)));

        let rx = FramedRead::new(client_rx, LanguageServerCodec::default());
        let tx = FramedWrite::new(server_tx, LanguageServerCodec::default());

        Self {
            rx,
            tx,
            server_handle: Some(server_handle),
            id_counter: 0,
        }
    }

    // `jsonrpc::Id` requires i64 IDs
    fn id(&mut self) -> i64 {
        let id = self.id_counter;
        self.id_counter = id + 1;
        id
    }

    pub async fn recv_response(&mut self) -> jsonrpc::Response {
        // Unwrap: Option (None if stream closed), then Result (Err if codec fails).
        self.rx.next().await.unwrap().unwrap()
    }

    pub async fn request<R>(&mut self, params: R::Params) -> i64
    where
        R: lsp_types::request::Request,
    {
        let id = self.id();
        let req = Request::from_request::<R>(jsonrpc::Id::Number(id), params);

        // Unwrap: For this test client it's fine to panic if we can't send
        self.tx.send(req).await.unwrap();

        id
    }

    pub async fn initialize(&mut self) -> i64 {
        let params = lsp_types::InitializeParams::default();
        self.request::<lsp_types::request::Initialize>(params).await
    }

    pub async fn shutdown(&mut self) -> i64 {
        self.request::<lsp_types::request::Shutdown>(()).await
    }
}

impl Drop for TestClient {
    fn drop(&mut self) {
        // TODO: Check that no messages are pending

        // Unwrap: We drop only once, so handle must be Some
        let _handle = std::mem::take(&mut self.server_handle).unwrap();

        block_on(async {
            self.shutdown().await;

            // TODO: Implement Shutdown
            // Unwrap: Panics if task can't shut down as expected
            // handle.await.unwrap();
        })
    }
}
