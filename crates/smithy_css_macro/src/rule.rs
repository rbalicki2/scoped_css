use crate::parser_types::*;

use crate::{
  core::{
    parse_any,
    parse_ident,
    parse_punct,
  },
  types::{
    PropertyBlock,
    Rule,
    RuleOrProperty,
    RuleSet,
  },
  util::{
    alt,
    many_0,
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

  let (_rest, grouped_property_values) = many_0(many_0_joint(parse_any))(property_values)?;

  let property_values_string = grouped_property_values
    .into_iter()
    .map(|vec| {
      vec
        .into_iter()
        .map(|g| g.to_string())
        .collect::<Vec<_>>()
        .join("")
    })
    .collect::<Vec<_>>()
    .join(" ");

  Ok((rest, (property_name, property_values_string)))
}

fn parse_rule_or_property(input: TokenStream) -> TokenStreamIResult<RuleOrProperty> {
  alt(
    map(parse_property, |property| {
      RuleOrProperty::Property(property)
    }),
    map(parse_rule, |rule| RuleOrProperty::Rule(rule)),
  )(input)
}

fn parse_rule(input: TokenStream) -> TokenStreamIResult<Rule> {
  let (rest, nested_selector_list) = crate::selector::parse_nested_selector_list(input)?;
  let (rest, group_contents) =
    crate::core::parse_group_with_delimiter(rest, Some(Delimiter::Brace))?;

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

pub fn parse_rule_set(input: TokenStream) -> TokenStreamIResult<RuleSet> {
  let (rest, rules) = crate::util::many_0(parse_rule)(input)?;
  Ok((rest, RuleSet(rules)))
}
