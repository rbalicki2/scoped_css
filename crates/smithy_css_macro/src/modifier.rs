use crate::parser_types::*;

use crate::{
  types::Modifier,
  util::alt,
};

pub fn parse_modifier(input: TokenStream) -> TokenStreamIResult<Modifier> {
  alt(
    alt(
      |input| crate::attribute::parse_attribute_selector(input),
      |input| crate::class::parse_class_selector(input),
    ),
    |input| crate::id::parse_id_selector(input),
  )(input)
}
