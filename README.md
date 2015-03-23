rcrypt
=====================
rcrypt is a rust-based cryptographic library build entirely for learning purposes. It is **not** suitable for high security purposes and will not be for the foreseeable future.

Using rcrypt in your project
-------------------
If, for some reason, you wish to include rcrypt in your rust project, add the following to your ```Cargo.toml``` file:
```toml
[dependencies.rcrypt]
git = "https://github.com/dgriffen/rcrypt.git"
```
then add
```rust
extern crate rcrypt;
```
to your rust project.
