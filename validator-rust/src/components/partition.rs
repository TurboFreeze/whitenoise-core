use crate::errors::*;


use std::collections::HashMap;

use crate::{proto, base};

use crate::components::{Component};
use crate::base::{Value, Jagged, NodeProperties, ValueProperties, HashmapProperties, ArrayProperties};
use crate::utilities::prepend;


impl Component for proto::Partition {
    fn propagate_property(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        public_arguments: &HashMap<String, Value>,
        properties: &base::NodeProperties,
    ) -> Result<ValueProperties> {
        let mut data_property = properties.get("data")
            .ok_or("data: missing")?.array()
            .map_err(prepend("data:"))?.clone();

        Ok(match properties.get("by") {
            Some(by_property) => {
                let by_property = by_property.array()
                    .map_err(prepend("by:"))?.clone();
                let by_num_columns= by_property.num_columns
                    .ok_or::<Error>("number of columns must be known on by".into())?;
                if by_num_columns != 1 {
                    return Err("Partition's by argument must contain a single column".into());
                }
                let categories = by_property.categories()
                    .map_err(prepend("by:"))?;
                data_property.num_records = None;

                HashmapProperties {
                    num_records: data_property.num_records,
                    disjoint: true,
                    properties: match categories {
                        Jagged::Bool(categories) => broadcast_partitions(&categories, &data_property)?.into(),
                        Jagged::Str(categories) => broadcast_partitions(&categories, &data_property)?.into(),
                        Jagged::I64(categories) => broadcast_partitions(&categories, &data_property)?.into(),
                        _ => return Err("partitioning based on floats is not supported".into())
                    },
                    columnar: false
                }
            },
            None => {

                let num_partitions = public_arguments.get("num_partitions")
                    .ok_or("num_partitions or by must be passed to Partition")?.array()?.first_i64()?;

                let lengths = match data_property.num_records {
                    Some(num_records) => (0..num_partitions)
                        .map(|index| Some(num_records / num_partitions + (if index > (num_records % num_partitions) {0} else {1})))
                        .collect::<Vec<Option<i64>>>(),
                    None => (0..num_partitions)
                        .map(|_| None)
                        .collect::<Vec<Option<i64>>>()
                };

                HashmapProperties {
                    num_records: data_property.num_records,
                    disjoint: false,
                    properties: lengths.iter().enumerate().map(|(index, partition_num_records)| {
                        let mut partition_property = data_property.clone();
                        partition_property.num_records = partition_num_records.clone();
                        (index as i64, ValueProperties::Array(partition_property))
                    }).collect::<HashMap<i64, ValueProperties>>().into(),
                    columnar: false
                }
            }
        }.into())
    }

    fn get_names(
        &self,
        _properties: &NodeProperties,
    ) -> Result<Vec<String>> {
        Err("get_names not implemented".into())
    }
}

pub fn broadcast_partitions<T: Clone + Eq + std::hash::Hash>(
    categories: &Vec<Option<Vec<T>>>, properties: &ArrayProperties
) -> Result<HashMap<T, ValueProperties>> {

    if categories.len() != 1 {
        return Err("categories: must be defined for one column".into())
    }
    let partitions = categories[0].clone()
        .ok_or::<Error>("categories: must be defined".into())?;
    Ok(partitions.iter()
        .map(|v| (v.clone(), ValueProperties::Array(properties.clone())))
        .collect())
}