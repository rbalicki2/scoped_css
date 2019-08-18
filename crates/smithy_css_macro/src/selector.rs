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
  combinator::opt,
  sequence::tuple,
};
use proc_macro2::{
  Ident,
  Spacing,
};

fn parse_selector(input: TokenStream) -> TokenStreamIResult<Selector> {
  let (rest, ident) = opt(parse_ident)(input)?;
  // TODO make sure the ident is adjacent
  let (rest, modifiers) = many_0_joint(crate::modifier::parse_modifier)(rest)?;
  Ok((
    rest,
    Selector {
      element: ident.map(|x| x.to_string()),
      modifiers,
    },
  ))
}

fn parse_nested_selector(input: TokenStream) -> TokenStreamIResult<NestedSelector> {
  let (rest, selectors) = many_0(parse_selector)(input)?;
  Ok((rest, selectors))
}

pub fn parse_nested_selector_list(input: TokenStream) -> TokenStreamIResult<NestedSelectorList> {
  let (rest, vec) = many_0(tuple((parse_nested_selector, |input| {
    parse_punct(input, Some(Spacing::Alone), Some(','))
  })))(input)?;
  Ok((rest, vec.into_iter().map(|x| x.0).collect()))
}
