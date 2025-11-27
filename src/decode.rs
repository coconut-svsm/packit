// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Carlos López <carlos.lopez@suse.com

use crate::{PackItFile, PackItHeader, PackItResult};

/// A lazy raw PackIt archive decoder.
#[derive(Clone, Copy, Debug)]
pub struct PackItArchiveDecoder<'a> {
    hdr: PackItHeader,
    raw_data: &'a [u8],
}

impl<'a> PackItArchiveDecoder<'a> {
    /// The archive header
    pub const fn header(&self) -> PackItHeader {
        self.hdr
    }

    /// Load an archive from an existing blob
    pub fn load(raw_data: &'a [u8]) -> PackItResult<Self> {
        let (hdr, raw_data) = PackItHeader::load(raw_data)?;
        Ok(Self { hdr, raw_data })
    }
}

/// Iterate over packed [`PackItFile`] files.
impl<'a> core::iter::Iterator for PackItArchiveDecoder<'a> {
    type Item = PackItResult<PackItFile<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.raw_data.is_empty() {
            return None;
        }

        match PackItFile::load(self.raw_data) {
            Ok((f, rest)) => {
                self.raw_data = rest;
                Some(Ok(f))
            }
            Err(e) => {
                // Stop iterating
                self.raw_data = &[];
                Some(Err(e))
            }
        }
    }
}
