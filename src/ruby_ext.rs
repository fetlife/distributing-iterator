use magnus::{error::Result, function, exception, Error};

use crate::distribute_csv;

fn distribute_csv_ruby(data: String, field: String, spread: u64) -> Result<String> {
    match distribute_csv(&data, &field, spread) {
        Ok(result) => Ok(result),
        Err(e) => {
            Err(Error::new(exception::standard_error(), format!("{:?}", e)))
        }
    }
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<()> {
    let module = ruby.define_module("DistributingIterator")?;
    module.define_module_function("distribute", function!(distribute_csv_ruby, 3))?;
    Ok(())
}
