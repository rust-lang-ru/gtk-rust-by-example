# Обработка сочетаний клавиш
Добавим обработку некоторых сочетаний клавиш.
- **F11** - перейти в полноэкранный режим.
- **Ctrl+S** - сохранить файл.

## Полноэкранный режим и **App::connect_events()**
Вы хотим знать, когда мы должны вызывать метод **Window::fulscreen()** или
метод **Window::unfullscreen()**, поэтому нам важно сохранять состояние
нахождения в полноэкранном режиме в специальной логической переменной
**AtomicBool**. Эта переменная будет передана в **App::key_events()**, который
также примет ссылку на **ActiveMetadata** текущего файла для проведения
необходимых операций с файлом, например, сохранение.
```rust
pub fn connect_events(self) -> ConnectedApp {
    // Внешнее состояние, которое доступно для событий.
    let current_file = Arc::new(RwLock::new(None));
    // Отслеживать то, находится ли программа в полноэкранном режиме.
    let fullscreen = Arc::new(AtomicBool::new(false));

    {
        let save = &self.header.save;
        let save_as = &self.header.save_as;

        // Присоединить все события, которые данные программа будет
        // обрабатывать.
        self.editor_changed(current_file.clone(), &save.clone());
        self.open_file(current_file.clone());
        self.save_event(&save.clone(), &save.clone(), current_file.clone(), false);
        self.save_event(&save, &save_as, current_file.clone(), true);
        self.key_events(current_file, fullscreen);
    }

    // Обернуть `App` в `ConnectedApp` для того, чтобы дать возможность
    // разработчику выполнять программу.
    ConnectedApp(self)
}
```
## Реализация метода **App::key_events()**
То место, где необходимо использовать пакет **gdk**: он поможет нам определить
клавиши, на которые нажал пользователь, и были ли активированы некоторые
состояния (подобно удерживанию Ctrl во время ввода).

Используя **connect_key_press_event()** в главном окне, мы можем обрабатывать
нажатия клавиш. Вам нужно сопоставлять полученное состояние с определенными
клавишами и выполнять функциональность, которая вам нужна.
```rust
/// Обрабатывает нажатия определенных сочетаний клавиш.
fn key_events(
    &self,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    fullscreen: Arc<AtomicBool>,
) {
    // Получить необходимые ссылки заранее.
    let editor = self.content.source.buff.clone();
    let headerbar = self.header.container.clone();
    let save_button = self.header.save.clone();

    // Каждое нажатие кнопки вызовет эту функцию.
    self.window.connect_key_press_event(move |window, gdk| {
        match gdk.get_keyval() {
            // Перевести программу в полноэкранный режим при нажатии на F11.
            key::F11 => if fullscreen.fetch_xor(true, Ordering::SeqCst) {
                window.unfullscreen();
            } else {
                window.fullscreen();
            },
            // Сохранить файл при нажатии Ctrl+S
            key if key == 's' as u32 && gdk.get_state().contains(CONTROL_MASK) => {
                save(&editor, &headerbar, &save_button, &current_file, false);
            }
            _ => (),
        }
        Inhibit(false)
    });
}
```