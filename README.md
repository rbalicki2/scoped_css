# smithy_css

> Tools for working with css in [Smithy](https://www.smithy.rs)

## How to use

```rs
let my_css = smithy_css::css!(
  .my_class {
    color: red;
  }
  #my_id {
    color: blue;
  }
);
// my_css is a custom struct (one definition per instance) which looks like:
// CssProperties {
//     ids: CssIds {
//         my_id: "id_18319942900107291251_0",
//     },
//     classes: CssClasses {
//         my_class: "cl_18319942900107291251_0",
//     },
// }

smd!(
  <style>{ my_css.to_css_string() }</style>
  <div class={my_css.classes.my_class}>This will be red</div>
)
```
