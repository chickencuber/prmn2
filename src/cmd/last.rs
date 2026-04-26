use std::io::{IsTerminal, stdout};

use cursive::CursiveExt;

use crate::{cmd::{Commands, shared::{Conf, output}, start}, data::Data};

pub fn last(mut args: Vec<String>) {
    args.pop(); //remove the command name
    let mut conf = Data::new().expect("failed to load config");
    let c = conf.last.clone();
    let mut cmd = Commands::from(args.clone());
    if !stdout().is_terminal() {
        cmd.out = true;
    }
    if let Some(last) = c {
        if last.is_dir() {
            output(Conf::Data(&mut conf), cmd.out, last.to_string_lossy());
            return;
        }
    }
    start(cmd, Some(conf)).run();
}
