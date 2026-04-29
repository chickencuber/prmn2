mod cmd;
mod data;
mod theme;
mod traits;
mod ui;
mod wrapper;

use cmd::Commands;
use cmdparsing::cmd;
use cursive::{Cursive, CursiveExt};
use std::io::{IsTerminal, stdout};

use crate::{cmd::shared::print_output, data::Data, traits::GetCursive, ui::setup};

fn fun<F: Fn(Commands, Cursive) -> R, R: GetCursive>(f: F) -> impl Fn(Commands) {
    return move |mut cmd| {
        if !stdout().is_terminal() {
            cmd.out = true;
        }
        let siv = setup(Data::new().expect("failed to load config"));
        if let Some(mut siv) = f(cmd, siv).get_cursive() {
            siv.run();
        }
        print_output();
    };
}

cmd! {
    help: cmd::HELP;
    .Commands;
    :fun(|cmd, siv| {
        return cmd::start(cmd, siv);
    });
    fun(cmd::last)=>"l"|"last",
    fun(cmd::find)=>"f"|"find",
}
