use cursive::{
    Cursive, View,
    event::{Event, Key},
    utils::markup::StyledString,
    view::{Nameable, Resizable, Scrollable},
    views::{EditView, LinearLayout, SelectView},
};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

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

    LinearLayout::vertical().child(search).child(select).full_screen()
}
