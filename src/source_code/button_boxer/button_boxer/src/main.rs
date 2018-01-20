extern crate gtk;
use gtk::*;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// Заданные сообщения, которые будут использоваться в UI
// при определённых условиях.
const MESSAGES: [&str; 3] = [
    "Ой! Ты ударил меня!",
    "...",
    "Спасибо!",
];

#[repr(u8)]
// Типаж с типом `u8`, который используется как индекс в массиве `MESSAGES`.
enum Message {
    Hit,
    Dead,
    Heal,
}

fn main() {
    // Инициализируем GTK перед продолжением.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    /*  Установим начальное состояние для нашего компонента - `health`.
     *   Воспользуемся `Arc`, для того, чтобы мы могли
     *   использовать несколько programmable замыканий.
     */
    let health = Arc::new(HealthComponent::new(10));

    // Инициализируем начальное состояние UI.
    let app = App::new(&health);

    {
        // Запрограммируем кнопку `Ударить` чтобы уменьшить здоровье.
        let health = health.clone();
        let message = app.content.message.clone();
        let info = app.content.health.clone();
        app.header.hit.connect_clicked(move |_| {
            let new_health = health.subtract(1);
            let action = if new_health == 0 {
                Message::Dead
            } else {
                Message::Hit
            };
            message.set_label(MESSAGES[action as usize]);
            info.set_label(new_health.to_string().as_str());
        });
    }

    {
        // Запрограммируем кнопку `Лечить`, чтобы вернуть очки здоровья.
        let health = health.clone();
        let message = app.content.message.clone();
        let info = app.content.health.clone();
        app.header.heal.connect_clicked(move |_| {
            let new_health = health.heal(5);
            message.set_label(MESSAGES[Message::Heal as usize]);
            info.set_label(new_health.to_string().as_str());
        });
    }

    // Сделаем все виджеты видимыми в UI.
    app.window.show_all();

    // Запуск основного цикла GTK.
    gtk::main();
}

pub struct HealthComponent(AtomicUsize);

impl HealthComponent {
    fn new(initial: usize) -> HealthComponent {
        HealthComponent(AtomicUsize::new(initial))
    }

    fn get_health(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value { 0 } else { current - value };
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub hit: Button,
    pub heal: Button,
}

pub struct Content {
    pub container: Box,
    pub health: Label,
    pub message: Label,
}

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
