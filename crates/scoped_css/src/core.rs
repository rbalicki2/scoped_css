use crate::parser_types::*;
use proc_macro2::{
  Delimiter,
  Ident,
  Punct,
  Spacing,
};

pub fn parse_any(input: TokenStream) -> TokenStreamIResult<TokenTree> {
  if let Some((first_tree, rest)) = crate::util::stream_to_tree_vec(&input).split_first() {
    Ok((crate::util::slice_to_stream(rest), first_tree.clone()))
  } else {
    Err(Err::Error((input, ErrorKind::TakeTill1)))
  }
}

pub fn parse_ident(input: TokenStream) -> TokenStreamIResult<Ident> {
  if let Some((first_tree, rest)) = crate::util::stream_to_tree_vec(&input).split_first() {
    match first_tree {
      TokenTree::Ident(ident) => Ok((crate::util::slice_to_stream(rest), ident.clone())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  } else {
    // Err(Err::Incomplete(Needed::Size(1)))
    Err(Err::Error((input, ErrorKind::TakeTill1)))
  }
}

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
pub fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult<TokenStream> {
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
    // None => Err(Err::Incomplete(Needed::Size(1))),
    None => Err(Err::Error((input, ErrorKind::TakeTill1))),
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
    // None => Err(Err::Incomplete(Needed::Size(1))),
    None => Err(Err::Error((input, ErrorKind::TakeTill1))),
  }
}

pub fn parse_punct(
  input: TokenStream,
  spacing: Option<Spacing>,
  ch: Option<char>,
) -> TokenStreamIResult<Punct> {
  // N.B. this can probably be improved
  let vec = crate::util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Punct(p) => spacing.map_or_else(
        || {
          ch.map_or_else(
            || Ok((crate::util::slice_to_stream(rest), p.clone())),
            |ch| {
              if ch == p.as_char() {
                Ok((crate::util::slice_to_stream(rest), p.clone()))
              } else {
                Err(Err::Error((input.clone(), ErrorKind::TakeTill1)))
              }
            },
          )
        },
        |spacing| {
          ch.map_or_else(
            || {
              if spacing == p.spacing() {
                Ok((crate::util::slice_to_stream(rest), p.clone()))
              } else {
                Err(Err::Error((input.clone(), ErrorKind::TakeTill1)))
              }
            },
            |ch| {
              if spacing == p.spacing() && ch == p.as_char() {
                Ok((crate::util::slice_to_stream(rest), p.clone()))
              } else {
                Err(Err::Error((input.clone(), ErrorKind::TakeTill1)))
              }
            },
          )
        },
      ),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Error((input, ErrorKind::TakeTill1))),
  }
}

/// this will match any number of Punct's that having Spacing::Joint
/// followed by a single Punct with Spacing::Alone
pub fn parse_grouped_puncts(input: TokenStream) -> TokenStreamIResult<Vec<Punct>> {
  let (rest, mut vec) =
    crate::util::many_0(|input| parse_punct(input, Some(Spacing::Joint), None))(input)?;
  let (rest, punct) = parse_punct(rest, Some(Spacing::Alone), None)?;
  vec.push(punct);
  Ok((rest, vec))
}
