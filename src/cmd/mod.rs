mod start;
mod shared;
use cmdparsing::define;
pub use start::*;

mod find;
pub use find::*;

mod last;
pub use last::*;

define! {
    Commands;
    help: HELP;
          flags {
              out: bool = "o" | "out",
          };
          args {};
}

pub const HELP: &str = r#"usage: prmn [subcommand] [flags]
    ==========FLAGS==========
    -h(--help): displays this message
    -o(--out): outputs the project dir into stdout instead of opening the editor
    =======SUB COMMANDS======
    [none]: shows the menu
    help: shows this message
    l | last: opens the last project opened
    f | find: opens the search menu"#;
