use cursive::{Cursive, views::Dialog};

use crate::{cmd::{Commands, selector, start}, ui::push_layer};

pub fn find(cmd: Commands, siv: Cursive) -> Cursive {
    let out = cmd.out;
    let mut siv = start(cmd, siv);
    let select = selector(out, &mut siv);
    push_layer(&mut siv, Dialog::new().content(select).title("Search"));
    siv
}
