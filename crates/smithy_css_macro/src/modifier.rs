use crate::parser_types::*;

use crate::{
  types::Modifier,
  util::alt,
};

pub fn parse_modifier(input: TokenStream) -> TokenStreamIResult<Modifier> {
  alt(
    alt(
      |input| {
        crate::attribute::parse_attribute_selector(input)
          .map(|(rest, attribute_selector)| (rest, Modifier::Attribute(attribute_selector)))
      },
      |input| {
        crate::class::parse_class_selector(input)
          .map(|(rest, class)| (rest, Modifier::Class(class)))
      },
    ),
    |input| crate::id::parse_id_selector(input).map(|(rest, id)| (rest, Modifier::Id(id))),
  )(input)
}
