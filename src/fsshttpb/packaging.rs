use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::fsshttpb::data::object_types::ObjectType;
use crate::fsshttpb::data::stream_object::ObjectHeader;
use crate::fsshttpb::data_element::DataElementPackage;
use crate::onestore::file::FileHeader;
use crate::shared::guid::Guid;
use crate::Reader;

/// A OneNote file packaged in FSSHTTPB format.
///
/// See [\[MS-ONESTORE\] 2.8.1]
///
/// [\[MS-ONESTORE\] 2.8.1]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-onestore/a2f046ea-109a-49c4-912d-dc2888cf0565
#[derive(Debug)]
pub(crate) struct OneStorePackaging {
    pub(crate) header_guids: FileHeader,
    pub(crate) storage_index: ExGuid,
    pub(crate) cell_schema: Guid,
    pub(crate) data_element_package: DataElementPackage,
}

#[derive(PartialEq)]
pub(crate) enum CellSchemaId {
    Notebook,
    Section,
}

impl OneStorePackaging {
    pub(crate) fn parse(reader: Reader, header_guids: FileHeader) -> Result<OneStorePackaging> {

        if reader.get_u32()? != 0 {
            return Err(ErrorKind::MalformedFssHttpBData("invalid padding data".into()).into());
        }

        ObjectHeader::try_parse_32(reader, ObjectType::OneNotePackaging)?;

        let storage_index = ExGuid::parse(reader)?;
        let cell_schema = Guid::parse(reader)?;

        let data_element_package = DataElementPackage::parse(reader)?;

        ObjectHeader::try_parse_end_16(reader, ObjectType::OneNotePackaging)?;

        Ok(OneStorePackaging {
            header_guids,
            storage_index,
            cell_schema,
            data_element_package,
        })
    }

    pub(crate) fn determine_format(&self) -> Result<CellSchemaId> {
        let onenote_package_section_guid = guid!({1F937CB4-B26F-445F-B9F8-17E20160E461});
        let onenote_package_notebook_guid = guid!({E4DBFD38-E5C7-408B-A8A1-0E7B421E1F5F});

        if self.cell_schema == onenote_package_notebook_guid {
            Ok(CellSchemaId::Notebook)
        } else if self.cell_schema == onenote_package_section_guid {
            Ok(CellSchemaId::Section)
        } else {
            Err(ErrorKind::UnknownFileType {
                guid: self.cell_schema.to_string(),
            }
            .into())
        }
    }
}
