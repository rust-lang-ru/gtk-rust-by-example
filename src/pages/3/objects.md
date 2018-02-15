# Упаковки, кнопки и метки

Цель этого раздела заключается в том, чтобы дать объяснение объектам, которые будут использованы до того, как мы применим их на практике в следующих разделах.

## GtkBox

**GtkBox** является фактически эквивалентом UI вектора в Rust и должен быть объявлен с помощью **Orientation**, который определяет, что должны ли элементы быть выровнены слева направо или сверху вниз. Для тех, кто имеет опыт разработки с современным дизайном HTML5/CSS3, **GtkBox** эквивалентен гибким упаковкам -- они могут расширяться на всё пространство, а виджеты, содержащиеся внутри, тоже могут расширяться в соответствии с правилами, применяемыми при дочерней упаковке.

## Создание упаковки

В следующем примере, мы создадим горизонтальную и вертикальную упаковку с нулевым отступом между дочерними элементами, содержащимися в упаковке. После создания упаковки, вы можете задать виджеты, используя метод `pack_*`.

```rust
let padding_between_children = 0;
let horizontal_box = Box::new(Orientation::Horizontal, padding_between_children);
let vertical_box = Box::new(Orientation::Vertical, padding_between_children);
```

## Упаковка упаковки

Вы могли заметить, что метод `pack_*` принимает большое количество параметров. Первым параметром должна быть ссылка на виджет, которую вы добавляете в контейнер. Вторым и третьим параметрами объявляют параметры заполнения соответственно. Последним параметром объявляют - как много единиц пространства должно быть между дочерними элементами в упаковке.

> To further elaborate on the expand and fill parameters, expand defines whether the
> given widget should attempt to use all of the extra space that it can. Each widget that has
> the expand parameter set will equally share that extra space. Meanwhile, fill defines whether
> the extra spaced should actually have that widget fill to cover that extra space, or should
> merely use that extra space as padding.

```rust
health_info.pack_start(&health_label, false, false, 5);
health_info.pack_start(&health, true, true, 5);
```


## GtkLabel

**GtkLabel** - это простой виджет, который состоит исключительно из текста. Название говорит само за себя. Всё, что вам нужно запомнить - это как создать метку (label) и изменить её.

```rust
let information_label = Label::new("Specific Information: ");
let value = Label::new("Linux");
value.set_label("Redox");

let horizontal_box = Box::new(Orientation::Horizontal, 5);
horizontal_box.pack_start(&information_label, false, false, 0);
horizontal_box.pack_start(&value, true, false, 0);
```

## GtkButton

### Создание кнопок

**GtkButton** - это простая кнопка, содержащая текстовую метку (label) и/или изображения для представления действия, которое должно быть выполнено при нажатии кнопки.

```rust
let text_button = Button::new_with_label("Ok");
let image_button = Button::new_from_icon_name("имя-иконки", 32);
```

### Дизайн кнопок

Виджеты в GTK можно оформить так, чтобы они отличались от других виджетов в пользовательском интерфейсе (UI). В частности, кнопки поддерживают два класса стилей: destructive-action и suggested-action. Если в вашем UI есть особенная кнопка, которая должна отличаться, вы можете установить её так:

```rust
// Добавьте соответствующие классы стилей к этим кнопкам.
delete_button.get_style_context().map(|c| c.add_class("destructive-action"));
new_button.get_style_context().map(|c| c.add_class("suggested-action"));
```

Каждый **GtkWidget** предоставляет метод **get_style_context()**, который возвращает
**Option<StyleContext>**, тем самым предоставляя метод **add_class()**, который используется чтобы установить класс стиля. Понимаете это? Хорошо. Наиболее важные классы кнопок, которые нужно знать - это `destructive-action` и `suggested-action`. Как правило, destructive action окрашивает кнопку в красный цвет, между тем, suggested action использует синий цвет. Актуальный цвет будет зависеть от того, какая тема GTK используется вами.

