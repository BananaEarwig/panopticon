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

extern crate panopticon;

use std::path::Path;
use panopticon::project::Project;

#[test]
fn project_pe() {
    let maybe_project = Project::pe(Path::new("tests/data/test.exe"));

    assert!(maybe_project.is_some());
}

#[test]
fn project_open() {
    let maybe_project = Project::open(Path::new("tests/data/save.panop"));

    assert!(maybe_project.ok().is_some());
}

#[test]
fn project_empty() {
    let maybe_project = Project::open(Path::new("tests/data/empty.panop"));

    assert!(maybe_project.ok().is_none());
}
