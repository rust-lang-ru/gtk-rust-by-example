# Создаем структуру UI

![head_pic](https://mmstick.github.io/gtkrs-tutorials/images/ch03_complete.png)

Используя первую главу как шаблон и расширяя его, мы должны
продумать дизайн пользовательского интерфейса согласно нашему
видению UI, который представлен выше.

Основные элементы интерфейса, которые стоит взять на заметку - это кнопка
**Post** в заголовочной панели. Внутри нашего окна содержимым содержится
элемент **title**, элемент **tags**, текстовая панель **content** и
текстовая панель **right_pane** для отображения текста в виде HTML-разметки.
Таким образом, структура нашего пользовательского интерфейса выглядит так:
```rust
pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub post: Button
}

pub struct Content {
    pub container: Paned,
    pub title: Entry,
    pub tags: Entry,
    pub content: TextBuffer,
    pub right_pane: TextBuffer,
}
```
Заметьте, что контейнер для нашего содержимого будет расположен не в **GtkBox**,
а в **GtkPaned**. Это позволит пользователю перемещать разграничительную полосу
между панелями для того чтобы менять размер панелей по своему усмотрению.
В добавок поля **content** и **right_pane** хранятся как **GtkTextBuffer**,
а не как **GtkTextViews**. Это потому, что мы не будет программировать
панели для просмотра, но будем использовать лежащие в их основе текстовые
буферы, которые связаны с этими панелями.

## Написание программы
Новым здесь будет то, что мы определим для окон размер по умолчанию, потому
что мы должны иметь разумный размер, с которым пользователь будет
взаимодействовать по умолчанию, чтобы пользователю не приходилось менять размер
окна, чтобы удобнее обозревать его содержимое. Мы также изменяем заголовок на
"HTML Articler". Кроме этого, все остальное должно быть похоже на то, как вы
разрабатываете другие приложения.
```rust
impl App {
    fn new() -> App {
        // Создадим новое окно верхнего уровня.
        let window = Window::new(WindowType::Toplevel);
        // Создадим заголовочную панель и связанное с ней содержимое.
        let header = Header::new();
        // Создадим элемент для хранения содержимого.
        let content = Content::new();

        // Сделаем заголовочную панель виджетом в панели с названием.
        window.set_titlebar(&header.container);
        // Установим название окна.
        window.set_title("HTML Articler");
        // Установим управляющий класс для окна.
        window.set_wmclass("html-articler", "HTML Articler");
        // Иконка, которую программа будет показывать.
        Window::set_default_icon_name("iconname");
        // Установить размер окна по умолчанию.
        window.set_default_size(800, 600);
        // Добавим содержимое в окно.
        window.add(&content.container);

        // Запрограммируем, что делать, когда нажата кнопка выхода.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Возвратим главное состояние приложения.
        App { window, header, content }
    }
}
```
## Реализация заголовка
Наша заголовочная панель будет иметь кнопку с надписью **Post**, которая
получит _CSS_-стиль 'suggested-action' и будет расположена в конце панели.
Название заголовочной панели должна совпадать с именем нашего приложения.
```rust
impl Header {
    fn new() -> Header {
        // Создает контейнер для хранения виджета - заголовочной панели.
        let container = HeaderBar::new();

        // Устанавливает текст для показа в области названия в заголовочной панели.
        container.set_title("HTML Articler");
        // Установим элементы управления в области заголовочной панели.
        container.set_show_close_button(true);

        // Создадим кнопку, которая будет выводить HTML-статью.
        let post = Button::new_with_label("Post");
        post.get_style_context().map(|x| x.add_class("suggested-action"));

        container.pack_end(&post);

        // Возвратим заголовок и все его состояние.
        Header { container, post }
    }
}
```
## Реализация содержимого
Это то место, где мы будет тратить большую часть нашего времени, находясь
в приложении. Во-первых, создадим контейнер **GtkPaned**, который будет
содержать левую и правую панели, размер которых изменяем. Нашей правой панелью
будет **GtkTextView**, а правой будет вертикальный **GtkBox**. Заметьте,
мы заинтересованы в том, чтобы получить прямой доступ к тексту внутри буфера
панели для просмотра, так что буфер мы инициализируем перед созданием панели.
Контейнер левой панели будет содержать отступ в `5` условных единицах,
чтобы они не слеплялись вместе.
```rust
// Главный контейнер будет содержать левую и правую панели. Левая панели
// предназначена для пользовательского ввода, в то время как правая
// предназначена для показа сгенерированных выходных данных.
let container = Paned::new(Orientation::Horizontal);
let left_pane = Box::new(Orientation::Vertical, 5);
let right_pane = TextBuffer::new(None);
let right_pane_view = TextView::new_with_buffer(&right_pane);
```
После этого мы создаем элементы **title** и **tags**, в добавок создадим
панель **content**, которую мы будем использовать для построения левой панели.
```rust
// Левая панель будет состоять из заголовка, элемента с тэгами и текстовой
// панели для просмотра содержимого.
let title = Entry::new();
let tags = Entry::new();
let content = TextBuffer::new(None);
let content_view = TextView::new_with_buffer(&content);
```
Заметьте, мы также должны хранить размещенную по центру надпись над текстовой
панелью **content**, после которой мы добавим немного заполняющего текста
в элементах, и пояснительные подсказки, которые будут показываться, при наведении
мыши на элементы.
```rust
// Метка, которую мы будет показывать над элементом с содержимым
// для его описания.
let content_label = Label::new("Content");
content_label.set_halign(Align::Center);

// Показывать пользователю текстовые подсказки в полях для ввода. 
title.set_placeholder_text("Insert Title");
tags.set_placeholder_text("Insert Colon-Delimited Tags");

// Показывать всплывающие подсказки пользователю, которые будут
// проявляться при наведении мыши.
title.set_tooltip_text("Insert the title of article here");
tags.set_tooltip_markup("<b>tag_one</b>:<b>tag two</b>:<b> tag three</b>");
```
После этого сделаем так, чтобы текстовое поле на правой панели не разрешало
редактирование, и оба текстовых поля должны содержать в себе текст, в которым
переносы произведены по словам (а не по слогам).
```rust
// Правая панель не должна разрешать изменять ее содержимое, и оба редактора
// должны производить перенос по целым словам (а не по слогам).
right_pane_view.set_editable(false);
right_pane_view.set_wrap_mode(WrapMode::Word);
content_view.set_wrap_mode(WrapMode::Word);
```
Сейчас мы должны переносить текстовые поля внутри **GtkScrolledWindows** для
того, чтобы позволить пользователю прокручивать текст, на тот случай, если
имеется больше текста, чем доступно места для просмотра внутри элемента.
```rust
// Произвести необходимые переносы текста в прокручиваемых окнах.
let content_scroller = ScrolledWindow::new(None, None);
let right_pane_scrolled = ScrolledWindow::new(None, None);
content_scroller.add(&content_view);
right_pane_scrolled.add(&right_pane_view);
```
Для того, чтобы улучшить пользовательский интерфейс, мы можем добавить поля и
границы.
```rust
// Настраиваем поля.
left_pane.set_border_width(5);
right_pane_view.set_left_margin(5);
right_pane_view.set_right_margin(5);
right_pane_view.set_top_margin(5);
right_pane_view.set_bottom_margin(5);
content_view.set_left_margin(5);
content_view.set_right_margin(5);
content_view.set_top_margin(5);
content_view.set_bottom_margin(5);
```
И все, что остается - поместить элементы внутри соответствующих панелей
и вернуть структуру **Content**.
```rust
// Сначала добавим все в левую панель.
left_pane.pack_start(&title, false, true, 0);
left_pane.pack_start(&tags, false, true, 0);
left_pane.pack_start(&content_label, false, false, 0);
left_pane.pack_start(&content_scroller, true, true, 0);

// После этого добавьте правую и левую панели в контейнер.
container.pack1(&left_pane, true, true);
container.pack2(&right_pane_scrolled, true, true);

Content { container, title, tags, content, right_pane }
```

Собирая все вместе, мы должны получить следующую реализацию:
```rust
impl Content {
    fn new() -> Content {
        // Главный контейнер будет содержать левую и правую панели. Левая панель
        // предназначена для пользовательского ввода, а левая панель
        // предназначена для сгенерированных данных.
        let container = Paned::new(Orientation::Horizontal);
        let left_pane = Box::new(Orientation::Vertical, 5);
        let right_pane = TextBuffer::new(None);
        let right_pane_view = TextView::new_with_buffer(&right_pane);

        //Левая панель будет состоять из заголовка, элементов с тэгами и
        // текстовой панели для просмотра содержимого.
        let title = Entry::new();
        let tags = Entry::new();
        let content = TextBuffer::new(None);
        let content_view = TextView::new_with_buffer(&content);

        // Метка, которую мы будем показывать над содержимым для его описания.
        let content_label = Label::new("Content");
        content_label.set_halign(Align::Center);

        // Показывать пользователю текстовые подсказки в полях для ввода.
        title.set_placeholder_text("Insert Title");
        tags.set_placeholder_text("Insert Colon-Delimited Tags");

        // Показывать всплывающие подсказки пользователю, которые будут
        // проявляться при наведении мыши.
        title.set_tooltip_text("Insert the title of article here");
        tags.set_tooltip_markup("<b>tag_one</b>:<b>tag two</b>:<b> tag three</b>");

        // Правая панель не должна разрешать изменять ее содержимое, и оба редактора
        // должны производить перенос по целым словам (а не по слогам).
        right_pane_view.set_editable(false);
        right_pane_view.set_wrap_mode(WrapMode::Word);
        content_view.set_wrap_mode(WrapMode::Word);

        // Произвести необходимые переносы текста в прокручиваемых окнах.
        let content_scroller = ScrolledWindow::new(None, None);
        let right_pane_scrolled = ScrolledWindow::new(None, None);
        content_scroller.add(&content_view);
        right_pane_scrolled.add(&right_pane_view);

        // Настраиваем отступы.
        left_pane.set_border_width(5);
        right_pane_view.set_left_margin(5);
        right_pane_view.set_right_margin(5);
        right_pane_view.set_top_margin(5);
        right_pane_view.set_bottom_margin(5);
        content_view.set_left_margin(5);
        content_view.set_right_margin(5);
        content_view.set_top_margin(5);
        content_view.set_bottom_margin(5);

        // Сначала добавим все в левую панель.
        left_pane.pack_start(&title, false, true, 0);
        left_pane.pack_start(&tags, false, true, 0);
        left_pane.pack_start(&content_label, false, false, 0);
        left_pane.pack_start(&content_scroller, true, true, 0);

        // После этого добавьте правую и левую панели в контейнер.
        container.pack1(&left_pane, true, true);
        container.pack2(&right_pane_scrolled, true, true);

        Content { container, title, tags, content, right_pane }
    }
}
```