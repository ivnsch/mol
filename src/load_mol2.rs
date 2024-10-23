use anyhow::{anyhow, Result};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    prelude::Res,
};

use crate::mol2_asset_plugin::Mol2Molecule;

pub fn load_mol2(
    asset_server: &AssetServer,
    assets: &Res<Assets<Mol2Molecule>>,
) -> Result<Mol2Molecule> {
    let path = "embedded://mol/asset/benzene.mol2";
    // let path = "embedded://mol/asset/117_ideal.mol2";
    println!("will load asset");
    let handle: Handle<Mol2Molecule> = asset_server.load(path);

    if let Some(mol) = &handle {
        
    }
    match assets.get(&handle) {
        Some(mol) => Ok(mol.clone()),
        None => Err(anyhow!("Couldn't get asset with path: {}", path)),
    }
}
