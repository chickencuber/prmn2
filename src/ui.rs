use cursive::{Cursive, event::{Event, Key}};

pub fn setup() -> Cursive {
    let mut siv = Cursive::default();
    siv.set_theme(crate::theme::custom());
    let quit_or_back = |s: &mut Cursive| {
        if s.screen().len() <= 1 {
            s.quit();
        } else {
            s.pop_layer();
        }
    };

    siv.add_global_callback('q', quit_or_back);
    siv.add_global_callback(Key::Esc, quit_or_back);
    siv.add_global_callback('j', |s| {
        s.on_event(Event::Key(Key::Down));
    });

    siv.add_global_callback('k', |s| {
        s.on_event(Event::Key(Key::Up));
    });
    return siv;
}
