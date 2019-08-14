extern crate proc_macro;

mod types;
mod util;
mod attributes;
mod core;

use nom::{
  error::ErrorKind,
  sequence::tuple,
  Err,
  IResult,
  Needed,
};

use proc_macro2::{
  Delimiter,
  TokenStream,
  TokenTree,
};

type TokenStreamIResult<T> = IResult<TokenStream, T>;

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult<TokenStream> {
  // let cloned = input.clone();
  let vec = util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref g) => {
        if let Some(target_delimiter) = delimiter {
          if g.delimiter() == target_delimiter {
            Ok((util::slice_to_stream(rest), g.stream()))
          } else {
            Err(Err::Error((input, ErrorKind::TakeTill1)))
          }
        } else {
          Ok((util::slice_to_stream(rest), g.stream()))
        }
      },
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}


// TODO parse multiple adjacent symbols
fn parse_attribute_symbol(input: TokenStream) -> TokenStreamIResult<String> {
  let vec = util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Punct(p) => Ok((util::slice_to_stream(rest), p.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_literal_or_ident(input: TokenStream) -> TokenStreamIResult<String> {
  dbg!(&input);
  let vec = util::stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      // TODO strip quotes off of this string
      TokenTree::Literal(l) => Ok((util::slice_to_stream(rest), l.to_string())),
      TokenTree::Ident(i) => Ok((util::slice_to_stream(rest), i.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_contents_without_relation(
  input: TokenStream
) -> TokenStreamIResult<types::AttributeModifier> {
  crate::core::parse_ident(input)
    .map(|(rest, i)| (rest, types::AttributeModifier {
      attribute: i.to_string(),
      relation: None,
    }))
}

fn parse_attribute_contents_with_relation(
  input: TokenStream,
) -> TokenStreamIResult<types::AttributeModifier> {
  let cloned = input.clone();
  let (rest, (attribute, symbol, rhs)) =
    tuple((crate::core::parse_ident, parse_attribute_symbol, parse_literal_or_ident))(input)?;

  let relation = types::AttributeRelation::from_strings(&symbol, rhs);
  let relation = relation.ok_or(Err::Error((cloned, ErrorKind::TakeTill1)))?;

  Ok((
    rest,
    types::AttributeModifier {
      attribute: attribute.to_string(),
      relation: Some(relation),
    },
  ))
}


fn parse_attribute(input: TokenStream) -> TokenStreamIResult<types::AttributeModifier> {
  parse_group_with_delimiter(input, Some(Delimiter::Bracket))
    .and_then(|(_rest, input)| 
      util::alt(
        // TODO ensure_consumed should happen within these calls
        parse_attribute_contents_with_relation,
        parse_attribute_contents_without_relation,
      )(input)
    )
    .and_then(util::ensure_consumed)
}

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();
  println!("\ninput {:?}", input);
  // this seems like a hack. What I want is an iterator of &TokenTree's,
  // but TokenStream only implements into_iter for some reason
  //
  // (We actually need a slice of TokenTree's)
  // let input = input.into_iter().collect::<TokenTreeVec>();

  let foo = parse_attribute(input);
  println!("\nparse attribute result = {:?}", foo);
  match foo {
    Ok((_rest, some_vec)) => {
      let foo = format!("{:?}", some_vec);
      quote::quote!({
        #foo
      })
    }
    .into(),
    _ => unimplemented!("NOOO"),
  }

  // quote::quote!({
  //   #[derive(Debug, Clone)]
  //   struct CssClasses {
  //     my_class: String,
  //   }
  //   #[derive(Debug, Clone)]
  //   struct CssIds {}
  //   #[derive(Debug, Clone)]
  //   struct CssWrapper {
  //     classes: CssClasses,
  //     ids: CssIds,
  //   }
  //   // TODO figure out why this doesn't work
  //   // = help: message: attempt to subtract with overflow
  //   //
  //   // TODO: divide this into smithy_css_core and include this impl only later
  //   // impl CssWrapper {
  //   //   pub fn style_tag<'a>(&self) -> smithy::types::SmithyComponent<'a> {
  //   //     // smithy::smd!(<style>
  //   //     //   foo
  //   //     // </style>)
  //   //     let a = smithy::smd!(<div>a</div>);
  //   //     smithy::smd!()
  //   //   }
  //   // }

  //   impl ToString for CssWrapper {
  //     fn to_string(&self) -> String {
  //       format!(".{} {{ background-color: red; }}", self.classes.my_class)
  //     }
  //   }

  //   let my_class = "foo".into();
  //   CssWrapper {
  //     classes: CssClasses { my_class },
  //     ids: CssIds {},
  //   }
  // })
  // .into()
}
