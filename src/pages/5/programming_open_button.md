# Программирование кнопки Open
Самое время запрограммировать логику работы кнопок **Open**, **Save**,
**Save As** в нашем пользовательском интерфейсе. Начнём с реализации кнопки
**Open**, создав новый метод **open_file()**, который будет вызван из
метода **connect_events()**.

## Кнопка Open
Функция кнопки **open_file()** получит доступ к переменной **current_file**,
которая будет обновлена после успешного открытия выбранного файла.
```rust
pub fn connect_events(self) -> ConnectedApp {
    // Внешнее состояние, которое доступно для событий.
    let current_file = Arc::new(RwLock::new(None));

    // Подсоединить события, обрабатываемые пользовательским интерфейсом.
    self.editor_changed(current_file.clone(), &self.header.save.clone());
    self.open_file(current_file.clone());

    // Обернуть `ConnectedApp` вокруг `App` для того, чтобы дать возможность
    // разработчику выполнить программу.
    ConnectedApp(self)
}
```
### connect_clicked()
Метод **open_file()** будет получать:
- ссылки на буфер редактора и писать информацию из открытого файла в буфер.
- ссылки на панель web просмотра, так что мы сможем обновлять её после
открытия файла.
- ссылку на заголовочную панель, так что мы сможем обновлять название.
- кнопку **Open**, так что мы сможем отобразить событие **connect_clicked()**
на неё.
```rust
fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
    let editor = self.content.source.buff.clone();
    let preview = self.content.preview.clone();
    let headerbar = self.header.container.clone();
    self.header.open.connect_clicked(move |_| {
        // Программировать кнопку здесь.
    });
}
```
## Создание OpenDialog
Создадим новый **OpenDialog** внутри **connect_clicked()**. Открывая
диалоговое окно, мы попытаемся передать родительскую директорию в
**current_file**, если она существует, поэтому окно для открытия файла по
умолчанию использует данную директорию.
> Заметьте, я здесь использую **if let Some(ref path)**, а не просто **map**
из-за ограничений на заимствования - если нельзя добиться того, что **map**
будет правильно заимствовать, следует использовать **match** или **if let**.

```rust
// Создать диалоговое окно для открытия файла, используя родительскую
// директорию в качестве предпочитаемой, если он установлен.
let open_dialog = OpenDialog::new({
    let lock = current_file.read().unwrap();
    if let Some(ref path) = *lock {
        path.get_dir()
    } else {
        None
    }
});
```
## Запуск диалогового окна
После получения переменной **open_dialog**, мы можем запустить диалог, а также:
- захватить выбранный путь к файлу
- считать данные из файла в буфер
- обновить панель для web просмотра
- обновить название

```rust
// Запускает диалоговое окно и открывает файл, если он был выбран.
if let Some(new_file) = open_dialog.run() {
    if let Ok(mut file) = File::open(&new_file) {
        // Считать содержимое файла в находящийся в памяти буфер.
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        // Обновить название.
        set_title(&headerbar, &new_file);
        if let Some(parent) = new_file.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            headerbar.set_subtitle(subtitle);
        }

        // Установить публично доступный путь к файлу.
        *current_file.write().unwrap() =
            Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

        // Обновить содержимое редактора и предпросмотровую панель.
        editor.set_text(&contents);
        preview.load_html(&render(&contents), None);
    }
}
```
## Полный код
```rust
fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
    let editor = self.content.source.buff.clone();
    let preview = self.content.preview.clone();
    let headerbar = self.header.container.clone();
    self.header.open.connect_clicked(move |_| {
        // Создать диалоговое окно для открытия файла, используя родительскую
        // директорию в качестве предпочитаемой, если он установлен.
        let open_dialog = OpenDialog::new({
            let lock = current_file.read().unwrap();
            if let Some(ref path) = *lock {
                path.get_dir()
            } else {
                None
            }
        });

        // Запускает диалоговое окно и открывает файл, если он был выбран.
        if let Some(new_file) = open_dialog.run() {
            if let Ok(mut file) = File::open(&new_file) {
                // Считать содержимое файла в находящийся в памяти буфер.
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);

                // Обновить название.
                set_title(&headerbar, &new_file);
                if let Some(parent) = new_file.parent() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                }

                // Установить публично доступный путь к файлу.
                *current_file.write().unwrap() =
                    Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

                // Обновить содержимое редактора и предпросмотровую панель.
                editor.set_text(&contents);
                preview.load_html(&render(&contents), None);
            }
        }
    });
}
```