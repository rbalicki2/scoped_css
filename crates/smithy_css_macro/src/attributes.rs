use crate::parser_types::*;

use nom::sequence::tuple;
use proc_macro2::Delimiter;

fn parse_attribute_symbol(input: TokenStream) -> TokenStreamIResult<String> {
  let vec = crate::util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Punct(p) => Ok((crate::util::slice_to_stream(rest), p.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_contents_without_relation(
  input: TokenStream,
) -> TokenStreamIResult<crate::types::AttributeModifier> {
  crate::core::parse_ident(input).and_then(|(rest, i)| {
    let (rest, _) = crate::util::ensure_consumed((rest, ()))?;
    Ok((
      rest,
      crate::types::AttributeModifier {
        attribute: i.to_string(),
        relation: None,
      },
    ))
  })
}

fn parse_attribute_contents_with_relation(
  input: TokenStream,
) -> TokenStreamIResult<crate::types::AttributeModifier> {
  let cloned = input.clone();
  let (rest, (attribute, symbol, rhs)) = tuple((
    crate::core::parse_ident,
    parse_attribute_symbol,
    crate::core::parse_literal_or_ident,
  ))(input)?;

  let relation = crate::types::AttributeRelation::from_strings(&symbol, rhs);
  let relation = relation.ok_or(Err::Error((cloned, ErrorKind::TakeTill1)))?;

  let (rest, _) = crate::util::ensure_consumed((rest, ()))?;

  Ok((
    rest,
    crate::types::AttributeModifier {
      attribute: attribute.to_string(),
      relation: Some(relation),
    },
  ))
}

pub fn parse_attribute(input: TokenStream) -> TokenStreamIResult<crate::types::AttributeModifier> {
  crate::core::parse_group_with_delimiter(input, Some(Delimiter::Bracket)).and_then(
    |(rest, input)| {
      crate::util::alt(
        parse_attribute_contents_without_relation,
        parse_attribute_contents_with_relation,
      )(input)
      .map(|(_rest, x)| (rest, x))
    },
  )
}
