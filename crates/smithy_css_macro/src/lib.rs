extern crate proc_macro;

mod types;

use nom::{
  branch::alt,
  bytes::streaming::take_while_m_n,
  combinator::{map, flat_map, map_parser},
  error::ErrorKind,
  sequence::tuple,
  Err,
  IResult,
  Needed,
};

use proc_macro2::{
  Delimiter,
  Group,
  Ident,
  Literal,
  Punct,
  Span,
  TokenStream,
  TokenTree,
};

type TokenTreeSlice<'a> = &'a [TokenTree];
type TokenTreeVec = Vec<TokenTree>;
type TokenStreamIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;
type TokenStreamIResult2<T> = IResult<TokenStream, T>;

fn stream_to_tree_vec(input: &TokenStream) -> TokenTreeVec {
  input.clone().into_iter().collect::<TokenTreeVec>()
}

fn slice_to_stream(input: TokenTreeSlice) -> TokenStream {
  input.iter().map(|x| x.clone()).collect()
}

fn get_group_contents<'a>(
  tree: &'a TokenTree,
  delimiter: Option<Delimiter>,
) -> Option<TokenTreeVec> {
  match tree {
    TokenTree::Group(g) => match (delimiter, g.delimiter()) {
      (Some(target_delimiter), group_delimiter) if target_delimiter == group_delimiter => {
        Some(stream_to_tree_vec(&g.stream()))
      },
      _ => Some(stream_to_tree_vec(&g.stream())),
    },
    _ => None,
  }
}

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult2<TokenStream> {
  // let cloned = input.clone();
  let vec = stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref g) => {
        if let Some(target_delimiter) = delimiter {
          if g.delimiter() == target_delimiter {
            Ok((slice_to_stream(rest), g.stream()))
          } else {
            Err(Err::Error((input, ErrorKind::TakeTill1)))
          }
        } else {
          Ok((slice_to_stream(rest), g.stream()))
        }
      },
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

// // Do not need this
// fn many_groups(input: TokenTreeSlice) -> TokenStreamIResult<Vec<TokenStream>> {
//   let parse_group = |input| parse_group_with_delimiter(input, None);
//   let (rest, (g1, g2, g3)) = tuple((parse_group, parse_group, parse_group))(input)?;
//   Ok((rest, vec![g1, g2, g3]))
// }

fn parse_ident(input: TokenStream) -> IResult<TokenStream, Ident> {
  // println!("parse ident input {:?}", input);
  if let Some((first_tree, rest)) = stream_to_tree_vec(&input).split_first() {
    match first_tree {
      TokenTree::Ident(ident) => Ok((slice_to_stream(rest), ident.clone())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  } else {
    Err(Err::Incomplete(Needed::Size(1)))
  }
}

// TODO parse multiple adjacent symbols
fn parse_attribute_symbol(input: TokenStream) -> TokenStreamIResult2<String> {
  let vec = stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Punct(p) => Ok((slice_to_stream(rest), p.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
  // let cloned = input.clone();

  // match input.split_first() {
  //   Some((first, rest)) => match first {
  //     TokenTree::Punct(punct) => Ok((rest, punct.to_string())),
  //     _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
  //   },
  //   None => Err(Err::Incomplete(Needed::Size(1))),
  // }
  // unimplemented!()
}

fn parse_literal_or_ident(input: TokenStream) -> TokenStreamIResult2<String> {
  dbg!(&input);
  let vec = stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      // TODO strip quotes off of this string
      TokenTree::Literal(l) => Ok((slice_to_stream(rest), l.to_string())),
      TokenTree::Ident(i) => Ok((slice_to_stream(rest), i.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_contents_without_relation(
  input: TokenStream
) -> TokenStreamIResult2<types::AttributeModifier> {
  println!("FOOOO!!!! {:?}", input);
  parse_ident(input)
    .map(|(rest, i)| (rest, types::AttributeModifier {
      attribute: i.to_string(),
      relation: None,
    }))
}

fn parse_attribute_contents_with_relation(
  input: TokenStream,
) -> TokenStreamIResult2<types::AttributeModifier> {
  let cloned = input.clone();
  let (rest, (attribute, symbol, rhs)) =
    tuple((parse_ident, parse_attribute_symbol, parse_literal_or_ident))(input)?;

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

fn ensure_consumed<T>((rest, t): (TokenStream, T)) -> TokenStreamIResult2<T> {
  if (!rest.is_empty()) {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, t))
  }
}

fn my_alt<T>(
  b1: impl Fn(TokenStream) -> TokenStreamIResult2<T>,
  b2: impl Fn(TokenStream) -> TokenStreamIResult2<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult2<T> {
  move |input| {
    let cloned = input.clone();
    b1(input)
      .or_else(|_| b2(cloned))
  }
}

fn parse_attribute(input: TokenStream) -> TokenStreamIResult2<types::AttributeModifier> {
  parse_group_with_delimiter(input, Some(Delimiter::Bracket))
    .and_then(|(rest, input)| 
      my_alt(
        // TODO ensure_consumed should happen within these calls
        parse_attribute_contents_with_relation,
        parse_attribute_contents_without_relation,
      )(input)
    )
    .and_then(ensure_consumed)
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
    Ok((rest, some_vec)) => {
      let foo = format!("{:?}", some_vec);
      quote::quote!({
        #foo
        // vec![
        //   #(#some_vec),*
        // ]
      })
    }
    .into(),
    _ => unimplemented!("NOOO"),
  }
  // quote::quote!(123).into()

  // let starts_with_group = parse_group_with_delimiter(tree_vec.as_slice(), None);

  // println!("starts with group {:?}", starts_with_group);
  // // println!("css input {}", input);

  // // println!(
  // //   "CSS is some ???? {:?}",
  // //   get_group_contents(&input.into_iter().next().unwrap(), Some(Delimiter::Brace))
  // // );

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
