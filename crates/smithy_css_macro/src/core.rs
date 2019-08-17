use crate::parser_types::*;
use proc_macro2::{
  Delimiter,
  Ident,
};

pub fn parse_ident(input: TokenStream) -> TokenStreamIResult<Ident> {
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
pub fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult<TokenStream> {
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

pub fn parse_literal_or_ident(input: TokenStream) -> TokenStreamIResult<String> {
  let vec = crate::util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      // TODO strip quotes off of this string
      TokenTree::Literal(l) => Ok((crate::util::slice_to_stream(rest), l.to_string())),
      TokenTree::Ident(i) => Ok((crate::util::slice_to_stream(rest), i.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}
