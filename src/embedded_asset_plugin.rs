use bevy::{
    app::{App, Plugin},
    asset::embedded_asset,
};

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        // this is not compiling, so using the complete paths (slight repetition)
        //
        // We get to choose some prefix relative to the workspace root which
        // will be ignored in "embedded://" asset paths.
        // let omit_prefix = "asset";
        // Path to asset must be relative to this file, because that's how
        // include_bytes! works.
        // embedded_asset!(app, omit_prefix, "fonts/FiraMono-Medium.ttf");

        embedded_asset!(app, "asset/fonts/FiraMono-Medium.ttf");
        embedded_asset!(app, "asset/117_ideal.mol2");
        embedded_asset!(app, "asset/benzene.mol2");
    }
}
