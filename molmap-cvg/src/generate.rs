// SPDX-FileCopyrightText: 2025 Matthew J. Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use molmap::na;
use molmap_svg;

fn bondable_to_id_string(bondable: molmap::Bondable) -> String {
    match bondable {
        molmap::Bondable::Bond(id) => format!("b{}", id),
        molmap::Bondable::Atom(id) => format!("a{}", id),
        molmap::Bondable::Pseudoatom(id) => format!("p{}", id),
        molmap::Bondable::Node(id) => format!("n{}", id),
    }
}

// TODO Get a realistic size for the canvas
fn canvas_size(molmap: &molmap::MolMap) -> na::Vector2<f32> {
    na::Vector2::new(400.0, 400.0)
}

fn bond_to_cvg(molmap: &molmap::MolMap, id: molmap::BondId) -> Result<svg::node::element::Path, molmap::IdError> {
    let path = molmap_svg::bond_to_svg(molmap, id)?;
    // Add CVG namespace attributes
    let path = 
        path
        .set("cvg:role", "bond")
        .set("cvg:order", molmap.bond_order(id)?)
        .set("cvg:connects", molmap.bond_connects(id)?.iter().map(|&x| bondable_to_id_string(x)).collect::<Vec<String>>());
Ok(path)
}

fn atom_to_cvg(molmap: &molmap::MolMap, id: molmap::AtomId) -> Result<svg::node::element::Text, molmap::IdError> {
    let text = molmap_svg::atom_to_svg(molmap, id)?;
    // Add CVG namespace attributes
    let text = text.set("cvg:role", "atom");
    Ok(text)
}

pub fn to_cvg(molmap: &molmap::MolMap) -> svg::Document {
    let size = canvas_size(molmap);
    let mut document = svg::Document::new()
        .set("width", size.x)
        .set("height", size.y)
        .set("xmlns:cvg", "http://www.github.com/matterhorn103/cvg");
    for id in molmap.atoms() {
        document = document.add(atom_to_cvg(molmap, id).unwrap());
    }
    for id in molmap.bonds() {
        document = document.add(bond_to_cvg(molmap, id).unwrap());
    }
    document
}
