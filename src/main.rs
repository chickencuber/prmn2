mod theme;
mod ui;
mod cmd;
mod data;

use cmdparsing::cmd;

cmd! {
    help: cmd::HELP;
    :cmd::start;
    cmd::last=>"l"|"last",
    cmd::find=>"f"|"find",
}

