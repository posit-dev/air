use ruff_source_file::LineEnding;
use ruff_source_file::LineIndex;

use crate::edit::Indel;
use crate::edit::PositionEncoding;
use crate::edit::TextEdit;
use crate::proto::TextRangeExt;

impl TextEdit {
    pub(crate) fn into_proto(
        self,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
        ending: LineEnding,
    ) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
        self.into_iter()
            .map(|indel| indel.into_proto(text, index, encoding, ending))
            .collect()
    }
}

impl Indel {
    fn into_proto(
        self,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
        ending: LineEnding,
    ) -> anyhow::Result<lsp_types::TextEdit> {
        let range = self.delete.into_proto(text, index, encoding);
        let new_text = match ending {
            LineEnding::Lf => self.insert,
            LineEnding::Crlf => self.insert.replace('\n', "\r\n"),
            LineEnding::Cr => self.insert.replace('\n', "\r"),
        };
        Ok(lsp_types::TextEdit { range, new_text })
    }
}
