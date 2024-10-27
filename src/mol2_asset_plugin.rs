use anyhow::{anyhow, Result};
use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, LoadContext};
use bevy::tasks::futures_lite::io::BufReader;
use bevy::tasks::futures_lite::{AsyncBufReadExt, StreamExt};
use bevy::{math::Vec3, reflect::TypePath};

use crate::bounding_box::BoundingBox;
use crate::element::Element;

pub struct Mol2AssetPlugin;

impl Plugin for Mol2AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Mol2Molecule>()
            .register_asset_loader(Mol2AssetLoader);
    }
}

pub struct Mol2AssetLoader;

impl AssetLoader for Mol2AssetLoader {
    type Asset = Mol2Molecule;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Mol2Molecule, Self::Error> {
        let buffered_reader = BufReader::new(reader);

        let mut parsing_atoms = false;
        let mut parsing_bonds = false;
        let mut parsing_mol = false;

        let mut atoms = vec![];
        let mut bonds = vec![];

        let mut lines = buffered_reader.lines();

        let mut mol_name = None;

        while let Some(line) = lines.next().await {
            let line = line?;

            match parse_mol2_line(&line) {
                ProcessMol2LineResult::Empty => continue,
                ProcessMol2LineResult::Header(header) => match header {
                    Header::Atom => {
                        parsing_atoms = true;
                        parsing_bonds = false;
                        parsing_mol = false;
                    }
                    Header::Bond => {
                        parsing_bonds = true;
                        parsing_atoms = false;
                        parsing_mol = false;
                    }
                    Header::Mol => {
                        parsing_bonds = false;
                    parsing_atoms = false;
                        parsing_mol = true;
                    }
                    // we don't use this yet, ignore
                    // entries belonging to section will also be ignored
                    Header::Other => {
                        parsing_bonds = false;
                        parsing_atoms = false;
                        parsing_mol = false;
                    }
                },
                // assumption: &parts has correct length for respective handlers
                ProcessMol2LineResult::Entity { parts } => {
                    if parsing_atoms {
                        let atom = parse_atom_line(&parts)?;
                        atoms.push(atom);
                    } else if parsing_bonds {
                        let bond = parse_bond_line(&parts)?;
                        bonds.push(bond);
                    } else if parsing_mol {
                        // we just care about the name for now: finish parsing this section
                        parsing_mol = false;
                        let parsed_mol_name = parse_mol_name_line(&parts)?;
                        mol_name = Some(parsed_mol_name);
                    }
                }
            }
        }

        // mol name seems mandatory, so err if not found
        let mol_name = mol_name.ok_or_else(|| anyhow!("File has no molecule name."))?;

        println!(
            "finished parsing mol2 file: atoms: {}, bonds: {}",
            atoms.len(),
            bonds.len()
        );
        let mol = Mol2Molecule {
            name: mol_name,
            atoms,
            bonds,
        };
        Ok(mol)
    }
}

enum ProcessMol2LineResult<'a> {
    Empty,
    Header(Header),
    Entity { parts: Vec<&'a str> },
}

enum Header {
    Atom,
    Bond,
    Mol,
    Other, // for now just ignoring these
}

fn parse_mol2_line(line: &str) -> ProcessMol2LineResult {
    // println!("{}", line);

    if line.trim().is_empty() {
        return ProcessMol2LineResult::Empty;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    match parts[0] {
        "@<TRIPOS>ATOM" => {
            return ProcessMol2LineResult::Header(Header::Atom);
        }
        "@<TRIPOS>BOND" => {
            return ProcessMol2LineResult::Header(Header::Bond);
        }
        "@<TRIPOS>MOLECULE" => {
            return ProcessMol2LineResult::Header(Header::Mol);
        }
        "@<TRIPOS>SUBSTRUCTURE" => {
            return ProcessMol2LineResult::Header(Header::Other);
        }
        _ => {}
    }

    ProcessMol2LineResult::Entity { parts }
}

fn parse_atom_line(parts: &[&str]) -> Result<Mol2Atom> {
    let type_ = parts[5];
    Ok(Mol2Atom {
        id: parts[0].parse()?,
        name: parts[1].to_string(),
        element: parse_element_from_type(type_)?,
        x: parts[2].parse()?,
        y: parts[3].parse()?,
        z: parts[4].parse()?,
        type_: type_.to_string(),
        bond_count: parts[6].parse()?,
        mol_name: parts[7].to_string(),
    })
}

fn parse_element_from_type(type_: &str) -> Result<Element> {
    let parts: Vec<&str> = type_.split(".").collect();
    if parts.is_empty() {
        return Err(anyhow!("Invalid element type entry: {}", type_));
    }
    parse_element(parts[0])
}

fn parse_element(element: &str) -> Result<Element> {
    match element {
        "H" => Ok(Element::H),
        "C" => Ok(Element::C),
        "N" => Ok(Element::N),
        "O" => Ok(Element::O),
        "F" => Ok(Element::F),
        "P" => Ok(Element::P),
        "S" => Ok(Element::S),
        "Ca" => Ok(Element::Ca),
        _ => Err(anyhow!("Not handled element str: {}", element)),
    }
}

fn parse_bond_line(parts: &[&str]) -> Result<Mol2Bond> {
    Ok(Mol2Bond {
        id: parts[0].parse()?,
        atom1: parts[1].parse()?,
        atom2: parts[2].parse()?,
        type_: parts[3].to_string(),
    })
}

fn parse_mol_name_line(parts: &[&str]) -> Result<String> {
    Ok(parts[0].to_string())
}

// TODO performance: remove clone from these
#[derive(Default, Debug, Clone, Asset, TypePath)]
pub struct Mol2Molecule {
    pub name: String,
    pub atoms: Vec<Mol2Atom>,
    pub bonds: Vec<Mol2Bond>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Mol2Atom {
    pub id: i32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub type_: String,
    pub bond_count: i32,
    pub mol_name: String,
    pub element: Element,
}

impl Mol2Atom {
    pub fn loc_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Mol2Bond {
    pub id: u32,
    pub atom1: usize,
    pub atom2: usize,
    pub type_: String,
}

pub fn bounding_box_for_mol(mol2_mol: &Mol2Molecule) -> BoundingBox {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;

    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for m in &mol2_mol.atoms {
        let x = m.x;
        let y = m.y;
        let z = m.z;

        if x < min_x {
            min_x = x;
        };
        if x > max_x {
            max_x = x;
        };
        if y < min_y {
            min_y = y;
        };
        if y > max_y {
            max_y = y;
        };
        if z < min_z {
            min_z = z;
        };
        if z > max_z {
            max_z = z;
        };
    }
    BoundingBox {
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}
