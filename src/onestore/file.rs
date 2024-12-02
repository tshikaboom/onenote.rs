use crate::errors::{ErrorKind, Result};
use crate::onestore::types::file_chunk_reference;
use crate::shared::guid::Guid;
use crate::Reader;

use cfg_log::debug;

use self::file_chunk_reference::{FileChunkReference32, FileChunkReference64x32};

/// Part of a OneStore file's header data.
/// This is shared between the normal file header format and the FSSHTTPB format.
///
/// See [\[MS-ONESTORE\] 2.3.1]
///
/// [\[MS-ONESTORE\] 2.3.1]: https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-onestore/2b394c6b-8788-441f-b631-da1583d772fd

#[derive(Debug)]
pub(crate) struct FileHeader {
    pub(crate) file_type: Guid,
    pub(crate) file: Guid,
    pub(crate) legacy_file_version: Guid,
    pub(crate) file_format: Guid,
}

pub(crate) struct OneStoreHeader {
    pub(crate) header_guids: FileHeader,
    pub(crate) last_code_version_that_wrote_to_it: u32,
    pub(crate) oldest_code_version_that_wrote_to_it: u32,
    pub(crate) newest_code_version_that_wrote_to_it: u32,
    pub(crate) oldest_code_version_that_can_read_it: u32,
    pub(crate) fcr_legacy_free_chunk_list: FileChunkReference32,
    pub(crate) fcr_legacy_transaction_log: FileChunkReference32,
    pub(crate) transactions_in_log: u32,
    pub(crate) legacy_expected_file_length: u32,
    pub(crate) rgb_placeholder: u64,
    pub(crate) fcr_legacy_file_node_list_root: FileChunkReference32,
    pub(crate) legacy_free_space_in_chunk_list: u32,
    pub(crate) needs_defrag: u8,
    pub(crate) repaired_file: u8,
    pub(crate) needs_garbage_collect: u8,
    pub(crate) has_no_embedded_objects: u8,
    pub(crate) guid_ancestor: Guid,
    pub(crate) file_name_crc: u32,
    pub(crate) fcr_hashed_chunk_list: FileChunkReference64x32,
    pub(crate) fcr_transaction_log: FileChunkReference64x32,
    pub(crate) fcr_root_file_node_list: FileChunkReference64x32,
    pub(crate) fcr_free_chunk_list: FileChunkReference64x32,
    pub(crate) expected_file_length: u64,
    pub(crate) free_space_in_free_chunk_list: u64,
    pub(crate) file_version: Guid,
    pub(crate) times_this_file_changed: u64,
    pub(crate) guid_deny_read_file_version: Guid,
    pub(crate) debug_log_flags: u32,
    pub(crate) fcr_debug_log: FileChunkReference64x32,
    pub(crate) fcr_alloc_verification_free_chunk_list: FileChunkReference64x32,
    pub(crate) bn_created_this: u32,
    pub(crate) bn_last_wrote_to_this: u32,
    pub(crate) bn_oldest_wrote_to_this: u32,
    pub(crate) bn_newest_wrote_to_this: u32,
    // rgbReserved, 728 bytes
}

#[derive(Debug, PartialEq)]
pub(crate) enum HeaderGuids {
    OneNotePackageStore,
    OneNoteRevisionStore,
}

pub(crate) fn determine_format(file_format: Guid) -> Result<HeaderGuids> {
    let package_store_format = Guid::from_str("638DE92F-A6D4-4bc1-9A36-B3FC2511A5B7")?;
    let revision_store_format = Guid::from_str("109ADD3F-911B-49F5-A5D0-1791EDC8AED8")?;

    if file_format == revision_store_format {
        Ok(HeaderGuids::OneNoteRevisionStore)
    } else if file_format == package_store_format {
        Ok(HeaderGuids::OneNotePackageStore)
    } else {
        Err(ErrorKind::UnknownFileType {
            guid: file_format.to_string(),
        }
        .into())
    }
}

impl HeaderGuids {
    pub(crate) fn parse(reader: Reader) -> Result<FileHeader> {
        let file_type = Guid::parse(reader)?;
        let file = Guid::parse(reader)?;
        let legacy_file_version = Guid::parse(reader)?;
        let file_format = Guid::parse(reader)?;

        debug!("guidFileType\t\t{:?}", &file_type);
        debug!("guidLegacyFileVersion\t{:?}", &legacy_file_version);
        debug!("guidFileFormat\t\t{:?}", &file_format);

        Ok(FileHeader {
            file_type,
            file,
            legacy_file_version,
            file_format,
        })
    }
}
