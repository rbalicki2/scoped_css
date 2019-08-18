use crate::parser_types::*;

use crate::{
  core::{
    parse_ident,
    parse_punct,
  },
  types::{
    NestedSelector,
    NestedSelectorList,
    Selector,
  },
  util::{
    many_0,
    many_0_joint,
  },
};

use nom::{
  combinator::{
    map,
    opt,
  },
  sequence::tuple,
};
use proc_macro2::Spacing;

fn parse_selector(input: TokenStream) -> TokenStreamIResult<Selector> {
  let (rest, element) = opt(map(parse_ident, |ident| ident.to_string()))(input)?;
  // TODO make sure the ident is adjacent
  let (rest, modifiers) = many_0_joint(crate::modifier::parse_modifier)(rest)?;
  Ok((rest, Selector { element, modifiers }))
}

fn parse_nested_selector(input: TokenStream) -> TokenStreamIResult<NestedSelector> {
  let (rest, selectors) = many_0(parse_selector)(input)?;
  Ok((rest, selectors))
}

pub fn parse_nested_selector_list(input: TokenStream) -> TokenStreamIResult<NestedSelectorList> {
  let (rest, nested_selectors) = many_0(tuple((parse_nested_selector, |input| {
    parse_punct(input, Some(Spacing::Alone), Some(','))
  })))(input)?;

  // drop the commas
  let mut nested_selectors = nested_selectors
    .into_iter()
    .map(|x| x.0)
    .collect::<Vec<_>>();

  let (rest, nested_selector) = parse_nested_selector(rest)?;
  nested_selectors.push(nested_selector);
  Ok((rest, nested_selectors))
}
