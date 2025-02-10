//
// lsp_client.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use futures::StreamExt;
use futures_util::sink::SinkExt;
use std::future::Future;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
use tokio_util::codec::{FramedRead, FramedWrite};
use tower_lsp::lsp_types::ClientInfo;
use tower_lsp::{jsonrpc, lsp_types};

use crate::tower_lsp::codec::LanguageServerCodec;
use crate::tower_lsp::request::Request;

pub struct TestClient {
    pub rx: FramedRead<ReadHalf<SimplexStream>, LanguageServerCodec<jsonrpc::Response>>,
    pub tx: FramedWrite<WriteHalf<SimplexStream>, LanguageServerCodec<Request>>,

    server_handle: Option<tokio::task::JoinHandle<()>>,
    id_counter: i64,

    init_params: Option<lsp_types::InitializeParams>,
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
            init_params: None,
        }
    }

    fn id(&mut self) -> jsonrpc::Id {
        let id = self.id_counter;
        self.id_counter = id + 1;
        jsonrpc::Id::Number(id)
    }

    pub async fn recv_response(&mut self) -> jsonrpc::Response {
        // Unwrap: Option (None if stream closed), then Result (Err if codec fails).
        self.rx.next().await.unwrap().unwrap()
    }

    pub async fn notify<N>(&mut self, params: N::Params)
    where
        N: lsp_types::notification::Notification,
    {
        let not = Request::from_notification::<N>(params);

        // Unwrap: For this test client it's fine to panic if we can't send
        self.tx.send(not).await.unwrap();
    }

    pub async fn request<R>(&mut self, params: R::Params) -> jsonrpc::Id
    where
        R: lsp_types::request::Request,
    {
        let id = self.id();
        let req = Request::from_request::<R>(id.clone(), params);

        // Unwrap: For this test client it's fine to panic if we can't send
        self.tx.send(req).await.unwrap();

        id
    }

    pub async fn initialize(&mut self) -> jsonrpc::Id {
        let params: Option<lsp_types::InitializeParams> = std::mem::take(&mut self.init_params);
        let params = params.unwrap_or_default();
        let params = Self::with_client_info(params);
        self.request::<lsp_types::request::Initialize>(params).await
    }

    // Regardless of how we got the params, ensure the client name is set to
    // `AirTestClient` so we can recognize it when we set up global logging.
    fn with_client_info(
        mut init_params: lsp_types::InitializeParams,
    ) -> lsp_types::InitializeParams {
        init_params.client_info = Some(ClientInfo {
            name: String::from("AirTestClient"),
            version: None,
        });
        init_params
    }

    pub fn with_initialize_params(&mut self, init_params: lsp_types::InitializeParams) {
        self.init_params = Some(init_params);
    }

    pub async fn initialized(&mut self) {
        let params = lsp_types::InitializedParams {};
        self.notify::<lsp_types::notification::Initialized>(params)
            .await
    }

    pub async fn close_document(&mut self, uri: url::Url) {
        let params = lsp_types::DidCloseTextDocumentParams {
            text_document: lsp_types::TextDocumentIdentifier { uri },
        };
        self.did_close_text_document(params).await;
    }

    pub async fn shutdown(&mut self) {
        // TODO: Check that no messages are incoming
        let id = self.id();

        // Don't use `Request::from_request()`. It has a bug with undefined
        // params (when `R::Params = ()`) which causes tower-lsp to not
        // recognise the Shutdown request.
        let req = Request::build("shutdown").id(id.clone()).finish();

        // Unwrap: For this test client it's fine to panic if we can't send
        self.tx.send(req).await.unwrap();

        let response = self.recv_response().await;
        assert_eq!(&id, response.id());
    }

    pub async fn exit(&mut self) {
        // Unwrap: Can only exit once
        let handle = std::mem::take(&mut self.server_handle).unwrap();

        self.notify::<lsp_types::notification::Exit>(()).await;

        // Now wait for the server task to complete.
        // Unwrap: Panics if task can't shut down as expected
        handle.await.unwrap();
    }

    pub async fn did_open_text_document(&mut self, params: lsp_types::DidOpenTextDocumentParams) {
        self.notify::<lsp_types::notification::DidOpenTextDocument>(params)
            .await
    }

    pub async fn did_close_text_document(&mut self, params: lsp_types::DidCloseTextDocumentParams) {
        self.notify::<lsp_types::notification::DidCloseTextDocument>(params)
            .await
    }

    pub async fn did_change_workspace_folders(
        &mut self,
        params: lsp_types::DidChangeWorkspaceFoldersParams,
    ) {
        self.notify::<lsp_types::notification::DidChangeWorkspaceFolders>(params)
            .await
    }

    pub async fn formatting(&mut self, params: lsp_types::DocumentFormattingParams) -> jsonrpc::Id {
        self.request::<lsp_types::request::Formatting>(params).await
    }

    pub async fn range_formatting(
        &mut self,
        params: lsp_types::DocumentRangeFormattingParams,
    ) -> jsonrpc::Id {
        self.request::<lsp_types::request::RangeFormatting>(params)
            .await
    }
}
