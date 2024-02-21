use super::Error;

/// Extended Result
///
/// This type is meant to be a result that differentiates between:
/// - Typed errors (information)
/// - Exceptions (untreated errors)
///
/// In this case, T is the optimal return value (Ok), E is the
/// informational error (Err), and the exception type is predefined
/// as a thread-safe implementation of `std::error::Error`.
pub type ResultEx<T, E> = Result<Result<T, E>, Error>;

#[allow(clippy::unnecessary_wraps)]
#[allow(non_snake_case)]
pub fn to_Ok<T, E: Send + Sync>(value: T) -> ResultEx<T, E> {
  let res = Result::Ok(value);
  Result::Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
#[allow(non_snake_case)]
pub fn to_Err<T, E: Send + Sync>(err: E) -> ResultEx<T, E> {
  let res = Result::Err(err);
  Result::Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
#[allow(non_snake_case)]
pub fn to_Exc<T, E: Send + Sync>(err: Error) -> ResultEx<T, E> {
  Result::Err(err)
}

pub trait Extensions<T, E: Send + Sync> {
  fn is_ok(&self) -> bool;
  fn is_err(&self) -> bool;
  fn is_exc(&self) -> bool;
  fn unwrap(self) -> Result<T, E>;
  fn unwrap_ok(self) -> T;
  fn unwrap_err(self) -> E;
  fn unwrap_exc(self) -> Error;
  fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ResultEx<U, E>;
  fn map_err<U, F: FnOnce(E) -> U>(self, f: F) -> ResultEx<T, U>;
  fn map_exc<F: FnOnce(Error) -> Error>(self, f: F) -> ResultEx<T, E>;
  fn and_then<U, F: FnOnce(T) -> ResultEx<U, E>>(self, f: F) -> ResultEx<U, E>;
}

impl<T, E: Send + Sync> Extensions<T, E> for ResultEx<T, E> {
  fn is_ok(&self) -> bool {
    matches!(self, Result::Ok(Result::Ok(_)))
  }

  fn is_err(&self) -> bool {
    matches!(self, Result::Ok(Result::Err(_)))
  }

  fn is_exc(&self) -> bool {
    matches!(self, Result::Err(_))
  }

  fn unwrap(self) -> Result<T, E> {
    match self {
      Result::Ok(value) => value,
      Result::Err(err) => panic!("Exception! {err}"),
    }
  }

  fn unwrap_ok(self) -> T {
    match self {
      Result::Ok(Result::Ok(value)) => value,
      _ => panic!("Not Ok!"),
    }
  }

  fn unwrap_err(self) -> E {
    match self {
      Result::Ok(Result::Err(err)) => err,
      _ => panic!("Not an Error!"),
    }
  }

  fn unwrap_exc(self) -> Error {
    match self {
      Result::Err(err) => err,
      _ => panic!("Not an Exception!"),
    }
  }

  fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ResultEx<U, E> {
    match self {
      Result::Ok(Result::Ok(value)) => Result::Ok(Result::Ok(f(value))),
      Result::Ok(Result::Err(err)) => Result::Ok(Result::Err(err)),
      Result::Err(err) => Result::Err(err),
    }
  }

  fn map_err<U, F: FnOnce(E) -> U>(self, f: F) -> ResultEx<T, U> {
    match self {
      Result::Ok(Result::Ok(value)) => Result::Ok(Result::Ok(value)),
      Result::Ok(Result::Err(err)) => Result::Ok(Result::Err(f(err))),
      Result::Err(err) => Result::Err(err),
    }
  }

  fn map_exc<F: FnOnce(Error) -> Error>(self, f: F) -> Self {
    match self {
      Result::Ok(Result::Ok(value)) => Result::Ok(Result::Ok(value)),
      Result::Ok(Result::Err(err)) => Result::Ok(Result::Err(err)),
      Result::Err(err) => Result::Err(f(err)),
    }
  }

  fn and_then<U, F: FnOnce(T) -> ResultEx<U, E>>(self, f: F) -> ResultEx<U, E> {
    match self {
      Result::Ok(Result::Ok(value)) => f(value),
      Result::Ok(Result::Err(err)) => Result::Ok(Result::Err(err)),
      Result::Err(err) => Result::Err(err),
    }
  }
}

// Not using like this so we can still use the Result type's methods and traits
// enum ResultEx<T, E> {
//   Res(Result<T, E>),
//   Exc(Error),
// }
