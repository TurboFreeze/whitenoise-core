use crate::errors::*;

use std::collections::HashMap;

use crate::{proto, base};

use crate::components::{Component};
use crate::base::{Hashmap, Value, NodeProperties, ValueProperties, HashmapProperties, ArrayProperties, DataType};
use crate::utilities::serial::parse_i64_null;

impl Component for proto::Materialize {
    // modify min, max, n, categories, is_public, non-null, etc. based on the arguments and component
    fn propagate_property(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        public_arguments: &HashMap<String, Value>,
        _properties: &base::NodeProperties,
    ) -> Result<ValueProperties> {

        let column_names = public_arguments.get("column_names")
            .and_then(|column_names| column_names.array().ok()?.string().ok()).cloned();
        let num_columns = public_arguments.get("num_columns")
            .and_then(|num_columns| num_columns.array().ok()?.first_i64().ok());

        Ok(HashmapProperties {
            num_records: None,
            disjoint: false,
            properties: match (column_names, num_columns) {
                (Some(column_names), _) => Hashmap::<ValueProperties>::Str(column_names.iter()
                    .map(|name| (name.clone(), ValueProperties::Array(ArrayProperties {
                        num_records: None,
                        num_columns: Some(1),
                        nullity: true,
                        releasable: !self.private,
                        c_stability: vec![1.],
                        aggregator: None,
                        nature: None,
                        data_type: DataType::Str,
                        dataset_id: self.dataset_id.as_ref().and_then(parse_i64_null)
                    }))).collect()),
                (None, Some(num_columns)) => Hashmap::<ValueProperties>::I64((0..num_columns)
                    .map(|name| (name.clone(), ValueProperties::Array(ArrayProperties {
                        num_records: None,
                        num_columns: Some(1),
                        nullity: true,
                        releasable: !self.private,
                        c_stability: vec![1.],
                        aggregator: None,
                        nature: None,
                        data_type: DataType::Str,
                        dataset_id: self.dataset_id.as_ref().and_then(parse_i64_null)
                    }))).collect()),
                _ => return Err("either column_names or num_columns must be specified".into())
            },
            columnar: true
        }.into())
    }

    fn get_names(
        &self,
        _properties: &NodeProperties,
    ) -> Result<Vec<String>> {
        Err("get_names not implemented".into())
    }
}
