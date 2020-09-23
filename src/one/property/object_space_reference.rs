use crate::one::property::PropertyType;
use crate::onestore::object::Object;
use crate::onestore::types::compact_id::CompactId;
use crate::onestore::types::property::{PropertyId, PropertyValue};
use crate::types::exguid::ExGuid;

pub(crate) struct ObjectSpaceReference;

impl ObjectSpaceReference {
    pub(crate) fn parse_vec(prop_type: PropertyType, object: &Object) -> Option<Vec<ExGuid>> {
        object
            .props()
            .get(prop_type)
            .map(|value| {
                value
                    .to_object_space_ids()
                    .expect("object space reference array is not a object space array")
            })
            .map(|count| {
                object
                    .props()
                    .object_space_ids()
                    .iter()
                    .skip(Self::get_offset(prop_type, object))
                    .take(count as usize)
                    .map(|id| Self::resolve_id(id, object))
                    .collect()
            })
    }

    fn get_offset(prop_type: PropertyType, object: &Object) -> usize {
        let prop_index = object
            .props()
            .properties()
            .index(PropertyId::new(prop_type as u32))
            .unwrap();

        Self::count_references(
            object
                .props()
                .properties()
                .values_with_index()
                .filter(|(idx, _)| *idx < prop_index)
                .map(|(_, value)| value),
        )
    }

    fn count_references<'a>(props: impl Iterator<Item = &'a PropertyValue>) -> usize {
        props
            .map(|v| match v {
                PropertyValue::ObjectSpaceId => 1,
                PropertyValue::ObjectSpaceIds(c) => *c as usize,
                PropertyValue::PropertyValues(_, sets) => sets
                    .iter()
                    .map(|set| Self::count_references(set.values()))
                    .sum(),
                _ => 0,
            })
            .sum()
    }

    fn resolve_id(id: &CompactId, object: &Object) -> ExGuid {
        object
            .mapping()
            .get_object(*id)
            .expect("id not defined in mapping")
    }
}
