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
struct Headers(HashMap<String, String>);

type CompactDrain = Fuse<Mutex<CompactFormat<TermDecorator>>>;

// currently unused, but fails in the same way slog_json does
pub fn default_compact_drain() -> CompactDrain {
    let decorator = slog_term::TermDecorator::new().build();
    let format = slog_term::CompactFormat::new(decorator).build();
    std::sync::Mutex::new(format).fuse()
}

fn main() {
    let drain = Mutex::new(slog_json::Json::default(std::io::stdout())).map(slog::Fuse);
    let logger = slog::Logger::root(drain, o!());
    let mut header_map = HashMap::new();
    header_map.insert("User-Agent".into(), "curl/7.65.3".into());
    let headers = Headers(header_map);
    debug!(logger, "Hello, world!"; "headers" => headers);
}
