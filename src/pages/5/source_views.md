# Выбор файла, просмотр кода и web-страниц
## GtkWebView

![pic_gtkwebview](https://mmstick.github.io/gtkrs-tutorials/images/web_view.png)

Этот виджет, предоставляемый [webkit2gtk][], который содержит прокручиваемое
окно и предоставляет встроенный web-движок для рендеринга HTML. Данный элемент
нужен для показа отрендеренного HTML, полученного из текста Markdown в буфере.
```rust
let context = WebContext::get_default().unwrap();
let webview = WebView::new_with_context(&context);
```
Мы поместим HTML, который получен посредством использования пакета horrowshow,
в web-панель напрямую, с помощью метода `load_html()`.
```rust
webview.load_html(&html, None);
```
## GtkSourceView
![pic_gtkwebview]: https://mmstick.github.io/gtkrs-tutorials/images/source_view.png
Это предоставляемый нам усовершенствованный **GtkTextView**. Однако не
ожидайте слишком многого от него, потому что на текущий момент он достаточно
примитивен.
```rust
let source_buffer = Buffer::new(None);
let source_view = View::new_with_buffer(&source_buffer);
```
Настройки по умолчанию не отличаются от соответствующих настроек **GtkTextView**,
поэтому вам нужно будет самостоятельно настроить виджет.
```rust
source_view.set_show_line_numbers(true);
source_view.set_monospace(true);
let font = FontDescription::from_string("monospace 11");
WidgetExt::override_font(&source_view, &font);
```
## GtkFileChooserDialog
![pic_GtkFileChooserDialog](https://mmstick.github.io/gtkrs-tutorials/images/file_chooser_dialog.png)
**GtkFileChooserDialogs** будет использоваться для программирования поведения
кнопок для открытия/закрытия/сохранения файлов. Они будут открывать окно, где
вы сможете выбрать файл. Важный момент: **GtkFileChooserDialogs** не
использует возможности типажа **Drop**. Но беспокоиться не стоит, все нужное
мы реализуем самостоятельно.
```rust
// Создать новое диалоговое окно выбора файла для открытия.
let open_dialog = FileChooserDialog::new(
    Some("Open"),
    Some(&Window::new(WindowType::Popup)),
    FileChooserAction::Open,
);

// Добавить кнопки **Cancel**, **Save** в это диалоговое окно.
open_dialog.add_button("Cancel", ResponseType::Cancel.into());
open_dialog.add_button("Open", ResponseType::Ok.into());

// Открыть созданное диалоговое окно и принять полученный результат.
if open_dialog.run() == ResponseType::Ok.into() {
    if let Some(filename) = open_dialog.get_filename() {
        // Сделать что-то с полученным `PathBuf`.
    }
}

// Уничтожить диалоговое окно. Будьте внимательны: не возвращайтесь из
// функции, не уничтожив прежде диалоговое окно.
open_dialog.destroy();
```