use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;

use crate::file_patterns::FilePatterns;

#[derive(Debug, Clone)]
pub struct ExcludePatterns(FilePatterns);

impl ExcludePatterns {
    pub(crate) fn try_from_iter<'str, P, I>(root: P, patterns: I) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = &'str str>,
    {
        Ok(Self(FilePatterns::try_from_iter(root, patterns)?))
    }
}

impl Deref for ExcludePatterns {
    type Target = FilePatterns;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExcludePatterns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
