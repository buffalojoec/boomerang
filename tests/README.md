# Boomerang Tests

This test folder is designed to demonstrate use of Boomerang for testing
Solana programs using each of the methods outlined in the
[README](../README.md);

One such test program is the Address Lookup Table program. Here the program is
provided in its BPF form, to be tested against the native version.

You can find the tests using Boomerang in the
[tests](./address-lookup-table/tests) folder of the program.

All tests are dictated by the config in `tests/main.rs` and are run with:

```
cargo test-sbf
```
