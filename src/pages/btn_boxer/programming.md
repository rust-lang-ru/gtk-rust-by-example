# Programming the UI

At this point, we can now wire everything together in the main thread. First we will set the
default value (health value) for the state of the program. That value will be used to set
the initial state of the GTK application structure. Then we shall program the hit and heal
buttons, which shall change values in the content area of main window.

## Before We Start

We are going to have some predefined strings utilized based on what action was performed, and
certain conditions of the **HealthComponents** value. To do this, we will have a **MESSAGES**
array that we will access via a **u8**-sized enum, which will be used to get indexes into the
array.

```rust
/// Predefined messages that will be used by the UI upon certain conditions.
const MESSAGES: [&str; 3] = ["Ouch! You hit me!", "...", "Thanks!"];

#[repr(u8)]
// An enum, represented as a u8, that is used as an index into the `MESSAGES` array.
enum Message { Hit, Dead, Heal }
```

For those not yet versed in Rust, the `#[repr(u8)]` attribute defines that the following item
should be represented as a **u8** value in memory. By default, enum variants start counting from
zero, so **Hit** is `0`, whereas **Heal** is `2`. If you want to make this explicit, you can also
write this as so:

```rust
#[repr(u8)]
enum Message { Hit = 0, Dead = 1, Heal = 2 }
```

## Initializing the Health Component & Application Structure

After initializing GTK, we will create our health component, which will be wrapped within an
atomic reference-counted pointer (**Arc**). If we remember from previous code, the inner value
is actually an **AtomicUsize**, which serves as our health counter. This value will be shared
among multiple closures, hence the requirement for the reference counter.

```rust
let health = Arc::new(HealthComponent::new(10));
```

Using this value, we will create our application's UI structure. Note that `&health` is
automatically referenced as an **&HealthComponent**, even though it's wrapped within an **Arc**.

```rust
let app = App::new(&health);
```

## Programming the Hit Button

From here on, all we need to is to program our widgets, and this is where we will share both
our health component, and various other UI widgets across closures. Starting with the hit button,
we simply need to program what will happen when that button is clicked. The **ButtonExt** trait
provides a **connect_clicked()** method for precisely that.

> Note that widgets in GTK typically pass themselves through their closures, so if you need to
> manipulate the calling widget, you can do so by using the assigned value passed through the
> closure. We don't require this functionality, so we shall ignore the value.
>
> ```rust
> widget.connect_action(move |widget| {});
> ```

```rust
{
    // Program the Hit button to subtract health.
    let health = health.clone();
    let message = app.content.message.clone();
    let info = app.content.health.clone();
    app.header.hit.clone().connect_clicked(move |_| {
        let new_health = health.subtract(1);
        let action = if new_health == 0 { Message::Dead } else { Message::Hit };
        message.set_label(MESSAGES[action as usize]);
        info.set_label(new_health.to_string().as_str());
    });
}
```

In the above, we create an anonymous scope so that we can self-contain our cloned references.
Each invocation of **clone()** will simply increment a reference counter, and enable these values
to be used again at a later time.

After subtracting from the health component, if the health is now `0`, we will return
**Message::Dead**, otherwise the message shall be **MessageHit**. Once we have this information,
it's just a matter of updated the labels with their new values.

## Programming the Heal Button

This works almost identically, so we can effectively copy and paste the above code, and then
modify it to meet our needs for this action.

```rust
{
    // Program the Heal button to restore health.
    let health = health.clone();
    let message = app.content.message.clone();
    let info = app.content.health.clone();
    app.header.heal.clone().connect_clicked(move |_| {
        let new_health = health.heal(5);
        message.set_label(MESSAGES[Message::Heal as usize]);
        info.set_label(new_health.to_string().as_str());
    });
}
```

## Altogether

After programming the UI, you can end the code by tacking on the following at the end:

```rust
// Make all the widgets within the UI visible.
app.window.show_all();

// Start the GTK main event loop
gtk::main();
```

And you should have your source code look as follows:

```rust
/// Predefined messages that will be used by the UI upon certain conditions.
const MESSAGES: [&str; 3] = ["Ouch! You hit me!", "...", "Thanks!"];

#[repr(u8)]
// An enum, used as a u8, that is used as an index into the `MESSAGES` array.
enum Message { Hit, Dead, Heal }

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Set the initial state of our health component. We use an `Arc` so that we can share
    // this value across multiple programmable closures.
    let health = Arc::new(HealthComponent::new(10));

    // Initialize the UI's initial state.
    let app = App::new(&health);

    {
        // Program the Hit button to subtract health.
        let health = health.clone();
        let message = app.content.message.clone();
        let info = app.content.health.clone();
        app.header.hit.clone().connect_clicked(move |_| {
            let new_health = health.subtract(1);
            let action = if new_health == 0 { Message::Dead } else { Message::Hit };
            message.set_label(MESSAGES[action as usize]);
            info.set_label(new_health.to_string().as_str());
        });
    }

    {
        // Program the Heal button to restore health.
        let health = health.clone();
        let message = app.content.message.clone();
        let info = app.content.health.clone();
        app.header.heal.clone().connect_clicked(move |_| {
            let new_health = health.heal(5);
            message.set_label(MESSAGES[Message::Heal as usize]);
            info.set_label(new_health.to_string().as_str());
        });
    }

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
```