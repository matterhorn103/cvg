// SPDX-FileCopyrightText: 2025 Matthew J. Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use molmap;
use crate::generate;
pub use svg;
pub use svg::write;

pub fn open<T>(path: T) -> molmap::MolMap
where
    T: AsRef<std::path::Path>,
{
    let mut content = String::new();
    let canvas = molmap::MolMap::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            svg::parser::Event::Tag(svg::node::element::tag::Path, _, attributes) => {
                if *attributes.get("cvg::role").unwrap() == "bond" {
                    //canvas.canvas.add_bond(order, connect);
                    let data = attributes.get("d").unwrap();
                    let data = svg::node::element::path::Data::parse(data).unwrap();
                    for command in data.iter() {
                        match command {
                            svg::node::element::path::Command::Move(..) => { /* … */ }
                            svg::node::element::path::Command::Line(..) => { /* … */ }
                            _ => {}
                        }
                    }
                }
            }
            svg::parser::Event::Tag(svg::node::element::tag::Text, _, attributes) => {
                
            }
            _ => {}
        }
    }
    canvas
}

pub fn save<T>(path: T, canvas: &molmap::MolMap) -> Result<(), std::io::Error>
where
    T: AsRef<std::path::Path>,
{
    let doc = generate::to_cvg(canvas);
    svg::save(path, &doc)
}

// <path cvg:role="bond" id="2" d="M10 110 L70 150" stroke="black" stroke-width="10" fill="none" />
// <text cvg:role="atom" id="3" x="211.94539" y="203.75427">A</text>
