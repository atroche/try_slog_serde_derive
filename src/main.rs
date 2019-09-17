extern crate serde;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_derive;
extern crate erased_serde;
use slog_json;

use std::collections::HashMap;

use serde::Serialize;

use slog::{Drain, Fuse};
use slog_term::{CompactFormat, TermDecorator};
use std::sync::Mutex;

#[derive(Clone, SerdeValue, Serialize)]
struct MyString(String);

fn main() {
    let drain = Mutex::new(slog_json::Json::default(std::io::stdout())).map(slog::Fuse);
    let logger = slog::Logger::root(drain, o!());
    let my_string = MyString("testing".into());
    debug!(logger, "Hello, world!"; "my_string" => my_string);
}


type CompactDrain = Fuse<Mutex<CompactFormat<TermDecorator>>>;
// currently unused, but fails in the same way slog_json does
pub fn default_compact_drain() -> CompactDrain {
    let decorator = slog_term::TermDecorator::new().build();
    let format = slog_term::CompactFormat::new(decorator).build();
    std::sync::Mutex::new(format).fuse()
}
