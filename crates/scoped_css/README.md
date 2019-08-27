# scoped_css

> Tools for generating scoped css.

## How to use

```rs
fn main() {
  let my_css = scoped_css::css!(
    .my_class {
      color: red;
    }
  );

  // this will generate a struct with a .classes.my_class field which
  // is an obfuscated &'static str.

  let my_app = scoped::smd!(
    <style>{ my_css.to_string() }</style>
    <div class={my_css.classes.my_class}>This will be red</div>
  );
}

// or the following, which generated a static variable named css
scoped_css::static_css!(
  .my_class {
    color: red;
  }
)
```

## Supported CSS features

Classes, ids, tags and attributes are supported. Selectors like `>` are not.

In addition, nested selectors are supported.

```
scoped_css::static_css!(
  body.my_app #some_id[some-attr="hello"] {
    .nested_stuff_is_supported_too {
      color: blue;
    }
  }
)
```

## TODO and drawbacks

This is pretty alpha, and is only being published for a demo :)

* There is no support for selectors that define the relationship between elements, e.g. `>`
* There is no support for pseudo-selectors
* There is no support for classes that aren't valid property names. For example, classes with dashes are not supported.
* The `to_css_string` function should be a trait, so that CSS can be passed around.
* There are no tests.
* There is no ability to change the name of the static css variable, or of its type.
