#![allow(non_camel_case_types, unused_imports, dead_code)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
// @Import modules
pub mod interface;
pub mod utils;
pub mod xml;

use utils::*;
use xml::*;

use treexml::Document;

fn main() {
    let data = file_reader("data/community.xml");
    let doc = Document::parse(data.as_bytes()).unwrap();
    let root = doc.root.unwrap();

    let mut my_store: Store = Store::default();

    for app in root.children.iter() {
        let new_app: Application = Application::new(app.clone());
        my_store.applications.push(new_app);
    }

    file_writer(my_store, "community.json", true).unwrap();
}
