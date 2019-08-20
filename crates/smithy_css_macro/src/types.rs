use std::{
  collections::{
    HashMap,
    HashSet,
  },
  ops::Deref,
};

/* -------- SELECTORS -------- */

// TODO handle +, >, pseudo selectors, etc.
pub type Element = String;

// Going off of https://www.w3schools.com/css/css_attribute_selectors.asp
#[derive(Debug, Clone)]
pub enum AttributeRelation {
  Equal(String),          // =
  Containing(String),     // ~=
  BeginsWithWord(String), // |=
  BeginsWith(String),     // ^=
  EndsWith(String),       // $=
  Contains(String),       // *=
}

impl AttributeRelation {
  pub fn from_strings(s1: &str, s2: String) -> Option<AttributeRelation> {
    match s1 {
      "=" => Some(AttributeRelation::Equal(s2)),
      "~=" => Some(AttributeRelation::Containing(s2)),
      "|=" => Some(AttributeRelation::BeginsWithWord(s2)),
      "^=" => Some(AttributeRelation::BeginsWith(s2)),
      "$=" => Some(AttributeRelation::EndsWith(s2)),
      "*=" => Some(AttributeRelation::Contains(s2)),
      _ => None,
    }
  }
}

impl ToString for AttributeRelation {
  fn to_string(&self) -> String {
    "=poop".to_string()
    // unimplemented!()
    // match self {

    // }
  }
}

#[derive(Debug, Clone)]
pub struct AttributeModifier {
  pub attribute: String,
  pub relation: Option<AttributeRelation>,
}

impl ToString for AttributeModifier {
  fn to_string(&self) -> String {
    format!(
      "[{}{}]",
      self.attribute,
      self
        .relation
        .as_ref()
        .map(|r| r.to_string())
        .unwrap_or("".to_string())
    )
  }
}

#[derive(Debug, Clone)]
pub enum Modifier {
  Class(String),
  Id(String),
  Attribute(AttributeModifier),
}

// TODO is Selector { element: None, modifiers: vec![] } valid?
// This is like body.some_class#some_id[some-attribute]
#[derive(Debug, Clone)]
pub struct Selector {
  pub element: Option<Element>,
  pub modifiers: Vec<Modifier>,
}

impl Selector {
  pub fn transform_classes_and_ids(
    &self,
    classes: &HashMap<String, String>,
    ids: &HashMap<String, String>,
  ) -> String {
    format!(
      "{}{}",
      self.element.as_ref().unwrap_or(&"".to_string()),
      self
        .modifiers
        .iter()
        .map(|modifier| match modifier {
          Modifier::Class(class) => format!(".{}", classes.get(class).unwrap()),
          Modifier::Id(id) => format!("#{}", ids.get(id).unwrap()),
          Modifier::Attribute(attr) => attr.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
    )
  }
}

// This is like body .foo, a vec of length 2
pub type NestedSelector = Vec<Selector>;
// This is like body .foo, span, p, a vec of length 3
pub type NestedSelectorList = Vec<NestedSelector>;

/* -------- PROPERTIES, RULES (putting it all together) -------- */

// TODO this should be a struct, preventing you from setting
// invalid css values, like margin: red
pub type Properties = HashMap<String, String>;

/// This is the most important struct, the unit of functionality
///
/// TODO: A better description
#[derive(Debug, Clone)]
pub struct Rule {
  pub nested_selector_list: NestedSelectorList,
  pub property_block: PropertyBlock,
}

#[derive(Debug, Clone)]
pub enum RuleOrProperty {
  Rule(Rule),
  Property((String, String)),
}

#[derive(Debug, Clone)]
pub struct PropertyBlock {
  pub properties: Properties,
  pub nested_rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct RuleSet(pub Vec<Rule>);

impl Deref for RuleSet {
  type Target = Vec<Rule>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Rule {
  pub fn classes_and_ids(&self) -> (HashSet<String>, HashSet<String>) {
    // TODO avoid allocating and calling .cloned
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    for nested_selector in self.nested_selector_list.iter() {
      for selector in nested_selector.iter() {
        for modifier in selector.modifiers.iter() {
          match modifier {
            Modifier::Class(class) => {
              classes.insert(class.into());
            },
            Modifier::Id(id) => {
              ids.insert(id.into());
            },
            _ => {},
          }
        }
      }
    }

    for child_rule in self.property_block.nested_rules.iter() {
      let (new_classes, new_ids) = child_rule.classes_and_ids();

      classes = classes.union(&new_classes).cloned().collect();
      ids = ids.union(&new_ids).cloned().collect();
    }
    (classes, ids)
  }

  pub fn as_css_string(
    &self,
    classes: &HashMap<String, String>,
    ids: &HashMap<String, String>,
    parent_selectors: &Vec<String>,
  ) -> String {
    let selectors = self
      .nested_selector_list
      .iter()
      .map(|nested_selector| {
        nested_selector
          .iter()
          .map(|selector| selector.transform_classes_and_ids(&classes, &ids))
          .collect::<Vec<String>>()
          .join(" ")
        // convert_nested_selector(classes, ids)
      })
      .collect::<Vec<String>>();

    let combined_selectors = itertools::iproduct!(parent_selectors, selectors)
      .map(|(v1, v2)| {
        let mut v1 = v1.clone();
        v1.push(' ');
        v1.push_str(&v2);
        v1
      })
      .collect::<Vec<String>>();

    let property_block_css = format!(
      "{{\n{}\n}}",
      self
        .property_block
        .properties
        .iter()
        .map(|(property_name, property_value)| format!("{}: {};", property_name, property_value))
        .collect::<Vec<String>>()
        .join("\n")
    );

    let mut css_string = format!("{}{}", combined_selectors.join(", "), property_block_css);
    let inner_css = self
      .property_block
      .nested_rules
      .iter()
      .map(|rule| rule.as_css_string(classes, ids, &combined_selectors))
      .collect::<Vec<_>>()
      .join("");

    css_string.push_str(&inner_css);
    css_string
  }
}

impl RuleSet {
  pub fn classes_and_ids(&self) -> (HashSet<String>, HashSet<String>) {
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    for rule in self.iter() {
      let (new_classes, new_ids) = rule.classes_and_ids();
      classes = classes.union(&new_classes).cloned().collect();
      ids = ids.union(&new_ids).cloned().collect();
    }
    (classes, ids)
  }

  pub fn as_css_string(
    &self,
    classes: &HashMap<String, String>,
    ids: &HashMap<String, String>,
  ) -> String {
    let mut css_string: String = "".into();
    for rule in self.iter() {
      css_string.push_str(&rule.as_css_string(&classes, &ids, &vec!["".to_string()]));
    }
    css_string
  }
}
