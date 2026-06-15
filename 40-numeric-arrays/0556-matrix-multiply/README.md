# 0556 — Matrix multiply

Matrix multiplication with the `ndarray` crate. Two integer `Array2<i32>` matrices `[[1,2],[3,4]]` and `[[5,6],[7,8]]` are multiplied with `a.dot(&b)`, which performs the dot-product matmul. Each resulting row is printed as space-joined integers, computed entirely by the library.

## Run

    cargo run
