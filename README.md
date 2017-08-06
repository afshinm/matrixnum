# Matrixnum

Makes it easier to work with Matrix in Rust

## Example

```rust
use matrixnum::basic::BasicMatrix;
use matrixnum::matrix_trait::MatrixTrait;

let matrix = BasicMatrix::generate(2, 2, &|_,_| 0f64); // 2 x 2 matrix

println!("{}", matrix.rows()); // 2
println!("{}", matrix.cols()); // 2
```

## Author

Afshin Mehrabani (afshin.meh@gmail.com)

## License

GPLv3
