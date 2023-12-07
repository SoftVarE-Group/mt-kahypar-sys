# mt-kahypar-sys

> Rust bindings to [Mt-KaHyPar][mtkahypar].

This Rust crate links the C interface of [Mt-KaHyPar][mtkahypar].

## Usage

Add it as a dependency to your Cargo.toml:

```toml
[dependencies]
mt-kahypar-sys = "0.1"
```

## Requirements

 - [Mt-KaHyPar][mtkahypar]

**Note:** Currently, Mt-KaHyPar can not be built statically.
Therefore, bundling it with the crate is not possible and the shared library has to be present.

### Using existing Mt-KaHyPar

When using an already built version of Mt-KaHyPar (currently required), the environment variable `MTKAHYPAR_DIR` might be used to point to a directory containing it.
This directory must contain `include` and `lib` or `lib64` subdirectories with header and library files respectively.

## FFI

Currently, this crate only builds and links the C interface.
No FFI functionality is available right now.

[mtkahypar]: https://github.com/kahypar/mt-kahypar
