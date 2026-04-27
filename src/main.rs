mod cmd;
mod data;
mod theme;
mod ui;

use cmd::Commands;
use cmdparsing::cmd;
use cursive::CursiveExt;
use std::io::{IsTerminal, stdout};

use crate::cmd::shared::print_output;

fn fun<F: Fn(Commands)+'static>(f: F)-> impl Fn(Commands) {
    return move |mut cmd| {
        if !stdout().is_terminal() {
            cmd.out = true;
        }
        f(cmd); 
        print_output();
    }
}

cmd! {
    help: cmd::HELP;
    .Commands;
    :fun(|cmd| {
        cmd::start(cmd, None).run();
    });
    fun(cmd::last)=>"l"|"last",
    fun(cmd::find)=>"f"|"find",
}

