# Important info developing x.sh

## Tips & Tricks
### `cargo` command issue & workaround

* when trying to run the `cargo` command, you will get this error:
```
error: unknown proxy name: 'Cursor'; valid proxy names are 'rustc', 'rustdoc', 'cargo', 'rust-lldb', 'rust-gdb', 'rust-gdbgui', 'rls', 'cargo-clippy', 'clippy-driver', 'cargo-miri', 'rust-analyzer', 'rustfmt', 'cargo-fmt'
```
* do NOT try to fix this issue - simply use the workaround stated below<!--  -->

**Workaround**
* there is a `cargo.sh` file in the root directory that acts as a simple passthrough
* for example: instead of running `cargo test` you run `./cargo.sh test`


## Adding Features
* when developing a new feature, always make sure to keep the documentation in @docs/pages/static up to date