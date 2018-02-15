# Markdown to HTML

До того как мы реализуем метод **connect_changed()**, сначала нужно
реализовать модуль **preview**,  который будет использовать метод, чтобы получить
HTML-строку для передачи в web-представление.

Два шага, чтобы преобразовать Markdown в HTML. Первый, включает в себя простое преобразование Markdown в HTML, но и этого недостаточно.
Вам также необходимо интегрировать это в дополнительный HTML и сделать поддержку
подсветки синтаксиса с помощью JavaScript. Не беспокойтесь, потому что мы будем использовать **highlight.js**, который позаботится за нас об этом.

## Преобразование Markdown в HTML

К счастью, у корпорации Google существует пакет (crate)
[pulldown-cmark](https://github.com/google/pulldown-cmark). Обратите внимание, что
он реализован в качестве парсера для повышения эффективности. Осталось предоставить Markdown текст в качестве **&str** в структуру **Parser** и указать изменяемую ссылку **String**, чтобы вернуть HTML.

```rust
use pulldown_cmark::{html, Parser};

/// Входит Markdown текст; выходит HTML текст.
fn mark_to_html(markdown: &str) -> String {
    let parser = Parser::new(&markdown);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}
```

## Применение стиля к нашему HTML

Мы не хотим останавливаться на достигнутом, поэтому будем использовать функцию (указанную выше) внутри нашей публичной функии **render()**, чтобы интегрировать CSS вместе с JavaScript и получить ожидаемый вывод HTML в web просмотр.

> Заметьте, что мы предоставляем HTML из нашего markdown в секцию **body**
> HTML страницы и имеем завернутую строку **Raw**, чтобы сказать
> макросу **horrorshow** не уходить от внутреннего текста. Если вы предпочитаете
> большее кол-во стилей, тогда вы можете применить дополнительные стили для
> вашего текста.

```rust
use horrorshow::Raw;
use horrorshow::helper::doctype;

/// Входит Markdown текст; выходит стильный HTML текст.
pub fn render(markdown: &str) -> String {
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                    script {
                        : Raw("hljs.initHighlightingOnLoad()")
                    }
                    style {
                        : "body { width: 80%; margin: 0 auto }";
                        : "img { max-width: 80% }"
                    }
                }
                body {
                    : Raw(&mark_to_html(markdown));
                }
            }
        )
    )
}
```