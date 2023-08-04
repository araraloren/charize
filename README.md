# charize

Provide a simple proc macro `charize` convert token to char.

## Usage

```rust
pub fn main() {
    assert_eq!('o', charize::charize!(o));
    assert_eq!('1', charize::charize!(1));
    assert_eq!('我', charize::charize!(我));
    assert_eq!('P', charize::charize!(P));
    assert_eq!('@', charize::charize!(@));
    assert_eq!('&', charize::charize!(&));
}
```

## LICENSE

MPL-2.0