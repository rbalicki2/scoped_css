use crate::parser_types::*;

use nom::sequence::tuple;
use proc_macro2::Delimiter;

fn parse_attribute_symbol(input: TokenStream) -> TokenStreamIResult<String> {
  let (rest, punct_vec) = crate::core::parse_grouped_puncts(input)?;

  Ok((
    rest,
    punct_vec
      .into_iter()
      .map(|p| p.to_string())
      .collect::<Vec<String>>()
      .join(""),
  ))
}

fn parse_attribute_contents_without_relation(
  input: TokenStream,
) -> TokenStreamIResult<crate::types::AttributeModifier> {
  crate::core::parse_ident(input).and_then(|(rest, i)| {
    let (rest, _) = crate::util::ensure_consumed(rest)?;
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

  let (rest, _) = crate::util::ensure_consumed(rest)?;

  Ok((
    rest,
    crate::types::AttributeModifier {
      attribute: attribute.to_string(),
      relation: Some(relation),
    },
  ))
}

pub fn parse_attribute_selector(
  input: TokenStream,
) -> TokenStreamIResult<crate::types::AttributeModifier> {
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
