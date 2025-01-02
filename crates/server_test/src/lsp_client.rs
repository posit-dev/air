//
// lsp_client.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use lsp_server::Connection;
use lsp_server::RequestId;
use lsp_types::ClientInfo;
use lsp_types::GeneralClientCapabilities;
use lsp_types::PositionEncodingKind;
use std::num::NonZeroUsize;
use std::thread::JoinHandle;

pub const TEST_CLIENT_NAME: &str = "AirTestClient";

pub struct TestClient {
    client: Connection,
    server_handle: Option<JoinHandle<()>>,
    request_id: i32,
    encoding: PositionEncodingKind,
    init_params: Option<lsp_types::InitializeParams>,
}

impl TestClient {
    pub fn new<F>(start_server: F) -> Self
    where
        F: FnOnce(NonZeroUsize, lsp_server::Connection, Option<lsp_server::IoThreads>)
            + Send
            + 'static,
    {
        let worker_threads = NonZeroUsize::new(4).unwrap();
        let (server, client) = lsp_server::Connection::memory();

        let server_handle = std::thread::spawn(move || {
            start_server(worker_threads, server, None);
        });

        Self {
            client,
            server_handle: Some(server_handle),
            request_id: 0,
            encoding: PositionEncodingKind::UTF8,
            init_params: None,
        }
    }

    pub fn encoding(&self) -> &PositionEncodingKind {
        &self.encoding
    }

    fn id(&mut self) -> RequestId {
        let id = self.request_id;
        self.request_id = id + 1;
        RequestId::from(id)
    }

    pub fn recv_response(&mut self) -> lsp_server::Response {
        // Unwrap: Result (Err if stream closed)
        let message = self.client.receiver.recv().unwrap();

        match message {
            lsp_server::Message::Request(request) => panic!("Expected response, got {request:?}"),
            lsp_server::Message::Response(response) => response,
            lsp_server::Message::Notification(notification) => {
                panic!("Expected response, got {notification:?}")
            }
        }
    }

    pub fn notify<N>(&mut self, params: N::Params)
    where
        N: lsp_types::notification::Notification,
    {
        let method = N::METHOD.to_string();
        let notification = lsp_server::Notification::new(method, params);
        let message = lsp_server::Message::Notification(notification);

        // Unwrap: For this test client it's fine to panic if we can't send
        self.client.sender.send(message).unwrap()
    }

    pub fn request<R>(&mut self, params: R::Params) -> RequestId
    where
        R: lsp_types::request::Request,
    {
        let id = self.id();
        let method = R::METHOD.to_string();
        let request = lsp_server::Request::new(id.clone(), method, params);
        let message = lsp_server::Message::Request(request);

        // Unwrap: For this test client it's fine to panic if we can't send
        self.client.sender.send(message).unwrap();

        id
    }

    pub fn initialize(&mut self) -> RequestId {
        let params: Option<lsp_types::InitializeParams> = std::mem::take(&mut self.init_params);
        let params = params.unwrap_or_default();
        let params = Self::with_client_info(params);
        let params = Self::with_utf8(params);
        self.request::<lsp_types::request::Initialize>(params)
    }

    // Regardless of how we got the params, ensure the client name is set to
    // `AirTestClient` so we can recognize it when we set up global logging.
    fn with_client_info(
        mut init_params: lsp_types::InitializeParams,
    ) -> lsp_types::InitializeParams {
        init_params.client_info = Some(ClientInfo {
            name: String::from(TEST_CLIENT_NAME),
            version: None,
        });
        init_params
    }

    // Regardless of how we got the params, ensure we use UTF-8 encodings
    fn with_utf8(mut init_params: lsp_types::InitializeParams) -> lsp_types::InitializeParams {
        init_params.capabilities.general = Some(GeneralClientCapabilities {
            position_encodings: Some(vec![
                PositionEncodingKind::UTF8,
                PositionEncodingKind::UTF16,
            ]),
            ..Default::default()
        });
        init_params
    }

    pub fn initialized(&mut self) {
        let params = lsp_types::InitializedParams {};
        self.notify::<lsp_types::notification::Initialized>(params)
    }

    pub fn with_initialize_params(&mut self, init_params: lsp_types::InitializeParams) {
        self.init_params = Some(init_params);
    }

    pub fn close_document(&mut self, uri: url::Url) {
        let params = lsp_types::DidCloseTextDocumentParams {
            text_document: lsp_types::TextDocumentIdentifier { uri },
        };
        self.did_close_text_document(params)
    }

    pub fn shutdown(&mut self) {
        self.check_no_incoming();
        let id = self.request::<lsp_types::request::Shutdown>(());
        assert_eq!(id, self.recv_response().id);
    }

    fn check_no_incoming(&self) {
        let mut messages = Vec::new();

        while let Ok(message) = self.client.receiver.try_recv() {
            messages.push(message);
        }

        if !messages.is_empty() {
            panic!("Must handle all messages before shutdown. Found the following unhandled incoming messages:\n{messages:?}");
        }
    }

    pub fn exit(&mut self) {
        // Unwrap: Can only exit once
        let server_handle =
            std::mem::take(&mut self.server_handle).expect("`exit()` can only be called once");

        self.notify::<lsp_types::notification::Exit>(());

        // Now wait for the server task to complete.
        // Unwrap: Panics if task can't shut down as expected
        server_handle
            .join()
            .expect("Couldn't join on the server thread.");
    }

    pub fn did_open_text_document(&mut self, params: lsp_types::DidOpenTextDocumentParams) {
        self.notify::<lsp_types::notification::DidOpenTextDocument>(params)
    }

    pub fn did_close_text_document(&mut self, params: lsp_types::DidCloseTextDocumentParams) {
        self.notify::<lsp_types::notification::DidCloseTextDocument>(params)
    }

    pub fn formatting(&mut self, params: lsp_types::DocumentFormattingParams) -> RequestId {
        self.request::<lsp_types::request::Formatting>(params)
    }

    pub fn range_formatting(
        &mut self,
        params: lsp_types::DocumentRangeFormattingParams,
    ) -> RequestId {
        self.request::<lsp_types::request::RangeFormatting>(params)
    }
}
