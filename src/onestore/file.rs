use crate::errors::{ErrorKind, Result};
use crate::shared::guid::Guid;
use crate::Reader;

use cfg_log::debug;

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
