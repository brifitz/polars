use arrow::array::{MutableArray, MutableBinaryViewArray, ViewType};
use polars_error::PolarsResult;

use crate::parquet::statistics::BinaryStatistics;

pub(super) fn push<T: ViewType + ?Sized>(
    from: Option<&BinaryStatistics>,
    min: &mut dyn MutableArray,
    max: &mut dyn MutableArray,
) -> PolarsResult<()> {
    let min = min
        .as_mut_any()
        .downcast_mut::<MutableBinaryViewArray<T>>()
        .unwrap();
    let max = max
        .as_mut_any()
        .downcast_mut::<MutableBinaryViewArray<T>>()
        .unwrap();

    min.push(from.and_then(|s| {
        let opt_b = s.min_value.as_deref();
        opt_b.and_then(T::from_bytes)
    }));
    max.push(from.and_then(|s| {
        let opt_b = s.max_value.as_deref();
        opt_b.and_then(T::from_bytes)
    }));

    Ok(())
}
