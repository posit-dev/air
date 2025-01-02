use source_file::LineEnding;
use source_file::SourceFile;

use crate::document::Indel;
use crate::document::PositionEncoding;
use crate::document::TextEdit;
use crate::proto::TextRangeExt;

impl TextEdit {
    pub(crate) fn into_proto(
        self,
        source: &SourceFile,
        encoding: PositionEncoding,
        ending: LineEnding,
    ) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
        self.into_iter()
            .map(|indel| indel.into_proto(source, encoding, ending))
            .collect()
    }
}

impl Indel {
    fn into_proto(
        self,
        source: &SourceFile,
        encoding: PositionEncoding,
        ending: LineEnding,
    ) -> anyhow::Result<lsp_types::TextEdit> {
        let range = self.delete.into_proto(source, encoding);
        let new_text = match ending {
            LineEnding::Lf => self.insert,
            LineEnding::Crlf => self.insert.replace('\n', "\r\n"),
            LineEnding::Cr => self.insert.replace('\n', "\r"),
        };
        Ok(lsp_types::TextEdit { range, new_text })
    }
}
