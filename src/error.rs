//! Everything related to the wikilib error handling

use glob;
use iron::error::HttpError;
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error) #[doc="An I/O error"];
        Glob(glob::GlobError) #[doc="A glob error"];
        Pattern(glob::PatternError) #[doc="A glob pattern error"];
        Http(HttpError) #[doc="A http error"];
    }
}
