use nom::{
  error::ErrorKind,
  Err,
  IResult,
};
use proc_macro2::{
  TokenStream,
  TokenTree,
};
type TokenStreamIResult<T> = IResult<TokenStream, T>;
type TokenTreeSlice<'a> = &'a [TokenTree];
type TokenTreeVec = Vec<TokenTree>;

pub fn ensure_consumed<T>((rest, t): (TokenStream, T)) -> TokenStreamIResult<T> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, t))
  }
}

pub fn stream_to_tree_vec(input: &TokenStream) -> TokenTreeVec {
  input.clone().into_iter().collect::<TokenTreeVec>()
}

pub fn slice_to_stream(input: TokenTreeSlice) -> TokenStream {
  input.iter().map(|x| x.clone()).collect()
}

pub fn alt<T>(
  b1: impl Fn(TokenStream) -> TokenStreamIResult<T>,
  b2: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<T> {
  move |input| {
    let cloned = input.clone();
    b1(input)
      .or_else(|_| b2(cloned))
  }
}
