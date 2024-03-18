# distributing-iterator
Distributing iterator

[![Rust](https://github.com/fetlife/distributing-iterator/actions/workflows/rust.yml/badge.svg)](https://github.com/fetlife/distributing-iterator/actions/workflows/rust.yml)

# Synopsis

## Rust

```rust
use distributing_iterator::DistributingIterator;

#[derive(Debug, PartialEq)]
struct Item {
    id: u64,
}

let data = vec![
    Item { id: 1 },
    Item { id: 1 },
    Item { id: 1 },
    Item { id: 2 },
    Item { id: 2 },
    Item { id: 2 },
    Item { id: 3 },
    Item { id: 3 },
    Item { id: 3 },
];
let iterator = DistributingIterator::new(data.into(), 3, |item| item.id);
let result: Vec<Item> = iterator.collect();
assert_eq!(
    result,
    vec![
        Item { id: 1 }, Item { id: 2 }, Item { id: 3 },
        Item { id: 1 }, Item { id: 2 }, Item { id: 3 },
        Item { id: 1 }, Item { id: 2 }, Item { id: 3 },
    ]
);
```


## As a ruby gem:

Gemfile:

```ruby
gem 'distributing_iterator', git: 'fetlife/distributing-iterator'
```

```ruby
require 'distributing_iterator'

csv = <<~CSV
id,name
1,foo
1,bar
1,baz
2,qux
3,quux
3,corge
2,grault
2,garply
3,waldo
3,fred
2,plugh
3,xyzzy
2,thud
3,plugh
3,xyzzy
CSV

output = DistributingIterator.distribute(csv, 'id', 3)

puts output
# id,name
# 1,foo
# 2,qux
# 3,quux
# 1,bar
# 2,grault
# 3,corge
# 1,baz
# 2,garply
# 3,waldo
# 2,plugh
# 3,fred
# 2,thud
# 3,xyzzy
# 3,plugh
# 3,xyzzy

```
