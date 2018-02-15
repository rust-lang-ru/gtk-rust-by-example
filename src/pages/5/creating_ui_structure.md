# Создание структуры пользовательского интерфейса
TODO: вставить ссылку на картинку
## Реализация модуля пользовательского интерфейса (ui/mod.rs)
Теперь, когда относящийся к UI код перемещен в свой собственный модуль,
очень важно объявить подмодули, которые данный модуль будет импортировать.
Также важно объявить типы и функции, который данный модуль будет
экспортировать.
```rust
mod app;
mod content;
mod dialogs;
mod header;
pub mod misc;
pub mod save;

pub use self::app::App;
pub use self::content::Content;
pub use self::dialogs::{OpenDialog, SaveDialog};
pub use self::header::Header;
```
Как можно заметить выше, мы создаем следующие модули: **app.rs**,
**content.rs**, **dialogs.rs**, **save.rs**, **header.rs**, **misc.rs**.

## Реализация структуры программы (ui/app.rs)
Данная часть должна быть более-менее простой, так как использует код, подобный
тому, что был задействован в предыдущих главах. С той разницей, что мы
переместили инициализацию GTK в начало **App**.
```rust
use gtk;
use gtk::*;
use super::Header;
use super::Content;

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        // Инициализация GTK.
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        // Создать окно верхнего уровня.
        let window = Window::new(WindowType::Toplevel);
        // Создать заголовочную панель и связанное с ней содержимое.
        let header = Header::new();
        // Создать контейнер для хранения содержимого и виджетов.
        let content = Content::new();

        // Сделать заголовочную панель виджетом, содержащим название.
        window.set_titlebar(&header.container);
        // Установить заголовок окна.
        window.set_title("Markdown Editor");
        // Установить класс менеджера окна.
        window.set_wmclass("md-editor", "Markdown Editor");
        // Иконка программы.
        window.set_default_size(800, 600);
        Window::set_default_icon_name("iconname");
        // Добавить содержимое вовнутрь окна.
        window.add(&content.container);

        // Запрограммировать поведение кнопки выхода.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Возвратить структуру программы.
        App { window, header, content }
    }
}
``` 
## Реализация структуры заголовка (ui/header.rs)
Данную структуру будет реализовать ещё проще. Мы реализуем кнопки **Open**,
**Save**, **Save As**, которые будут находиться в заголовке. Мы будем
использовать мнемоники, так что одна из букв в названии кнопок будет
становиться подчеркнутой при нажатии на клавишу **Alt**, так что пользователь
сможет выбрать нужную кнопку.
```rust
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open:      Button,
    pub save:      Button,
    pub save_as:   Button,
}

impl Header {
    pub fn new() -> Header {
        // Создать контейнер для хранения главной заголовочной панели. 
        let container = HeaderBar::new();

        // Установить текст, который будет показывать в заголовке
        // заголовочной панели.
        container.set_title("Markdown Editor");
        // Включить элементы управления окна внутри заголовочной панели.
        container.set_show_close_button(true);

        let open = Button::new_with_mnemonic("_Open");
        let save = Button::new_with_mnemonic("_Save");
        let save_as = Button::new_with_mnemonic("Save _As");
        container.pack_start(&open);
        container.pack_end(&save_as);
        container.pack_end(&save);

        // Возвратить заголовок и все его внутреннее содержимое.
        Header { container, open, save, save_as }
    }
}
```
## Реализация структуры содержимого (ui/content.rs)
Здесь мы начнем использовать **GtkWebViews**, **GtkSourceViews**.
### Создание просмотра web
Создание просмотра web очень легко создать. Выполняя следующий код, вы получите
панель для просмотра web, которую вы можете внедрить в ваш пользовательский
интерфейс так же, как вы можете это сделать с другими виджетами.
```rust
// Create a the WebView for the preview pane.
let context = WebContext::get_default().unwrap();
let preview = WebView::new_with_context(&context);
```
Данный просмотр web прокручиваем, так что эту функциональность не нужно
реализовывать самостоятельно.
### Создаем и настраиваем просмотр кода
Просмотр кода являются более сложной реализацией, потому что он требуют
дополнительной настройки. Мы попытаемся получить следующий результат:
- Табуляция должна быть размером в 4 пробела
- Просмотрщик должен показывать номера строк
- Будем использовать шрифт по умолчанию - _monospaced_, размер - 11
- Подсветка синтаксиса Markdown
- Попытка использовать тему Builder, если не получится - тему Classic

Начнем со структуры **Source**.
```rust
pub struct Source {
    pub container: ScrolledWindow,
    pub view:      View,
    pub buff:      Buffer,
}

impl Source {
    fn new() -> Source {

    }
}
```
После этого создадим буфер и представление для просмотра, это делается
так же, как и создание простого текстового буфера и представления для
просмотра. Как только это сделано, мы поместим представление вовнутрь
прокручиваемого окна.
```rust
// Создать SourceView для редактора на левой панели.
let buff = Buffer::new(None);
let view = View::new_with_buffer(&buff);
let container = ScrolledWindow::new(None, None);
container.add(&view);
```
Установим настройки, используя функцию, принимающую представление и буфер:
```rust
fn configure_source_view(view: &View, buff: &Buffer) {
    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_show_right_margin(true);
    view.set_background_pattern(BackgroundPatternType::Grid);
    // TODO: следующий релиз пакета GTK
    // view.set_input_hints(InputHints::SPELLCHECK + InputHints::WORD_COMPLETION);
}
```
Мы можем использовать пакет **pango** для изменения шрифта в представлении.
Заметьте, нам необходимо объявить типаж, чтобы понять откуда появился метод
`override_font`. Возможно, будущее обновление GTK Rust API займется решением
этого вопроса.
```rust
// Настраивает шрифт, используемый в панели с кодом. По умолчанию используется
// Monospaced, размер - 11. Когда будем изменять шрифт, нужно вручную указать
// типаж, в котором находится нужный метод.
let font = FontDescription::from_string("monospace 11");
WidgetExt::override_font(&view, &font);
```
Включим подсветку синтаксиса Markdown по умолчанию. Используемые языки
извлекаются из **LanguageManager**. Язык будет присвоен напрямую буферу,
а не представлению.
```rust
// Включить подсветку Markdown
LanguageManager::new()
    .get_language("markdown")
    .map(|markdown| buff.set_language(&markdown));
```
Укажем используемую схему.
```rust
let manager = StyleSchemeManager::new();
    manager
        .get_scheme("Builder")
        .or(manager.get_scheme("Classic"))
        .map(|theme| buff.set_style_scheme(&theme));
```
## Полный исходный код
```rust
use gtk::*;
use pango::*;
use sourceview::*;
use webkit2gtk::*;

pub struct Content {
    pub container: Paned,
    pub source:    Source,
    pub preview:   WebView,
}

pub struct Source {
    pub container: ScrolledWindow,
    pub view:      View,
    pub buff:      Buffer,
}

impl Content {
    pub fn new() -> Content {
        // Создать контейнер для хранения главного содержимого
        let container = Paned::new(Orientation::Horizontal);
        let source = Source::new();

        // Создать WebView для предыдущей панели.
        let context = WebContext::get_default().unwrap();
        let preview = WebView::new_with_context(&context);

        // Упакуем
        container.pack1(&source.container, true, true);
        container.pack2(&preview, true, true);

        // Сделать так, чтобы две панели имели одинаковый размер - половина
        // контейнера, в котором они находятся.
        source.container.set_size_request(100, -1);
        preview.set_size_request(100, -1);

        Content { container, source, preview }
    }
}

impl Source {
    pub fn new() -> Source {
        // Создать SourceView для редактора на левой панели.
        let buff = Buffer::new(None);
        let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(None, None);
        container.add(&view);

        configure_source_view(&view, &buff);

        Source { container, buff, view }
    }
}

fn configure_source_view(view: &View, buff: &Buffer) {
    WidgetExt::override_font(view, &FontDescription::from_string("monospace"));

    LanguageManager::new()
        .get_language("markdown")
        .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("Builder")
        .or(manager.get_scheme("Classic"))
        .map(|theme| buff.set_style_scheme(&theme));

    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_show_right_margin(true);
    view.set_background_pattern(BackgroundPatternType::Grid);
    // TODO: следующий релиз пакета GTK
    // view.set_input_hints(InputHints::SPELLCHECK + InputHints::WORD_COMPLETION);
}
```