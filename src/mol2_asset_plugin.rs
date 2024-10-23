use anyhow::Result;
use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, LoadContext};
use bevy::tasks::futures_lite::io::BufReader;
use bevy::tasks::futures_lite::{AsyncBufReadExt, StreamExt};
use bevy::{math::Vec3, reflect::TypePath};

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

        let mut atoms = vec![];
        let mut bonds = vec![];

        let mut lines = buffered_reader.lines();

        // let mut line = String::new();
        println!("will start processing lines");
        while let Some(line) = lines.next().await {
            let line = line?;
            // while buffered_reader.read_line(&mut line).await? > 0 {
            // println!("{}", line);

            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts[0] {
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
            if parts.len() == 1 {
                continue;
            }

            // println!("parts length: {}", parts.length());

            // parts[0] is the line number, we'll ignore that

            if parsing_atoms {
                let atom = Mol2Atom {
                    id: parts[0].parse()?,
                    name: parts[1].to_string(),
                    x: parts[2].parse()?,
                    y: parts[3].parse()?,
                    z: parts[4].parse()?,
                    type_: parts[5].to_string(),
                    bond_count: parts[6].parse()?,
                    mol_name: parts[7].to_string(),
                };
                atoms.push(atom);
            } else if parsing_bonds {
                let bond = Mol2Bond {
                    id: parts[0].parse()?,
                    atom1: parts[1].parse()?,
                    atom2: parts[2].parse()?,
                    type_: parts[3].to_string(),
                };
                bonds.push(bond);
            }
        }

        println!(
            "finished parsing mol: atoms: {}, bonds: {}",
            atoms.len(),
            bonds.len()
        );
        let mol = Mol2Molecule { atoms, bonds };
        Ok(mol)
    }
}

// TODO performance: remove clone from these
#[derive(Default, Debug, Clone, Asset, TypePath)]
pub struct Mol2Molecule {
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
