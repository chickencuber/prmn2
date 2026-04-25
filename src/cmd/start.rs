use cursive::{CursiveExt, event::Key, view::Resizable, views::SelectView };

use crate::{cmd::Commands, data::Data, ui};

pub fn start(args: Vec<String>) {
    //no command name to remove
    let cmd = Commands::from(args);
    let conf = Data::new().expect("failed to load config");
    let mut select = SelectView::<String>::new();
    for cat in conf.categories.keys() {
        select.add_item(cat.clone(), cat.clone());
    }
    let mut siv = ui::setup();

    siv.add_layer(select.full_screen());
    siv.run();
}
