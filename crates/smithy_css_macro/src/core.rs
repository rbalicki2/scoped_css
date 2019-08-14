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
  Delimiter,
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

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> IResult<TokenStream, TokenStream> {
  // let cloned = input.clone();
  let vec = crate::util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref g) => {
        if let Some(target_delimiter) = delimiter {
          if g.delimiter() == target_delimiter {
            Ok((crate::util::slice_to_stream(rest), g.stream()))
          } else {
            Err(Err::Error((input, ErrorKind::TakeTill1)))
          }
        } else {
          Ok((crate::util::slice_to_stream(rest), g.stream()))
        }
      },
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

