use crate::parser_types::*;

use crate::{
  types::Selector,
  util::many_0_joint,
};

use nom::combinator::opt;
use proc_macro2::Ident;

pub fn parse_selector(input: TokenStream) -> TokenStreamIResult<Selector> {
  let (rest, ident) = opt(crate::core::parse_ident)(input)?;
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
