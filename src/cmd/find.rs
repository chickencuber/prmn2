use std::io::{IsTerminal, stdout};

use cursive::{CursiveExt, event::Event};

use crate::cmd::{Commands, start};


pub fn find(mut args: Vec<String>) {
    args.pop(); //remove the command name
    let mut cmd = Commands::from(args.clone());
    if !stdout().is_terminal() {
        cmd.out = true;
    }
    let mut siv = start(cmd, None);

    siv.cb_sink().send(Box::new(|siv| {
        siv.on_event(Event::Char('f'));
    })).unwrap();
    siv.run();
}
