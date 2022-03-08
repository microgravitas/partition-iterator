# Partition iterators

Iterator adaptors that enumerate all partitions or all k-partitions of an iterator's content.
The iterator is based on a <a href="https://codereview.stackexchange.com/questions/1526/finding-all-k-subset-partitions">
Python implementation</a> which generate Ruskey Gray codes for so-called "restricted growth sequences"
(which biject onto set partitions). As a result, the memory footprint is very low.

How to use with Cargo:

```toml
[dependencies]
partition-iterator = "0.1.0"
```

Example usage:

```rust
use partition_iterator::PartitionIter;

fn main() {
    let ls = vec!['a', 'b', 'c', 'd'];

    for a in ls.iter().kpartitions(3) {
        println!("{:?}", a);
    }
}
```

Output:
```
    [['a', 'b'], ['c'], ['d']]
    [['a'], ['b', 'c'], ['d']]
    [['a', 'c'], ['b'], ['d']]
    [['a'], ['b'], ['c', 'd']]
    [['a'], ['b', 'd'], ['c']]
    [['a', 'd'], ['b'], ['c']]
```

## License

Licensed under MIT license https://opensource.org/licenses/MIT.
