[phnt][github.com]: Rust bindings to the [System Informer][github.com/sysinf]'s [phnt][docs.rs]
========================================

[![phnt GitHub Actions][github.com/ci/img]][github.com/ci]
[![phnt on crates.io][crates.io/img]][crates.io]
[![phnt on docs.rs][docs.rs/img]][docs.rs]

This crate provides Rust with access to the **[System Informer][github.com/sysinf]**'s (formerly known as **Process Hacker**) [native Windows headers][github.com/sysinf/phnt] (shortened to **phnt**) which provide type definitions, constants, macros as well as function prototypes to even undocumented functions and syscalls.

| Feature | Description |
| --- | --- |
| [`regenerate`][github.com/src/build.rs] | Regenerated bindings from [phnt (nightly)][github.com/phnt_nightly] source code. |
| *`(default)`* | A vendored version of pre-generated bindings are used by default to improve build times. |

Crate Overview
- [`ffi`][docs.rs/ffi] Bindings for [phnt (nightly)][github.com/phnt_nightly] generated by [bindgen][crates.io/bindgen]
  - [Re-exports][docs.rs/ffi/reexports]
  - [Structs][docs.rs/ffi/structs]
  - [Enums][docs.rs/ffi/enums]
  - [Statics][docs.rs/ffi/structs]
  - [Functions][docs.rs/ffi/functions]
  - [Type Aliases][docs.rs/ffi/type-aliases]
  - [Unions][docs.rs/ffi/unions]

- [`ext`][docs.rs/ext] Extensions to the bindings (useful functions, macros, etc.)
  - [Functions][docs.rs/ext/functions]
  - [Macros][docs.rs/ext/macros]

**crate version:** 0.0.24 - unstable api

[github.com]:               https://github.com/oberrich/phnt-rs
[github.com/ci]:            https://github.com/oberrich/phnt-rs/actions/workflows/rust.yml
[github.com/ci/img]:        https://github.com/oberrich/phnt-rs/actions/workflows/rust.yml/badge.svg
[github.com/phnt_nightly]:  https://github.com/oberrich/phnt_nightly
[github.com/src/build.rs]:  https://github.com/oberrich/phnt-rs/blob/master/src/build.rs
[github.com/sysinf]:        https://github.com/winsiderss/systeminformer
[github.com/sysinf/phnt]:   https://github.com/winsiderss/systeminformer/tree/master/phnt

[crates.io]:                https://crates.io/crates/phnt
[crates.io/bindgen]:        https://crates.io/crates/bindgen
[crates.io/img]:            https://img.shields.io/crates/v/phnt.svg

[docs.rs]:                  https://docs.rs/phnt
[docs.rs/img]:              https://docs.rs/phnt/badge.svg
[docs.rs/ffi]:              https://docs.rs/phnt/latest/phnt/ffi/index.html
[docs.rs/ffi/reexports]:    https://docs.rs/phnt/latest/phnt/ffi/index.html#reexports
[docs.rs/ffi/structs]:      https://docs.rs/phnt/latest/phnt/ffi/index.html#structs
[docs.rs/ffi/enums]:        https://docs.rs/phnt/latest/phnt/ffi/index.html#enums
[docs.rs/ffi/constants]:    https://docs.rs/phnt/latest/phnt/ffi/index.html#constants
[docs.rs/ffi/statics]:      https://docs.rs/phnt/latest/phnt/ffi/index.html#statics
[docs.rs/ffi/functions]:    https://docs.rs/phnt/latest/phnt/ffi/index.html#functions
[docs.rs/ffi/type-aliases]: https://docs.rs/phnt/latest/phnt/ffi/index.html#types
[docs.rs/ffi/unions]:       https://docs.rs/phnt/latest/phnt/ffi/index.html#unions
[docs.rs/ext]:              https://docs.rs/phnt/latest/phnt/ext/index.html
[docs.rs/ext/functions]:    https://docs.rs/phnt/latest/phnt/ext/index.html#functions
[docs.rs/ext/macros]:       https://docs.rs/phnt/latest/phnt/index.html#macros
