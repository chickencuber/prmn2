
use std::path::PathBuf;

use cursive::{
    Cursive, view::{Resizable, Scrollable}, views::{Dialog, OnEventView, SelectView}
};

use crate::{
    cmd::{Commands, shared::{Conf, get_all_files, output, use_category}},
    data::{Category, Data},
    ui,
};



pub fn start(cmd : Commands, c: Option<Data>) -> Cursive {
    //no command name to remove
    let mut siv = ui::setup();
    siv.set_user_data(c.unwrap_or_else(|| Data::new().expect("failed to load config")));
    let out = cmd.out;
    let mut select = SelectView::new().on_submit(move |siv, (cat, name): &(Category, String)| {
        let select = use_category(cat, &cmd, name.clone()).expect("failed to read dir");
        siv.add_layer(Dialog::new().content(select).title(name));
    });
    for (k, v) in &siv.user_data::<Data>().unwrap().categories {
        if !PathBuf::from(&v.parent_dir).is_dir() {
            continue;
        }
        select.add_item(k.clone(), (v.clone(), k.clone()));
    }
    let event = OnEventView::new(select.scrollable()).on_event('f', move |siv| {
        let select = ui::fuzzy_picker(get_all_files(siv.user_data().unwrap(), None).expect("couldn't get the files"), move |siv, e| {
            let conf = siv.user_data::<Data>().unwrap();
            let o : Vec<String> = e.split("/").map(|s| s.to_string()).collect();
            let mut path = conf.categories[&o[0]].parent_dir.clone();
            path.push(&o[1]);
            output(Conf::Cursive(siv), out, path.to_string_lossy());
        });
        siv.add_layer(Dialog::new().content(select).title("Search"));
    });
    siv.add_layer(event.full_screen());
    siv
}
