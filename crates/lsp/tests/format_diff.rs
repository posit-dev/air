mod fixtures;

use fixtures::test_client::init_test_client;
use fixtures::tower_lsp_test_client::TestClientExt;
use lsp::documents::Document;

// https://github.com/posit-dev/air/issues/61
#[tests_macros::lsp_test]
async fn test_format_minimal_diff() {
    let mut client = init_test_client().await;

    #[rustfmt::skip]
    let doc = Document::doodle(
"1
2+2
3
",
    );

    let edits = client.format_document_edits(&doc).await.unwrap();
    assert!(edits.len() == 1);

    let edit = &edits[0];
    assert_eq!(edit.new_text, " + ");

    client
}
