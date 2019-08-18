use crate::parser_types::*;

use crate::{
  core::{
    parse_ident,
    parse_punct,
  },
  types::{
    PropertyBlock,
    Rule,
    RuleOrProperty,
  },
  util::{
    alt,
    many_0_joint,
    take_until_and_match,
  },
};

use proc_macro2::Delimiter;

use std::collections::HashMap;

use nom::combinator::map;

fn parse_property(input: TokenStream) -> TokenStreamIResult<(String, String)> {
  let (rest, property_name) = map(
    many_0_joint(alt(
      map(parse_ident, |ident| ident.to_string()),
      map(
        |input| parse_punct(input, None, Some('-')),
        |punct| punct.to_string(),
      ),
    )),
    |vec| vec.join(""),
  )(input)?;

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

fn parse_rule_or_property(input: TokenStream) -> TokenStreamIResult<RuleOrProperty> {
  alt(
    map(parse_property, |property| {
      RuleOrProperty::Property(property)
    }),
    map(parse_rule, |rule| RuleOrProperty::Rule(rule)),
  )(input)
}

pub fn parse_rule(input: TokenStream) -> TokenStreamIResult<Rule> {
  let (rest, nested_selector_list) = crate::selector::parse_nested_selector_list(input)?;
  let (rest, group_contents) =
    crate::core::parse_group_with_delimiter(rest, Some(Delimiter::Brace))?;
  // let (inner_rest, properties) = map(crate::util::many_0(parse_property), |rules| {
  //   rules
  //     .into_iter()
  //     .collect::<std::collections::HashMap<_, _>>()
  // })(group_contents)?;
  let (inner_rest, (properties, nested_rules)) = map(
    crate::util::many_0(parse_rule_or_property),
    |rule_or_property_vec| {
      rule_or_property_vec.into_iter().fold(
        (HashMap::new(), vec![]),
        |(mut properties, mut nested_rules), rule_or_property| {
          match rule_or_property {
            RuleOrProperty::Rule(r) => {
              nested_rules.push(r);
            },
            RuleOrProperty::Property((key, val)) => {
              properties.insert(key, val);
            },
          };
          (properties, nested_rules)
        },
      )
    },
  )(group_contents)?;
  crate::util::ensure_consumed(inner_rest)?;
  Ok((
    rest,
    Rule {
      nested_selector_list,
      property_block: PropertyBlock {
        properties,
        nested_rules,
      },
    },
  ))
}
