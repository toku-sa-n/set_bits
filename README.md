# set\_bits

set\_bits is a Rust library for setting and clearing bits on memory.

## Usage

Add `set_bits` to your `Cargo.toml` file.

```rust
use set_bits;

// Set 20 bits from the 10th bit of address 0xCAFEBABE
set_bits::set(0xCAFEBABE, 10, 20);

// Clear 20 bits from the 10th bit of address 0xCAFEBABE
set_bits::clear(0xCAFEBABE, 10, 20);
```

## License
[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/) or [MIT](https://choosealicense.com/licenses/mit/)
