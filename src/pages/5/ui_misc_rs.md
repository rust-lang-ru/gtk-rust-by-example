# Модуль ui/misc.rs
Мы реализуем несколько вспомогательных методов, которые при необходимости
будут использоваться в проекте. Это две функции: одна для извлечения текста из
**GtkSourceBuffer**, другая для установки названия у **GtkHeaderBar** с
заданным **Path**.
```rust
use gtk::*;
use sourceview::*;
use std::path::Path;

/// Присвоить заголовку в заголовочной панели ссылку на строковое
/// представление пути к файлу. 
pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(filename);
    }
}

/// Получить все внутреннее содержимое данного текстового буфера в виде
/// строки.
pub fn get_buffer(buffer: &Buffer) -> Option<String> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}
```