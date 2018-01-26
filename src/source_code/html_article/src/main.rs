extern crate gtk;
#[macro_use]
extern crate horrorshow;

use gtk::*;
use std::process;

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    // Initialize the UI's initial state
    let app = App::new();

    {
        // Program the post button to take the inputs in the left pane, and update HTML code
        // within the right pane accordingly. Prepared to increment reference counters...
        let title = app.content.title.clone();
        let tags = app.content.tags.clone();
        let content = app.content.content.clone();
        let right_pane = app.content.right_pane.clone();
        app.header.post.connect_clicked(move |_| {
            let inputs = (title.get_text(), tags.get_text(), get_buffer(&content));
            if let (Some(title), Some(tags), Some(content)) = inputs {
                right_pane.set_text(&generate_html(&title, &tags, &content));
            }
        });
    }

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

/// Obtain the entire text buffer's contents as a string.
fn get_buffer(buffer: &TextBuffer) -> Option<String> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}

/// Generates the minified HTML that will be displayed in the right pane
fn generate_html(title: &str, tags: &str, content: &str) -> String {
    format!{
        "{}",
        html!{
            article {
                header {
                    h1 { : &title }
                    div(class="tags") {
                        @ for tag in tags.split(':') {
                            div(class="tag") { : tag }
                        }
                    }
                }
                @ for line in content.lines().filter(|x| !x.is_empty()) {
                    p { : line }
                }
            }
        }
    }
}

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub post:      Button,
}

pub struct Content {
    pub container:  Paned,
    pub title:      Entry,
    pub tags:       Entry,
    pub content:    TextBuffer,
    pub right_pane: TextBuffer,
}

impl App {
    fn new() -> App {
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();
        // Create the main content.
        let content = Content::new();

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("HTML Articler");
        // Set the window manager class.
        window.set_wmclass("html-articler", "HTML Articler");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");
        // Set the default size of the window.
        window.set_default_size(800, 600);
        // Add the content to the window.
        window.add(&content.container);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        App { window, header, content }
    }
}

impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("HTML Articler");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Create a button that will post the HTML article.
        let post = Button::new_with_label("Post");
        post.get_style_context().map(|x| x.add_class("suggested-action"));

        container.pack_end(&post);

        // Returns the header and all of it's state
        Header { container, post }
    }
}

impl Content {
    fn new() -> Content {
        // The main container will hold a left and right pane. The left pane is for user input,
        // whereas the right pane is for the generated output.
        let container = Paned::new(Orientation::Horizontal);
        let left_pane = Box::new(Orientation::Vertical, 5);
        let right_pane = TextBuffer::new(None);
        let right_pane_view = TextView::new_with_buffer(&right_pane);

        // The left pane will consist of a title entry, tags entry, and content text view.
        let title = Entry::new();
        let tags = Entry::new();
        let content = TextBuffer::new(None);
        let content_view = TextView::new_with_buffer(&content);

        // The label that we will display above the content box to describe it.
        let content_label = Label::new("Content");
        content_label.set_halign(Align::Center);

        // Set placeholders within the entries to hint the user of the contents to enter.
        title.set_placeholder_text("Insert Title");
        tags.set_placeholder_text("Insert Colon-Delimited Tags");

        // Additionally set tooltips on the entries. Note that you may use either text or markup.
        title.set_tooltip_text("Insert the title of article here");
        tags.set_tooltip_markup("<b>tag_one</b>:<b>tag two</b>:<b> tag three</b>");

        // The right pane should disallow editing; and both editors should wrap by word.
        right_pane_view.set_editable(false);
        right_pane_view.set_wrap_mode(WrapMode::Word);
        content_view.set_wrap_mode(WrapMode::Word);

        // Wrap the text views within scrolled windows, so that they can scroll.
        let content_scroller = ScrolledWindow::new(None, None);
        let right_pane_scrolled = ScrolledWindow::new(None, None);
        content_scroller.add(&content_view);
        right_pane_scrolled.add(&right_pane_view);

        // Paddin' Widgets
        left_pane.set_border_width(5);
        right_pane_view.set_left_margin(5);
        right_pane_view.set_right_margin(5);
        right_pane_view.set_top_margin(5);
        right_pane_view.set_bottom_margin(5);
        content_view.set_left_margin(5);
        content_view.set_right_margin(5);
        content_view.set_top_margin(5);
        content_view.set_bottom_margin(5);

        // First add everything to the left pane box.
        left_pane.pack_start(&title, false, true, 0);
        left_pane.pack_start(&tags, false, true, 0);
        left_pane.pack_start(&content_label, false, false, 0);
        left_pane.pack_start(&content_scroller, true, true, 0);

        // Then add the left and right panes into the container
        container.pack1(&left_pane, true, true);
        container.pack2(&right_pane_scrolled, true, true);

        Content { container, title, tags, content, right_pane }
    }
}