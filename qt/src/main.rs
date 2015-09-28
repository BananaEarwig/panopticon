extern crate panopticon;
extern crate qmlrs;
extern crate libc;
extern crate graph_algos;
extern crate uuid;
extern crate rustc_serialize;
extern crate glpk_sys as glpk;

#[macro_use]
extern crate lazy_static;

mod controller;
mod state;
mod function;
mod sugiyama;
mod route;

use controller::create_singleton;

pub fn main() {
    qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

    let mut engine = qmlrs::Engine::new();
    engine.load_local_file("qt/res/Window.qml");
    engine.exec();
}
