extern crate proc_macro;

mod attributes;
mod core;
mod parser_types;
mod types;
mod util;

use proc_macro2::TokenStream;

use nom::multi::many0;

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();
  println!("\ninput {:?}", input);
  // this seems like a hack. What I want is an iterator of &TokenTree's,
  // but TokenStream only implements into_iter for some reason
  //
  // (We actually need a slice of TokenTree's)
  // let input = input.into_iter().collect::<TokenTreeVec>();

  let foo = util::many0(attributes::parse_attribute)(input);
  println!("\nparse attribute result = {:?}", foo);
  match foo {
    Ok((rest, some_vec)) => {
      util::ensure_consumed(rest).unwrap();
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
