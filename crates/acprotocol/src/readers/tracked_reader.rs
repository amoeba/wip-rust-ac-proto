use crate::ParsingContext;
use crate::readers::traits::ACReader;
use std::io::{self, Read, Seek, SeekFrom};

/// A wrapper around ACReader that tracks parsing context
pub struct TrackedReader<'a> {
    inner: &'a mut dyn ACReader,
    context: &'a mut ParsingContext,
    position: usize,
}

impl<'a> TrackedReader<'a> {
    pub fn new(inner: &'a mut dyn ACReader, context: &'a mut ParsingContext) -> Self {
        Self {
            inner,
            context,
            position: 0,
        }
    }

    /// Record a warning at the current position
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.context.warn(msg, Some(self.position));
    }

    /// Record an error at the current position
    pub fn error(&mut self, msg: impl Into<String>) {
        self.context.error(msg, Some(self.position));
    }

    /// Record a fatal error at the current position
    pub fn fatal(&mut self, msg: impl Into<String>) {
        self.context.fatal(msg, Some(self.position));
    }

    /// Get the current position
    pub fn position(&self) -> usize {
        self.position
    }
}

impl<'a> Read for TrackedReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.position += n;
        Ok(n)
    }
}

impl<'a> Seek for TrackedReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let result = self.inner.seek(pos)?;
        self.position = result as usize;
        Ok(result)
    }
}

// TrackedReader implements Read + Seek, so it automatically implements ACReader
// via the blanket impl in traits.rs
