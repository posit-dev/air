To run tests you need to install [cargo insta](https://crates.io/crates/cargo-insta):

```sh
cargo install cargo-insta
```

Run and review tests with:

```sh
cargo insta test
cargo insta review
```

The `src/generated.rs` file is produced by running this at the workspace root:

```sh
just gen-formatter
```
