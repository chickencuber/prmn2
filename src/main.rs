mod cmd;
mod data;
mod theme;
mod ui;

use cmd::Commands;
use cmdparsing::cmd;
use cursive::CursiveExt;
use std::io::{IsTerminal, stdout};

cmd! {
    help: cmd::HELP;
    :|args: Vec<String>| {
        let mut cmd = Commands::from(args.clone());
        if !stdout().is_terminal() {
            cmd.out = true;
        }
        cmd::start(cmd, None).run();
    };
    cmd::last=>"l"|"last",
    cmd::find=>"f"|"find",
}
