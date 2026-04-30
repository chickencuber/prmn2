pub mod shared;
pub mod start;
use cmdparsing::define;
pub use start::*;

pub mod find;
pub use find::*;

pub mod last;
pub use last::*;

define! {
    Commands;
    help: HELP;
          flags {
              out: bool = "o" | "out",
          };
          args {};
}

pub const HELP: &str = r#"usage: prmn [subcommand]
    ==========FLAGS==========
    -h | --help | -?: displays this message
    -o | --out: outputs the project dir into stdout instead of opening the editor
    =======SUB COMMANDS======
    [none]: shows the menu
    help | ?: shows this message
    l | last: opens the last project opened
    f | find: opens the search menu"#;
