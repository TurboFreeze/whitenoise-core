use whitenoise_validator::errors::*;

use crate::base::NodeArguments;
use whitenoise_validator::base::{Value, Array};
use crate::components::Evaluable;
use ndarray::{ArrayD, Axis};
use ndarray;
use whitenoise_validator::proto;
use whitenoise_validator::utilities::get_argument;


impl Evaluable for proto::Count {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<Value> {
        Ok(match get_argument(arguments, "data")?.array()? {
            Array::Bool(data) => count(data)?.into(),
            Array::F64(data) => count(data)?.into(),
            Array::I64(data) => count(data)?.into(),
            Array::Str(data) => count(data)?.into()
        })
    }
}

/// Gets number of rows of data.
///
/// # Arguments
/// * `data` - Data for which you want a count.
///
/// # Return
/// Number of rows in data.
///
/// # Example
/// ```
/// use ndarray::{ArrayD, arr1, arr2};
/// use whitenoise_runtime::components::count::count;
/// let data = arr2(&[ [false, false, true], [true, true, true] ]).into_dyn();
/// let n = count(&data).unwrap();
/// assert!(n.first().unwrap() == &2);
/// ```
pub fn count<T>(data: &ArrayD<T>) -> Result<ArrayD<i64>> {
    Ok(ndarray::Array::from_shape_vec(vec![], vec![data.len_of(Axis(0)) as i64])?)
}
