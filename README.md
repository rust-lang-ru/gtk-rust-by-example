# GRBE — Gtk-Rust на примерах

[![Build Status](https://travis-ci.org/ruRust/gtk-rust-by-example.svg?branch=master)](https://travis-ci.org/ruRust/gtk-rust-by-example) [![][License]](#Лицензия)

[License]: https://img.shields.io/crates/l/rustc-serialize.svg

Изучайте разработку GUI приложений вместе в библиотекой [GTK][gtk] на языке программирования [Rust][rust].

[rust]: https://www.rust-lang.org/ru-RU/
[gtk]: https://github.com/gtk-rs/gtk/

## Работа в процессе

Перевод `GRBE` находится в процессе.
Если вы заинтересованы в этом проекте и хотите помочь - смотрите [CONTRIBUTING.md](https://github.com/ruRust/gtk-rust-by-example/blob/master/CONTRIBUTING.md).

## Использование

Если вы хотите прочитать "Gtk-Rust на примерах" онлайн, вы можете посетить [эту страницу][page].

[page]:  http://rurust.github.io/gtk-rust-by-example

Для того, чтобы запустить локально, выполните следующее:

```bash
$ git clone git@github.com:ruRust/gtk-rust-by-example.git
$ cd gtk-rust-by-example
$ cargo install mdbook
$ mdbook build && mdbook serve
```
Откройте браузер и перейдите на `http://localhost:3000`.Теперь книгу можно читать офлайн.


## Лицензия

`Gtk-Rust на примерах` распространяется по двойной лицензии: лицензия [Apache 2.0][apache] и лицензия [MIT][mit].

[apache]: https://github.com/ruRust/gtk-rust-by-example/blob/master/LICENSE-APACHE
[mit]: https://github.com/ruRust/gtk-rust-by-example/blob/master/LICENSE-MIT.md
