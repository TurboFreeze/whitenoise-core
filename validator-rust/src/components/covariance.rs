use crate::errors::*;


use std::collections::HashMap;

use crate::{proto, base};

use crate::components::{Component, Aggregator};
use crate::base::{Value, Properties, NodeProperties, AggregatorProperties, Sensitivity};

impl Component for proto::Covariance {
    // modify min, max, n, categories, is_public, non-null, etc. based on the arguments and component
    fn propagate_property(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        _public_arguments: &HashMap<String, Value>,
        properties: &base::NodeProperties,
    ) -> Result<Properties> {

        if properties.contains_key("data") {
            let mut data_property = properties.get("data").unwrap().clone();
            data_property.assert_is_not_aggregated()?;

            // save a snapshot of the state when aggregating
            data_property.aggregator = Some(AggregatorProperties {
                component: proto::component::Variant::from(self.clone()),
                properties: properties.clone()
            });

            match data_property.num_columns {
                // number of rows is known if number of columns is known
                Some(num_columns) =>
                    data_property.num_records = (0..num_columns).map(|v| Some(v + 1)).collect(),

                // else number of rows is not known
                None =>
                    data_property.num_records = Vec::new()
            }

            // min/max of data is not known after computing covariance
            data_property.nature = None;
            return Ok(data_property);

        } else if properties.contains_key("left") && properties.contains_key("right") {
            let mut left_property = properties.get("left")
                .ok_or("left must be passed for cross-covariance")?.clone();
            left_property.assert_is_not_aggregated()?;

            let mut right_property = properties.get("right")
                .ok_or("right must be passed for cross-covariance")?.clone();
            right_property.assert_is_not_aggregated()?;

            // save a snapshot of the state when aggregating
            left_property.aggregator = Some(AggregatorProperties {
                component: proto::component::Variant::from(self.clone()),
                properties: properties.clone()
            });

            left_property.nature = None;
            left_property.releasable = left_property.releasable && right_property.releasable;

            return Ok(left_property);

        } else {
            return Err("either \"data\" for covariance, or \"left\" and \"right\" for cross-covariance must be supplied".into())
        }
    }

    fn get_names(
        &self,
        _properties: &NodeProperties,
    ) -> Result<Vec<String>> {
        Err("get_names not implemented".into())
    }
}

impl Aggregator for proto::Covariance {
    fn compute_sensitivity(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        properties: &NodeProperties,
        sensitivity_type: &Sensitivity
    ) -> Result<Vec<f64>> {
        let mut left_property = properties.get("left")
            .ok_or("left argument missing from DPCovariance")?.clone();


        match sensitivity_type {
            Sensitivity::KNorm(k) => {
                if k != &1 {
                    return Err("Covariance sensitivity is only implemented for KNorm of 1".into())
                }

                // check that all properties are satisfied
                println!("covariance left");
                let left_n = left_property.get_n()?;
                left_property.get_min_f64()?;
                left_property.get_max_f64()?;
                left_property.assert_non_null()?;

                let right_property = properties.get("right")
                    .ok_or("right argument missing from DPCovariance")?.clone();

                // check that all properties are satisfied
                println!("covariance right");
                let right_n = right_property.get_n()?;
                right_property.get_min_f64()?;
                right_property.get_max_f64()?;
                right_property.assert_non_null()?;

                if !left_n.iter().zip(right_n).all(|(left, right)| left == &right) {
                    return Err("n for left and right must be equivalent".into());
                }

                // TODO: derive proper propagation of covariance property
                left_property.num_records = (0..left_property.num_columns.unwrap()).map(|_| Some(1)).collect();
                left_property.releasable = true;

                // TODO: cross-covariance
                let data_property = properties.get("data")
                    .ok_or::<Error>("data must be passed to compute sensitivity".into())?;

                let min = data_property.get_min_f64()?;
                let max = data_property.get_max_f64()?;

                Ok(min.iter()
                    .zip(max)
                    .map(|(min, max)| (max - min) as f64)
                    .collect())
            },
            _ => return Err("Covariance sensitivity is only implemented for KNorm of 1".into())
        }
    }
}