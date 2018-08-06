# Диалог выбора файла

> Заметьте, что это модуль dialogs.rs

GTK Rust API не используют преимущества типажа **Drop** в Rust, поэтому при появлении диалога, он будет всегда находится на вашем экране. К счастью, мы можем решить эту проблему самостоятельно, создав тип для **GtkFileChooserDialogs** и реализовать типаж **Drop**, чтобы разрушить внутренний диалог после сброса типа.

## Создание OpenDialog

Начнём с создания простой кортёжной структуры.

```rust
pub struct OpenDialog(FileChooserDialog);
```

Реализуем простой метод **new()** для этой структуры и создадим внутренний **FileChooserDialog**.

```rust
impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        // Создадим новый диалог выбора файлов, чтобы открыть их.
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        // Добавим кнопки отмены и открытия для этого диалога.
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        // Установим стандартный путь открытия файла.
        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }
}
```

Смысл **FileChooserDialog** состоит в том, чтобы дать имя диалогу и предоставить новое окно с типом **Popup**, а затем выбрать соответствующее **FileChooserAction**,
который нужен вашим критериям при использовании. В этом случае, мы
создадим диалог **Open**, чтобы открыть файл.

После создания установите label для двух
кнопок внутри диалога и установите нужный **ResponseTypes** с этими кнопками.
Очень важно знать, нажал ли пользователь **Cancel** или **Ok**.

Как только закончим, нам необходимо обернуть тип в нашем **OpenDialog**.

## Создание SaveDialog

Диалог Save практически идентичен.

```rust
pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        // Инициализируем новый даилог встроенный в всплывающее окно.
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        // Добавим кнопки cancel и save к диалогу.
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        // Установим стандартный путь открытия файла.
        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }
}
```

## Реализация типажа Drop

Объекты в GTK разрушаются с помощью метода **destroy()**. Если вы хотите разрушить наш диалог после того, как мы сбросили их обращение (handles), мы можем сделать это автоматически реализуя типаж **Drop** на обоих типах.

```rust
impl Drop for OpenDialog {
    fn drop(&mut self) { self.0.destroy(); }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}
```

Глупо, но просто. Мы вызываем метод **destroy()**
для внутреннего значения наших новых типов.

## Реализация полезного метода

Следующий метод может быть добавлен в блок impl для обоих типов и они
упрощают запуск и получают данные на выходе, которые мы хотим.

```rust
pub fn run(&self) -> Option<PathBuf> {
    if self.0.run() == ResponseType::Ok.into() {
        self.0.get_filename()
    } else {
        None
    }
}
```

В основном мы показываем/запускаем диалог и проверяем вывод, чтобы определить,
получили ли мы ответ **Ok**. Если ответ **Ok**, мы просто пытаемся вернуть имя файла, которое существует или не существует.

## Весь код

```rust
use gtk::*;
use std::path::PathBuf;

/// Обёрнутый FileChooserDialog автоматически разрушающийся при сбрасывании.
pub struct OpenDialog(FileChooserDialog);

/// Обёрнутый FileChooserDialog автоматически разрушающийся при сбрасывании.
pub struct SaveDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new() -> OpenDialog {
        // Создадим новый диалог выбора файлов, чтобы открыть их.
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        // Добавим кнопки cancel и open для этого диалога.
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new() -> SaveDialog {
        // Инициализируем новый диалог, встроенный в всплывающее окно.
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        // Добавим кнопки cancel и save к диалогу.
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) { self.0.destroy(); }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}
```
