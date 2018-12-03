// +----------------------------------------------------------------------
// | Fighting for great , building a better edu framework
// |
// | Share with each other!
// |
// | Email:823923263@qq.com
// +----------------------------------------------------------------------
// | Author: ShirDon <http://www.shirdon.com>
// +----------------------------------------------------------------------
//! ## Strategy
//! All markdown files are read at startup by globbing for README.md in
//! the posts/ directory. These files are converted to HTML and stored
//! in a map. It's also linked up with a `data.json` and all links are
//! converted from relative links to something with a /static prefix.
//!
//! Summaries are created by somewhat naively trying to find the first
//! paragraph that is not a raw heading or an image.
//!
//! ## Serving
//! The main blog engine would be in the binary, and that's the only
//! thing that is depending on `rocket` and its ecosystem.
//!

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;


#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate glob;
extern crate hoedown;
extern crate regex;

//pub use errors::BlogResult;
pub use data::{Post, MetaData, DataBase, load_posts};

mod data;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}
