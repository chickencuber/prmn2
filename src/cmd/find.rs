use cursive::{Cursive, views::Dialog};

use crate::{cmd::{Commands, selector, start}, ui::push_layer};

pub fn find(cmd: Commands, siv: Cursive) -> Cursive {
    let out = cmd.out;
    let no_last = cmd.no_last;
    let mut siv = start(cmd, siv);
    let select = selector(out, no_last, &mut siv);
    push_layer(&mut siv, Dialog::new().content(select).title("Search"));
    siv
}
