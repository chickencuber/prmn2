use cursive::{CursiveExt, views::Dialog};

use crate::cmd::{Commands, selector, start};

pub fn find(cmd: Commands) {
    let out = cmd.out;
    let mut siv = start(cmd, None);
    let select = selector(out, &mut siv);
    siv.add_layer(Dialog::new().content(select).title("Search"));
    siv.run();
}
