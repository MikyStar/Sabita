use crate::assets::full_grid::ConstGridValues;
use crate::core::grid::GridValues;

////////////////////////////////////////

pub fn grid_values_array_to_vec(val: ConstGridValues) -> GridValues {
    let mut to_return = vec![];

    for line in val {
        to_return.push(line.to_vec());
    }

    to_return
}
