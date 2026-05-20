use anyhow::{Context, Result};
use std::collections::VecDeque;

use crate::distributing_iterator;
use csv::ByteRecord;

pub fn distribute(data: &str, field: &str, spread: u64) -> Result<String> {
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(data.as_bytes());
    let headers = csv.headers()?.clone();
    let field_index = headers
        .iter()
        .position(|header| header == field)
        .context(format!("field `{field}` not found in CSV headers"))?;
    let data = csv
        .into_byte_records()
        .map(|record| record.map_err(anyhow::Error::from))
        .collect::<Result<VecDeque<_>>>()?;
    let id_func = move |item: &ByteRecord| item[field_index].to_vec();
    let iterator = distributing_iterator::DistributingIterator::new(data, spread as usize, id_func);
    let data: Vec<_> = iterator.collect();
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&headers).context("writing headers")?;
    for record in data {
        wtr.write_byte_record(&record).context("writing record")?;
    }
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute() {
        let data = "id,name\n1,foo\n1,bar\n1,baz\n2,qux\n3,quux\n3,corge\n2,grault\n2,garply\n3,waldo\n3,fred\n2,plugh\n3,xyzzy\n2,thud\n3,plugh\n3,xyzzy\n"
            .to_string();
        let result = distribute(&data, "id", 3).unwrap();
        assert_eq!(
            result,
            "id,name\n1,foo\n2,qux\n3,quux\n1,bar\n2,grault\n3,corge\n1,baz\n2,garply\n3,waldo\n2,plugh\n3,fred\n2,thud\n3,xyzzy\n3,plugh\n3,xyzzy\n"
        );
    }
}
