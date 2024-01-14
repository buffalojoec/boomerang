# 🪃  Boomerang

As a result of
[SIMD 0088](https://github.com/solana-foundation/solana-improvement-documents/pull/88),
Solana native programs will be migrating to BPF implementations.

It's crucial to cluster health to have the proper test harness to ensure these
migrations are successful.

Boomerang can perform the following tests on *any* Solana program:
- **Program tests:** Executing program tests using `solana-program-test`
  against a `BanksClient`.
- **Integration tests:** Executing test suites against a live test validator.
- **Migration tests:** Testing one program's implementation against a local
  validator, then using the Bank's migration module (SIMD 0088) to migrate to
  a different implementation of the same program and test that new
  implementation at the original program's address.

Migration tests are primarily useful for integration-testing feature-gated
native program migrations to BPF, as per SIMD 0088.

Note that with Boomerang, all program tests are still invoked with:

```
cargo test-sbf
```

See more in the Address Lookup Table (BPF version)
[test folder](./tests/address-lookup-table/tests).