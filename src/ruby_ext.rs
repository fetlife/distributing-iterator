use magnus::{error::Result, exception, function, Error};

use crate::{distribute_ids, distribute_csv};

fn distribute_csv_ruby(data: String, field: String, spread: u64) -> Result<String> {
    match distribute_csv(&data, &field, spread) {
        Ok(result) => Ok(result),
        Err(e) => Err(Error::new(exception::standard_error(), format!("{:?}", e))),
    }
}

fn distribute_indexes_ruby(data: Vec<String>, spread: usize) -> Result<Vec<usize>> {
    Ok(distribute_ids(&data, spread))
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<()> {
    let module = ruby.define_module("DistributingIterator")?;
    module.define_module_function("distribute_csv", function!(distribute_csv_ruby, 3))?;
    module.define_module_function("distribute_indexes", function!(distribute_indexes_ruby, 2))?;
    Ok(())
}
