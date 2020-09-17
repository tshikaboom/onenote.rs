use crate::fsshttpb::data_element::value::DataElementValue;
use crate::types::exguid::ExGuid;
use crate::types::object_types::ObjectType;
use crate::types::stream_object::ObjectHeader;
use crate::Reader;

impl DataElementValue {
    pub(crate) fn parse_cell_manifest(reader: Reader) -> DataElementValue {
        let object_header = ObjectHeader::parse_16(reader);
        assert_eq!(object_header.object_type, ObjectType::CellManifest);

        let id = ExGuid::parse(reader);

        assert_eq!(ObjectHeader::parse_end_8(reader), ObjectType::DataElement);

        DataElementValue::CellManifest(id)
    }
}
