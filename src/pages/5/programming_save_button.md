# Программирование кнопки Save
Кнопки **Save** и **Save As** имеют более сложную реализацию, частично из-за
того, что нужно иногда менять состояние кнопки **Save**, а также из-за того,
что кнопка **Save** не должна открывать диалоговое окно когда имеется активный
файл - она должна немедленно сохранять его на диск.
## App:save_event()
Объявим метод **App::save_event()**, который мы будем использовать в методе
**App::connect_events()** и поместим детали реализации в функцию **save()**
модуля **save.rs**.

Ключевые параметры, которые нам нужны:
- параметр **save_button**, который мы будем программировать.
- кнопка **Save**, с именем **actual_button**.
- доступ к **ActiveMetadata** текущего файла.
- обозначение того, является ли параметр **save_button** кнопкой
**Save As** или нет.
```rust
// Используется для программирования кнопок **Save** и **Save As**.
fn save_event(
    &self,
    save_button: &Button,
    actual_button: &Button,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    save_as: bool,
) {
    let editor = self.content.source.buff.clone();
    let headerbar = self.header.container.clone();
    let save_button = save_button.clone();
    actual_button.connect_clicked(
        move |_| save(&editor, &headerbar, &save_button, &current_file, save_as),
    );
}
```
## Обновленный App:connect_event()
Далее, нужно написать метод **App::connect_events()** так, чтобы мы могли
передать в него два новых метода:
- первый - программирует кнопку **Save**.
- второй - программирует кнопку **Save As**.
```rust
/// Создать внешнее состояние и отобразить всю функциональность UI на
/// пользовательский интерфейс.
pub fn connect_events(self) -> ConnectedApp {
    // Внешнее состояние, доступное для событий.
    let current_file = Arc::new(RwLock::new(None));
    // Отслеживать, открыта ли программа на весь экран или нет.
    let fullscreen = Arc::new(AtomicBool::new(false));

    {
        let save = &self.header.save;
        let save_as = &self.header.save_as;

        // Присоединить все события, который наш пользовательский интерфейс
        // будет обрабатывать.
        self.editor_changed(current_file.clone(), &save.clone());
        self.open_file(current_file.clone());
        self.save_event(&save.clone(), &save.clone(), current_file.clone(), false);
        self.save_event(&save, &save_as, current_file.clone(), true);
    }

    // Обернуть `ConnectedApp` вокруг `App` для того, чтобы дать возможность
    // разработчику выполнять программу.
    ConnectedApp(self)
}
```
### Реализация модуля **save.rs**
Начнём с добавления необходимых импортов:
```rust
use super::SaveDialog;
use super::misc::*;
use gtk::*;
use sourceview::*;
use state::ActiveMetadata;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;
```
Мы хотим знать, был ли сохранён новый файл, перезаписан ли текущий или
сохранение было прервано.Создадим перечисление для представления различных
состояний:
```rust
pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}
```
## Запись данных и получение SaveAction
Приватная функция **write_data()** будет использована для записи данного
буфера (**data**) в файл и оповещения о результате действия по сохранению
файла. Если это была кнопка **Save** и сейчас существует **ActiveMetadata**,
тогда данные будут просто записаны в существующий файл и возвращен
**Ok(SaveAction::Saved)**.

В противном случае, если была нажата кнопка **Save As**, или была нажата
кнопка **Save**, но не было **ActiveMetadata**, тогда будет запущен
**SaveDialog** для получения нового файла и возвращения
**Ok(SaveAction::New(ActiveMetadata))**. Мы возвратим
**Ok(SaveAction::Cancelled)**, если пользователь нажал на кнопку отмены в
диалоговом окне.
```rust
/// Сохраняет данные в файл, находящийся по предоставленному пути. Если путь
/// является **None**, окно сохранения файла будет запущено для получения пути
/// от пользователя. Если будет запущено диалоговое окно, данная функция
/// возвратит **Ok(Some(path))**, иначе - **Ok(None)**. Значение **Err**
/// указывает на связанную с вводом/выводом ошибку, которая произошла при
/// попытке сохранения файла. 
fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        // Сохранить данные в предоставленный файл, предварительно усекая его.
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(true).open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }
    
    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(false).open(&new_path)?;
        file.write_all(data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}
```
### Написание функции **save()**
Напишем функцию **save()** в этом модуле. Первым шагом получим текст из
буфера. Потом предоставим метаинформацию о файле в зависимости от того, была
нажата кнопка **Save As** или **Save**. После этого обработает полученный
**SaveAction**:
- вариант **New** предоставит новую метаинформацию, которую мы сохраним как
текущую и обновим название.
- вариант **Saved** оповестит нас о том, что мы должны вычислить хеш текста,
который был записан на диск, и обновить текущую хеш-сумму файла для отражения
нового состояния файла.
```rust
pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        // Если мы программируем кнопку **Save As**, то мы не будем
        // использовать текущий путь. В противном случае мы сохраним текст в
        // редакторе в находящийся по текущему пути файл, если этот путь имеется.
        let result = if save_as {
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), text.as_bytes())
        };

        // Сейчас мы подберем соответствие к выведенному функцией **save()**
        // результату. Мы будем обрабатывать случай, когда возвращенное
        // значение соответствует шаблону **Ok(Some(ActiveMetadata))**,
        // устанавливая название заголовочной панели, и путь, который мы
        // получили, в качестве текущего файла.
        match result {
            Ok(SaveAction::New(file)) => {
                // Обновить название.
                set_title(&headerbar, file.get_path());
                if let Some(parent) = file.get_dir() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                }
                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }
            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }
            _ => (),
        }
    }
}
```