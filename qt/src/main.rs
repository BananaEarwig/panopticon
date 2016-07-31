/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate log;

extern crate panopticon;
extern crate qmlrs;
extern crate libc;
extern crate graph_algos;
extern crate uuid;
extern crate rustc_serialize;
extern crate cassowary;
extern crate tempdir;
extern crate byteorder;
extern crate chrono;
extern crate chrono_humanize;

#[cfg(unix)]
extern crate xdg;

#[macro_use]
extern crate lazy_static;

mod controller;
mod project;
mod function;
mod sugiyama;

use std::env;
use std::fs::File;
use std::path::{PathBuf,Path};
use std::borrow::Cow;
use std::error::Error;

#[cfg(unix)]
use xdg::BaseDirectories;

use qmlrs::{Variant};

use panopticon::result;
use panopticon::result::Result;

use controller::{
    create_singleton,
    create_request,
    Controller,
};

#[cfg(all(unix,not(target_os = "macos")))]
fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match BaseDirectories::with_prefix("panopticon") {
        Ok(dirs) => Ok(dirs.find_data_file(p).or(Some(Path::new(".").join(p)))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(all(unix,target_os = "macos"))]
fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match env::current_exe() {
        Ok(path) => Ok(path.parent().and_then(|x| x.parent()).
		map(|x| x.join("Resources").join(p))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(windows)]
fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match env::current_exe() {
        Ok(path) => Ok(path.parent().map(|x| x.join(p))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

fn main() {
    // workaround bug #165
    if cfg!(unix) {
        env::set_var("UBUNTU_MENUPROXY","");
    }

    let title_screen = find_data_file(&Path::new("qml").join("Title.qml"));
    let main_window = find_data_file(&Path::new("qml").join("Window.qml"));

    match (title_screen,main_window) {
        (Ok(Some(title)),Ok(Some(window))) => {

            {
                let mut engine = qmlrs::Engine::new();
                let mut req = create_request();

                engine.set_property("Panopticon",&req);
                engine.load_local_file(&format!("{}",title.display()));
                engine.exec();

                if let Variant::String(ref path) = req.get_property("path") {
                    if let Variant::String(ref typ) = req.get_property("type") {
                        let _ = Controller::set_request(path,typ);
                    }
                }
            }

            {
                qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

                let mut engine = qmlrs::Engine::new();
                engine.load_local_file(&format!("{}",window.display()));
                engine.exec();
            }

            return;
        },
        _ => {
            println!("Failed to open the QML files")
        },
    }
}
