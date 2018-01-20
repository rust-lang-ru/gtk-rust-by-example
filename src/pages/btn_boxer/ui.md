# Создание структуры пользовательского интерфейса (UI)

Используя структуру предыдущей главы в качестве образца, мы можем расширить её, чтобы включить новые элементы UI, которые будем использовать в нашей программе. It is important to note that you only need to store elements that you are going to later program after the UI is constructed.

В этой программе мы добавим два **GtkButtons** к панели заголовка и воспользуемся вертикальной и горизонтальной **GtkBox** с некоторыми метками (labels), чтобы отобразить информацию о текущем состоянии нашего приложения. Следующее изображение является нашей новой диаграммой структур.

<img src="img/btn_diagram.png" />

Что означает следующее в Rust:

```rust
pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub hit:       Button,
    pub heal:      Button,
}

pub struct Content {
    pub container: Box,
    pub health:    Label,
    pub message:   Label,
}
```

## Создание структуры App

Следуя последнему уроку, начнём с нашей структуры **App**. Метод **new()** должен принимать ссылку на **&HealthComponent** в качестве вводимого значения в UI, later on
down the road within our **Content** structure. One will note that we have added a new
**content** variable of type **Context**, which takes that health reference.

```rust
impl App {
    fn new(health: &HealthComponent) -> App {
        // Создадим новое окно с типом `Toplevel`.
        let window = Window::new(WindowType::Toplevel);
        // Создадим заголовок и связанное с ним содержимое.
        let header = Header::new();
        // Расположим содержимое в окне.
        let content = Content::new(health);

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Установим описание для окна.
        window.set_title("Боксирующие кнопки");
        // Установим класс для оконного менеджера.
        window.set_wmclass("app-name", "Боксирующие кнопки");
        // Установим иконку, отображаемую приложением.
        Window::set_default_icon_name("имя-иконки");
        // Добавим коробку с содержимым в окно.
        window.add(&content.container);

        // Запрограммируем выход из программы при нажатии кнопки.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Вернём состояние нашего приложения.
        App {
            window,
            header,
            content,
        }
    }
}
```

## Создание Header

Затем мы так же реализуем метод для нашего заголовка, который должен содержать два элемента **GtkButtons** -- кнопка удара и лечения. Также обратите внимание, что мы устанавливаем некоторые классы стилей этим кнопкам, чтобы дать им более информативную визуальную способность.

```rust
impl Header {
    fn new() -> Header {
        // Создадим главный заголовочный бар содержащий виджет.
        let container = HeaderBar::new();

        // Установим текст для отображения в секции для названия.
        container.set_title("Боксирующие кнопки");
        // Сделаем активными элементы управления окна в этой панели.
        container.set_show_close_button(true);

        // Создадим кнопки: `ударить` и `лечить`.
        let hit = Button::new_with_label("Ударить");
        let heal = Button::new_with_label("Лечить");

        // Добавим соответствующие классы стилей к этим кнопкам.
        hit.get_style_context()
            .map(|c| c.add_class("destructive-action"));
        heal.get_style_context()
            .map(|c| c.add_class("suggested-action"));

        // Теперь добавим их в панель заголовка.
        container.pack_start(&hit);
        container.pack_end(&heal);

        // Вернём the header and all of it's state
        Header {
            container,
            hit,
            heal,
        }
    }
}
```

## Создание Content

Теперь пришло время создать содержимое для нашего окна. При создании своего интерфейса (UI) с древовидной диаграммой, вы почти достигните **GtkBoxes**. При инициализации, эта упаковка должна быть указана с **Horizontal** или  **Vertical** оринтацией.

You will amost certainly reach for **GtkBoxes** for configuring your UI. These can be created with either a **Horizontal** or **Vertical** alignment. These boxes are where you will add all of your widgets, where they will be stacked according to the alignment of the box they are attached to.

Мы должны создать вертикальную упаковку, которая содержит два дочерних элемента: вертикальный **GtkBox** содержащий метку (label) и значение, а затем ниже простой **GtkLabel**.

```rust
impl Content {
    fn new(health: &HealthComponent) -> Content {
        // Создадим вертикальную упаковку, чтобы хранить там все дочерние элементы.
        let container = Box::new(Orientation::Vertical, 0);

        // Информация о здоровье будет храниться в горизонтальной упаковке вместе с вертикальной.
        let health_info = Box::new(Orientation::Horizontal, 0);
        let health_label = Label::new("Текущее значение здоровья:");
        let health = Label::new(health.get_health().to_string().as_str());

        // Установим горизонтальное выравнивание для наших объектов.
        health_info.set_halign(Align::Center);
        health_label.set_halign(Align::Start);
        health.set_halign(Align::Start);

        // Добивим информацию о здоровье в дочернюю коробку.
        health_info.pack_start(&health_label, false, false, 5);
        health_info.pack_start(&health, true, true, 5);

        /*
         *   Создадим метку, которая будет изменяться приложением
         *   при выполнении удара или лечения.
         */
        let message = Label::new("Привет");

        // Добавим все в нашу вертикальную коробку.
        container.pack_start(&health_info, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&message, true, false, 0);

        Content {
            container,
            health,
            message,
        }
    }
}
```

### Устанавливаем выравнивание

Возможно, вы заметили, что приведённый код выше устанавливает горизонтальные выравнивания.
По желанию, с помощью методов `set_halign()` и `set_valign()`, можно установить выравнивание для виджетов.

```rust
// Установим горизонтальное выравнивание для наших объектов.
health_info.set_halign(Align::Center);
health_label.set_halign(Align::Start);
health.set_halign(Align::Start);
```