use cursive::{
    Cursive, View,
    event::{Event, Key},
    utils::markup::StyledString,
    view::{Nameable, Resizable, Scrollable},
    views::{
        Dialog, DummyView, EditView, LayerPosition, LinearLayout, Menubar, SelectView, TextView,
    },
};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

use crate::{data::Data, wrapper::{Mode, ModeView}};

pub fn setup(conf: Data) -> Cursive {
    let mut siv = Cursive::new();
    siv.set_theme(crate::theme::custom());
    let mut i = 1;

    if conf.show_menubar {
        add_menubar(&mut siv);
    }
    if conf.show_hint {
        add_help_hint(&mut siv, &mut i);
    }
    siv.set_user_data(conf);

    siv.add_global_callback('j', |siv| {
        siv.on_event(Event::Key(Key::Down));
    });
    siv.add_global_callback('k', |siv| {
        siv.on_event(Event::Key(Key::Up));
    });

    let quit_or_back = move |siv: &mut Cursive| {
        if siv.screen().len() <= i {
            siv.quit();
        } else {
            pop_layer(siv);
        }
    };
    siv.add_global_callback('q', quit_or_back);
    siv.add_global_callback(Key::Esc, quit_or_back);
    siv.add_global_callback('?', |siv| {
        help_dialog(siv);
    });
    return siv;
}

const HELP_MENU_TEXT: &str = r#"f: enters find mode
Escape/q: goes back/quits
h/↑: Up
j/↓: Down
===When in Category Screen===
d: Deletes a Project
a: Adds a Project
r: Renames a Project"#;

fn help_dialog(siv: &mut Cursive) {
    if siv.find_name::<Dialog>("help-dialog").is_some() {
        return;
    }
    let dialog = Dialog::new()
        .content(TextView::new(HELP_MENU_TEXT))
        .title("HELP")
        .button("OK", |siv| {
            pop_layer(siv);
        })
        .with_name("help-dialog");
    push_layer(siv, dialog);
}

fn add_menubar(siv: &mut Cursive) {
    siv.set_autohide_menu(false);
    populate_menubar(siv.menubar(), None);
}

pub fn populate_menubar(menubar: &mut Menubar, mode: Option<Mode>) {
    menubar.clear();
    menubar.add_leaf("<Help>", |siv| {
        help_dialog(siv);
    });
    menubar.add_leaf("<Find>", |siv| {
        siv.on_event(Event::Char('f'));
    });
    if let Some(mode) = mode {
        menubar.add_delimiter();
        match mode {
            Mode::Category => {
                menubar.add_leaf("<Add>", |siv| {
                    siv.on_event(Event::Char('a'));
                });
                menubar.add_leaf("<Delete Selected>", |siv| {
                    siv.on_event(Event::Char('d'));
                });
                menubar.add_leaf("<Rename Selected>", |siv| {
                    siv.on_event(Event::Char('r'));
                });
            }
        }
    }
}

fn change_layer(siv: &mut Cursive) {
    let s = siv.screen().get(LayerPosition::FromFront(0)).unwrap();
    let mode = s.downcast_ref::<ModeView>().map(|v| v.mode.clone());
    populate_menubar(siv.menubar(), mode);
}

pub fn pop_layer(siv: &mut Cursive) {
    siv.pop_layer();
    change_layer(siv);
}

pub fn push_layer<T: View>(siv: &mut Cursive, view: T) {
    siv.add_layer(view);
    change_layer(siv);
}

fn add_help_hint(siv: &mut Cursive, i: &mut usize) {
    let hint = LinearLayout::vertical()
        .child(DummyView::new().full_height())
        .child(
            LinearLayout::horizontal()
                .child(DummyView::new().full_width())
                .child(TextView::new("help: ? ")),
        );

    siv.add_fullscreen_layer(hint);
    *i += 1;
}

pub fn fuzzy_picker<T, F>(items: Vec<T>, on_select: F) -> impl View
where
    T: Clone + 'static + Send + Sync + Into<StyledString> + ToString,
    F: Fn(&mut Cursive, &T) + 'static + Send + Sync + Clone,
{
    let matcher = SkimMatcherV2::default();

    let items_for_filter = items.clone();
    let items_for_render = items.clone();

    let mut select = SelectView::<T>::new().on_submit(on_select.clone());

    for item in &items_for_render {
        select.add_item(item.clone(), item.clone());
    }
    select.sort_by_label();

    let select = select.with_name("list").scrollable().show_scrollbars(false);

    let search = EditView::new()
        .on_edit(move |siv, text, _| {
            let mut scored: Vec<(i64, T)> = items_for_filter
                .iter()
                .filter_map(|item| {
                    matcher
                        .fuzzy_match(&item.to_string(), text)
                        .map(|score| (score, item.clone()))
                })
                .collect();

            scored.sort_by(|a, b| b.0.cmp(&a.0));

            siv.call_on_name("list", |view: &mut SelectView<T>| {
                view.clear();

                for (_, item) in scored {
                    view.add_item(&item.to_string(), item);
                }
            });
        })
        .on_submit(move |siv, _| {
            let selected = siv
                .call_on_name("list", |view: &mut SelectView<T>| {
                    view.selection().map(|s| s.clone())
                })
                .flatten();

            if let Some(s) = selected {
                let sink = siv.cb_sink().clone();

                let fun = on_select.clone();
                sink.send(Box::new(move |siv| {
                    fun(siv, &s);
                }))
                .unwrap();
            }
        });

    LinearLayout::vertical()
        .child(search)
        .child(select)
        .full_screen()
}
