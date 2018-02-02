# Обновление WebViews

Первой фичей, с нашим пользовательским интерфейсом, будет динамическое обновление web предпросмотра. Начнём с создания нового метода **App::editor_changed()** для **App**, который принимает компонент **current_file** и ссылку на кнопку **save**, для отключения и включения кнопки через некоторое время, чтобы указать, были ли сделаны изменения, требующие сохранения.

```rust
/// Обновляет WebView при изменении SourceBuffer.
fn editor_changed(
    &self,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    save_button: &Button,
) {
    let preview = self.content.preview.clone();
    let save_button = save_button.clone();
    self.content.source.buff.connect_changed(move |editor| {
        if let Some(markdown) = get_buffer(&editor) {
            preview.load_html(&render(&markdown), None);
            if let Some(ref current_file) = *current_file.read().unwrap() {
                let has_same_sum = current_file.is_same_as(&markdown.as_bytes());
                save_button.set_sensitive(!has_same_sum);
            }
        }
    });
}
```

## connect_changed()

В коде выше вы могли заметить, что мы вызываем метод **connect_changed**  из исходного буфера, чтобы обновить просмотр и изменить кнопку сохранения, как только буфер будет изменён.

## Получение текста из Source Buffer

Используем функцию **get_buffer()** из модуля **misc.rs**:

```rust
if let Some(markdown) = get_buffer(&editor) {

}
```

Мы можем получить текст в буфере редактора.

## Обновление Web просмотра

```rust
preview.load_html(&render(&markdown), None);
```

Здесь мы используем функцию **render()** из модуля **preview**
для преобразования `Markdown` текста в HTML строку и незамедлительно
передаём этот HTML в наш `preview` с помощью метода **load_html()**.

## Изменение кнопки Save

Этот раздел - последний фрагмент задачи, где мы получаем read-only
блокировку для текущего файла и проверяем: генерируется ли текст в буфере с таким же хешем, как и хеш хранящийся на диске. Если хеш совпадает - кнопка сохранения будет неактивна. Если не совпадает - она будет активна.

```rust
if let Some(ref current_file) = *current_file.read().unwrap() {
    let has_same_sum = current_file.is_same_as(&markdown.as_bytes());
    save_button.set_sensitive(!has_same_sum);
}
```