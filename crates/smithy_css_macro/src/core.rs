use nom::{
  error::ErrorKind,
  Err,
  IResult,
  Needed,
};
use proc_macro2::{
  TokenStream,
  TokenTree,
  Ident,
};

pub fn parse_ident(input: TokenStream) -> IResult<TokenStream, Ident> {
  if let Some((first_tree, rest)) = crate::util::stream_to_tree_vec(&input).split_first() {
    match first_tree {
      TokenTree::Ident(ident) => Ok((crate::util::slice_to_stream(rest), ident.clone())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  } else {
    Err(Err::Incomplete(Needed::Size(1)))
  }
}