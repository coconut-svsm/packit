// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Carlos López <carlos.lopez@suse.com

use crate::{PackItFile, PackItHeader, PackItResult};
use memmap2::Mmap;
use std::io::Write;

/// A lazy `PackIt` archive encoder.
pub struct PackItArchiveEncoder<'a, W> {
    hdr: PackItHeader,
    writer: &'a mut W,
}

impl<'a, W: Write> PackItArchiveEncoder<'a, W> {
    /// Create a new encoder with the default header
    ///
    /// # Errors
    ///
    /// Returns an error if writing to `writer` fails.
    pub fn new(writer: &'a mut W) -> PackItResult<Self> {
        let hdr = PackItHeader::new();
        Self::with_header(hdr, writer)
    }

    /// Create a a new encoder with the specified header
    ///
    /// # Errors
    ///
    /// Returns an error if writing to `writer` fails.
    pub fn with_header(hdr: PackItHeader, writer: &'a mut W) -> PackItResult<Self> {
        hdr.write(writer)?;
        Ok(Self { hdr, writer })
    }

    /// Get the archive header
    pub const fn header(&self) -> PackItHeader {
        self.hdr
    }

    /// Write a single file to the archive
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer provided in the constructor fails.
    pub fn write_file(&mut self, file: &PackItFile<'_>) -> PackItResult<()> {
        file.write(self.writer)
    }

    /// Load the specified file from disk into the archive. This is a higher
    /// level method than [`write_file`](Self::write_file).
    pub fn load_file(&mut self, name: &str, file: &std::fs::File) -> PackItResult<()> {
        let meta = file.metadata()?;
        match meta.len() {
            0 => {
                // Special case, a zero-length mapping will fail
                let pfile = PackItFile::new(name, &[])?;
                self.write_file(&pfile)?;
            }
            _ => {
                // SAFETY: assume user-provided file is not being modified.
                // Worst case scenario, we read the wrong bytes.
                let file_data = unsafe { Mmap::map(file) }?;
                let pfile = PackItFile::new(name, &file_data)?;
                self.write_file(&pfile)?;
            }
        }
        Ok(())
    }
}
