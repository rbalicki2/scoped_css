use crate::parser_types::*;

pub fn parse_class_selector(input: TokenStream) -> TokenStreamIResult<crate::types::Modifier> {
  let (rest, _punct) = crate::core::parse_punct(input, None, Some('.'))?;
  let (rest, ident) = crate::core::parse_ident(rest)?;
  Ok((rest, crate::types::Modifier::Class(ident.to_string())))
}
