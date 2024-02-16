use std::fmt::Display;

pub trait ExpectError<T, E> {
  fn expect_error<F: Display, O: FnOnce(E) -> F>(self, op: O) -> T;
}

impl<T, E> ExpectError<T, E> for Result<T, E> {
  #[inline]
  fn expect_error<F: Display, O: FnOnce(E) -> F>(self, op: O) -> T {
    match self {
      Ok(o) => o,
      Err(e) => panic!("{}", op(e)),
    }
  }
}
