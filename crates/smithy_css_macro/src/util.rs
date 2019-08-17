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

pub fn ensure_consumed(rest: TokenStream) -> TokenStreamIResult<()> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, ()))
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
    b1(input).or_else(|_| b2(cloned))
  }
}

pub fn many_0<T>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    let mut last_len = stream_to_tree_vec(&i).len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = stream_to_tree_vec(&i1).len();
          if last_len == new_len {
            return Err(Err::Error((i, ErrorKind::Many0)));
          }
          last_len = new_len;

          i = i1;
          acc.push(o);
        },
      }
    }
  }
}
