# Demo of a compilation error I don't understand

When running `cargo build` I get the following compilation error:

```
 Compiling hyper-problem-demo v0.1.0 (file:///.../hyper-problem-demo)
error[E0277]: the trait bound `MyServer: std::ops::Fn<()>` is not satisfied
  --> src/lib.rs:50:31
   |
50 |         return Ok(Http::new().bind(&addr, self)?);
   |                               ^^^^ the trait `std::ops::Fn<()>` is not implemented for `MyServer`
   |
   = note: required because of the requirements on the impl of `std::ops::FnOnce<()>` for `&MyServer`
   = note: required because of the requirements on the impl of `hyper::server::NewService` for `&MyServer`

error[E0277]: the trait bound `MyServer: std::ops::FnOnce<()>` is not satisfied
  --> src/lib.rs:50:31
   |
50 |         return Ok(Http::new().bind(&addr, self)?);
   |                               ^^^^ the trait `std::ops::FnOnce<()>` is not implemented for `MyServer`
   |
   = note: required because of the requirements on the impl of `hyper::server::NewService` for `&MyServer`

error: aborting due to 2 previous errors

error: Could not compile `hyper-problem-demo`.

To learn more, run the command again with --verbose.
```
