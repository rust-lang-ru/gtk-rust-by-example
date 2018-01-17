extern crate gtk;

use gtk::*;

// Объявим структуру `Application`.
pub struct Application {
    pub window: Window,
    pub header: Header,
}

// Объявим структуру `Header`.
pub struct Header {
    pub container: HeaderBar,
}

// Блок реализации.
impl Application {
    fn new() -> Application {
        // Создадим новое окно с типом `Toplevel`.
        let window = Window::new(WindowType::Toplevel);
        // Создадим header bar и и связанный с ним контент.
        let header = Header::new();

        // Укажем название заголовка виджета.
        window.set_titlebar(&header.container);
        // Укажем название для окна приложения.
        window.set_title("Простая программа");
        // Установим класс для оконного менеджера.
        window.set_wmclass("simple-gtk", "Простая программа");
        // Установим иконку, отображаемую приложением.
        Window::set_default_icon_name("имя иконки");

        // Программа закроется, если нажата кнопка выхода.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Возвращаем основное состояние приложения.
        Application { window, header }
    }
}

impl Header {
    fn new() -> Header {
        // Создадим виджет контейнера для главной панели заголовка.
        let container = HeaderBar::new();
        // Установим отображаемый тект в секции для названия.
        container.set_title("Простая программа");
        // Делаем активными элементы управления окна в этой панели.
        container.set_show_close_button(true);

        // Возвращаем заголовок и его состояние.
        Header { container }
    }
}

fn main() {
    // Инициализация GTK.
    if gtk::init().is_err() {
        eprintln!("Не удалось инициализировать GTK приложение.");
        return;
    }

    // Инициализация начального состояния UI.
    let app = Application::new();

    // Делаем видимыми все виджеты с UI.
    app.window.show_all();

    // Запуск основного цикла GTK.
    gtk::main();
}
