mod fixtures;

use fixtures::test_client::init_test_client;
use fixtures::tower_lsp_test_client::TestClientExt;
use lsp::documents::Document;

#[tests_macros::lsp_test]
async fn test_format() {
    let mut client = init_test_client().await;

    #[rustfmt::skip]
    let doc = Document::doodle(
"
1
2+2
3 + 3 +
3",
    );

    let formatted = client.format_document(&doc).await;
    insta::assert_snapshot!(formatted);

    client
}
