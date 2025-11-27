// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Carlos López <carlos.lopez@suse.com

use crate::{PackItError, PackItResult};
use core::mem::size_of;
#[cfg(feature = "std")]
use std::io::Write;
use zerocopy::byteorder::LittleEndian;
use zerocopy::{FromBytes, Immutable, IntoBytes, U32};

/// Header Magic (PKIT)
pub const PACKIT_MAGIC: [u8; 4] = [0x50, 0x4b, 0x49, 0x54];

/// A PackIt archive header
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
pub struct PackItHeader {
    magic: [u8; 4],
    header_size: U32<LittleEndian>,
}

impl PackItHeader {
    #[allow(dead_code)]
    pub(crate) const fn new() -> Self {
        Self {
            magic: PACKIT_MAGIC,
            header_size: U32::new(size_of::<Self>() as u32),
        }
    }

    pub(crate) fn load(data: &[u8]) -> PackItResult<(Self, &[u8])> {
        let (header, rest) =
            Self::read_from_prefix(data).map_err(|_| PackItError::UnexpectedEOF)?;

        if header.magic != PACKIT_MAGIC {
            return Err(PackItError::InvalidHeader);
        }

        Ok((header, rest))
    }

    #[cfg(feature = "std")]
    pub(crate) fn write<W: Write>(&self, dst: &mut W) -> PackItResult<()> {
        dst.write_all(self.as_bytes()).map_err(PackItError::IoError)
    }

    /// The size of the archive header
    pub const fn header_size(&self) -> u32 {
        self.header_size.get()
    }
}

impl Default for PackItHeader {
    fn default() -> Self {
        Self::new()
    }
}
