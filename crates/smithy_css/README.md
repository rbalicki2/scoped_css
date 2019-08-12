# smithy_css

> Tools for working with css in [Smithy](https://www.smithy.rs)
>
> To get the framework-agnostic version, use `smithy_css_core`.

## How to use

```rs
let my_css = smithy_css::css!(
  .my_class {
    color: red;
  }
);

smd!(
  { my_css.style_tag() }
  <div class={my_css.classes.my_class}>This will be red</div>
)
// or
smd!(
  <style>{ my_css.to_string() }</style>
  <div class={my_css.classes.my_class}>This will be red</div>
)
```
