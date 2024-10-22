use anyhow::Result;
use bevy::math::Vec3;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_mol2() -> Result<Mol2Molecule> {
    let file = File::open("assets/benzene.mol2")?;
    let reader = BufReader::new(file);

    let mut parsing_atoms = false;
    let mut parsing_bonds = false;

    let mut atoms = vec![];
    let mut bonds = vec![];

    for res in reader.lines() {
        let line = res?;
        println!("{}", line);

        // seems there's always a line number so commented for now
        // if line.trim().is_empty() {
        //     continue;
        // }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 1 {
            continue;
        }
        match parts[1] {
            "@<TRIPOS>ATOM" => {
                parsing_atoms = true;
                parsing_bonds = false;
            }
            "@<TRIPOS>BOND" => {
                parsing_bonds = true;
                parsing_atoms = false;
            }
            "@<TRIPOS>SUBSTRUCTURE" => {
                // we don't use this yet so just finish
                break;
            }
            _ => {}
        }
        // the above headers
        if parts.len() == 2 {
            continue;
        }

        // println!("parts length: {}", parts.length());

        // parts[0] is the line number, we'll ignore that

        if parsing_atoms {
            let atom = Mol2Atom {
                id: parts[1].parse()?,
                name: parts[2].to_string(),
                x: parts[3].parse()?,
                y: parts[4].parse()?,
                z: parts[5].parse()?,
                type_: parts[6].to_string(),
                bond_count: parts[7].parse()?,
                mol_name: parts[8].to_string(),
            };
            atoms.push(atom);
        } else if parsing_bonds {
            let bond = Mol2Bond {
                id: parts[1].parse()?,
                atom1: parts[2].parse()?,
                atom2: parts[3].parse()?,
                type_: parts[4].to_string(),
            };
            bonds.push(bond);
        }
    }

    let mol = Mol2Molecule { atoms, bonds };
    // println!("finished parsing mol: {:?}", mol);
    Ok(mol)
}

// TODO performance: remove clone from these
#[derive(Default, Debug, Clone)] // needed for bevy, since it's sent in an event..
pub struct Mol2Molecule {
    pub atoms: Vec<Mol2Atom>,
    pub bonds: Vec<Mol2Bond>,
}

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
}

impl Mol2Atom {
    pub fn loc_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct Mol2Bond {
    pub id: u32,
    pub atom1: usize,
    pub atom2: usize,
    pub type_: String,
}
