
#[macro_use]
extern crate ketos;
use ketos::*;

extern crate es;
use es::modes::{ModeList, new_mode};

use std::rc::Rc;
use std::path::Path;

fn main() {
    let global = Rc::new(ModeList::new("global"));
    let major = Rc::new(ModeList::new("major"));
    let minor = Rc::new(ModeList::new("minor"));

    let interp = Interpreter::new();

    ketos_fn! {
        interp.scope() => "new-mode" =>
        fn new_mode(list: &ModeList, name: &str, desc: &str) -> ()
    }

    let res = interp.run_file(Path::new("test.ket"));
    match res {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
        }
    }

    interp.call("es-main",
                vec![Value::Foreign(global.clone()),
                     Value::Foreign(major.clone()),
                     Value::Foreign(minor.clone())]).unwrap();

    global.list();
    major.list();
    minor.list();
}
