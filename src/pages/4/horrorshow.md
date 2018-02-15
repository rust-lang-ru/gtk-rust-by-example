# Horrorshow HTML-шаблоны

Хотя это и не связано с разработкой c использованием GTK, пакет horrorshow
предоставляет удобные макроопределения, которые дают возможность эффективно
генерировать HTML-строки в памяти, используя DSL (_domain-specific language_)
совместно с Rust, который может быть запущен посредством использования символа
(_sigil_) `@`.

```rust
#[macro_use]
extern crate horrorshow;
use horrorshow::helper::doctype;

let title = "Title";
let content = "A string\nwith multiple\n\nlines";
let html_string = format!(
    "{}",
    html!{
        : doctype::HTML,
        html {
            head {
                style { : "#style { }" }
            }
            body {
                h1(id="style") { : title }
                @ for line in content.lines().filter(|x| !x.is_empty()) {
                    p { : line }
                }
            }
        }
    }
);
```