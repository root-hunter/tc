# ran - Rust Archive Node

### Data Binary Structure
```rust
pub struct Data<E, C = u16> {
    dict: HashMap<E, Vec<usize>>,
    elements: Vec<C>
}
```

### Data Binary Structure

- D = dictionary size
- K = offset to dict keys array = 5
- V = offset to dict values array = 5 + D
- S = offset to dict keys elements = V + D

| Byte Offset | Field Name     | Description               | Value Example |
|-------------|----------------|---------------------------|---------------|
| 0-3         | Dict size      | Dictionary size           | 0x00000010    |
| 1-4         | Elements size  | Number of elements        | 0x00000020    |
| K-(V-1)         | Dict keys      | Timestamp of the packet    | 0x5F8D5D8A    |
| V-(S-1)        | Payload        | Data being transmitted     | [Data Block]  |
| S-EOF          | Checksum       | Checksum for integrity     | 0xABCDEF01    |
