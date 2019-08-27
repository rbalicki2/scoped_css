extern crate proc_macro;

mod attribute;
mod class;
mod core;
mod id;
mod modifier;
mod rule;
mod selector;

mod parser_types;
mod types;
mod util;

use proc_macro2::{
  Ident,
  Span,
  TokenStream,
};

use std::{
  collections::{
    hash_map::DefaultHasher,
    HashMap,
  },
  hash::{
    Hash,
    Hasher,
  },
};

#[macro_use]
extern crate itertools;

fn get_prefix(input: &TokenStream) -> String {
  // N.B. Is this correct? Multiple identical calls to css! will
  // result in the same prefix. Though the styles will be the same,
  // I'm not sure if we want that.
  let mut hasher = DefaultHasher::new();
  input.to_string().hash(&mut hasher);
  hasher.finish().to_string()
}

fn as_ident(s: &str) -> Ident {
  Ident::new(s, Span::call_site())
}

fn get_declaration(fields: &HashMap<String, String>) -> TokenStream {
  fields.keys().fold(quote::quote!(), |accum, field| {
    let field = as_ident(field);
    quote::quote!(
      #accum
      pub #field: &'static str,
    )
  })
}

fn get_initialization(fields: &HashMap<String, String>) -> TokenStream {
  // let prefix = as_ident(prefix);
  // let field_key = as_ident(field_key);

  fields
    .iter()
    .fold(quote::quote!(), |accum, (field, value)| {
      // TODO allow the user to customize this format
      // let value = format!("{}_{}_{}", field_key, prefix, i);
      let field = as_ident(field);
      quote::quote!(
        #accum
        #field: #value,
      )
    })
}

fn into_hashmap(
  iter: impl Iterator<Item = String>,
  prefix: &str,
  keyword: &str,
) -> HashMap<String, String> {
  iter
    .enumerate()
    .map(|(i, item)| (item, format!("{}_{}_{}", keyword, prefix, i)))
    .collect()
}

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();

  let prefix = get_prefix(&input);

  let (rest, rule_set) = rule::parse_rule_set(input).expect("css! macro failed to parse");
  util::ensure_consumed(rest).expect("css! macro had left over characters");

  let (classes, ids) = rule_set.classes_and_ids();
  let classes = into_hashmap(classes.into_iter(), &prefix, "cl");
  let ids = into_hashmap(ids.into_iter(), &prefix, "id");

  let class_declaration = get_declaration(&classes);
  let class_initialization = get_initialization(&classes);
  let id_declaration = get_declaration(&ids);
  let id_initialization = get_initialization(&ids);
  let css_string = rule_set.as_css_string(&classes, &ids);

  quote::quote!({
    #[derive(Debug, Clone)]
    struct CssClasses {
      #class_declaration
    }

    #[derive(Debug, Clone)]
    struct CssIds {
      #id_declaration
    }

    #[derive(Debug, Clone)]
    struct CssProperties {
      pub ids: CssIds,
      pub classes: CssClasses,
    }

    impl CssProperties {
      pub fn as_css_string(&self) -> &'static str {
        #css_string
      }
    }

    CssProperties {
      ids: CssIds {
        #id_initialization
      },
      classes: CssClasses {
        #class_initialization
      }
    }
  })
  .into()
}

#[proc_macro]
pub fn static_css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();

  let prefix = get_prefix(&input);

  let (rest, rule_set) = rule::parse_rule_set(input).expect("css! macro failed to parse");
  util::ensure_consumed(rest).expect("css! macro had left over characters");

  let (classes, ids) = rule_set.classes_and_ids();
  let classes = into_hashmap(classes.into_iter(), &prefix, "cl");
  let ids = into_hashmap(ids.into_iter(), &prefix, "id");

  let class_declaration = get_declaration(&classes);
  let class_initialization = get_initialization(&classes);
  let id_declaration = get_declaration(&ids);
  let id_initialization = get_initialization(&ids);
  let css_string = rule_set.as_css_string(&classes, &ids);

  quote::quote!(
    #[derive(Debug, Clone)]
    struct CssClasses {
      #class_declaration
    }

    #[derive(Debug, Clone)]
    struct CssIds {
      #id_declaration
    }

    #[derive(Debug, Clone)]
    struct CssProperties {
      pub ids: CssIds,
      pub classes: CssClasses,
    }

    impl CssProperties {
      pub fn as_css_string(&self) -> &'static str {
        #css_string
      }
    }

    static css: CssProperties = CssProperties {
      ids: CssIds {
        #id_initialization
      },
      classes: CssClasses {
        #class_initialization
      }
    };
  )
  .into()
}
