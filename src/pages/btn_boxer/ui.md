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
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();
        // Contains the content within the window.
        let content = Content::new(health);

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("App Name");
        // Set the window manager class.
        window.set_wmclass("app-name", "App name");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");
        // Add the content box into the window.
        window.add(&content.container);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        App { window, header, content }
    }
}
```

## Создание Header

Затем мы так же реализуем метод для нашего заголовка, который должен содержать два элемента **GtkButtons** -- кнопка удара и лечения. Также обратите внимание, что мы устанавливаем некоторые классы стилей этим кнопкам, чтобы дать им более информативную визуальную способность.

```rust
impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("App Name");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Create the hit and heal buttons.
        let hit = Button::new_with_label("Hit");
        let heal = Button::new_with_label("Heal");

        // Add the corresponding style classes to those buttons.
        hit.get_style_context().map(|c| c.add_class("destructive-action"));
        heal.get_style_context().map(|c| c.add_class("suggested-action"));

        // THen add them to the header bar.
        container.pack_start(&hit);
        container.pack_end(&heal);

        // Returns the header and all of it's state
        Header { container, hit, heal }
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
        // Create a vertical box to store all of it's inner children vertically.
        let container = Box::new(Orientation::Vertical, 0);

        // The health info will be contained within a horizontal box within the vertical box.
        let health_info = Box::new(Orientation::Horizontal, 0);
        let health_label = Label::new("Current Health:");
        let health = Label::new(health.get_health().to_string().as_str());

        // Set the horizontal alignments of each of our objects.
        health_info.set_halign(Align::Center);
        health_label.set_halign(Align::Start);
        health.set_halign(Align::Start);


        // Add the health info box's children
        health_info.pack_start(&health_label, false, false, 5);
        health_info.pack_start(&health, true, true, 5);

        // Create a message label that will later be modified by the application, upon
        // performing a hit or heal action.
        let message = Label::new("Hello");

        // Add everything to our vertical box
        container.pack_start(&health_info, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&message, true, false, 0);

        Content { container, health, message }
    }
}
```

### Устанавливаем выравнивание

Возможно, вы заметили, что приведённый код выше устанавливает горизонтальные выравнивания.
По желанию, с помощью методов `set_halign()` и `set_valign()`, можно установить выравнивание для виджетов.

```rust
// Set the horizontal alignments of each of our objects.
health_info.set_halign(Align::Center);
health_label.set_halign(Align::Start);
health.set_halign(Align::Start);
```