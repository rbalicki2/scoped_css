use crate::parser_types::*;

use crate::{
  core::{
    parse_ident,
    parse_punct,
  },
  types::{
    PropertyBlock,
    Rule,
  },
  util::{
    alt,
    many_0_joint,
    take_until_and_match,
  },
};

use proc_macro2::{
  Delimiter,
  Spacing,
};

fn parse_property(input: TokenStream) -> TokenStreamIResult<(String, String)> {
  let (rest, property_name) = many_0_joint(alt(
    |input| parse_ident(input).map(|(rest, ident)| (rest, ident.to_string())),
    |input| parse_punct(input, None, Some('-')).map(|(rest, punct)| (rest, punct.to_string())),
  ))(input)
  .map(|(rest, vec)| (rest, vec.join("")))?;

  let (rest, _colon) = parse_punct(rest, None, Some(':'))?;
  let (rest, (property_values, _semicolon)) =
    take_until_and_match(|input| parse_punct(input, None, Some(';')))(rest)?;

  // TODO: We lose track of spacing in some cases, like box-sizing: border-box and the like
  // Maybe we can use the ol take_0(take_0_joint) trick
  let property_values = property_values
    .into_iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(" ");

  Ok((rest, (property_name, property_values)))
}

pub fn parse_rule(input: TokenStream) -> TokenStreamIResult<Rule> {
  let (rest, nested_selector_list) = crate::selector::parse_nested_selector_list(input)?;
  let (rest, group_contents) =
    crate::core::parse_group_with_delimiter(rest, Some(Delimiter::Brace))?;
  let (inner_rest, rules) = crate::util::many_0(parse_property)(group_contents)?;
  crate::util::ensure_consumed(inner_rest)?;
  Ok((
    rest,
    Rule {
      nested_selector_list,
      property_block: PropertyBlock {
        properties: rules
          .into_iter()
          .collect::<std::collections::HashMap<_, _>>(),
        nested_rules: vec![],
      },
    },
  ))
}
