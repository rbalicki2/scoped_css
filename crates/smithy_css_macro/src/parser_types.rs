pub use nom::{
  error::ErrorKind,
  Err,
  IResult,
  Needed,
};
pub use proc_macro2::{
  TokenStream,
  TokenTree,
};

pub type TokenStreamIResult<T> = IResult<TokenStream, T>;
