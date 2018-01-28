## Заключение и обзор

Если вы нажмёте на кнопку `Ударить`, счётчик должен уменьшиться и сообщение должно измениться. Нажатие по кнопке `Лечить` должно  увеличить счётчик и также изменить сообщение.После запуска вашей программы с помощью `cargo run`, вы должны увидеть окно, которое выглядит так:

<img src="img/3/btn_boxer.png" />

На этом этапе, вы должны хорошо понимать как работают: **GtkBox**, **GtkButton** и **GtkLabel**. Вы можете вернуться к предыдущему разделу, чтобы ещё раз уточнить некоторые моменты.

## Практическое занятие
### Setting Inputs w/ Buttons

There isn't much that you can do with just buttons and labels. If you want a practice challenge, try creating a program that displays a simple random math problem, and asks the user to use buttons to set the value. If they get it correct, modify a label to tell the user that what they entered was correct. This is an incredibly annoying interface design, so don't do this in the real world!
Bonus: Timed Answers

Do the same as the above, but also take advantage of `gtk::timeout_add()` to decrement and update a timer label within the UI until the timer reaches zero.