use crate::{ui::CarbonCount, ui_events::UiCarbonCountInputEvent};
use bevy::prelude::{EventWriter, Query};
use chemcore::daylight::read_smiles;
use gamma::graph::Graph;

pub fn process_smiles(
    carbon_count_query: &mut Query<&CarbonCount>,
    event_writer: &mut EventWriter<UiCarbonCountInputEvent>,
    str: String,
) -> Result<(), String> {
    let carbon_count = carbon_count_query.single_mut();
    println!("Current carbon count: {}", carbon_count.0);
    match parse_smiles(str) {
        Ok(carbon_count) => {
            let current = carbon_count.0;
            println!(
                "Parsed carbon count: {}, changed: {}",
                carbon_count.0,
                current != carbon_count.0
            );
            event_writer.send(UiCarbonCountInputEvent(carbon_count.0));
            Ok(())
        }

        Err(e) => Err(e),
    }
}

pub fn parse_smiles(str: String) -> Result<CarbonCount, String> {
    let molecule = read_smiles(&str, None).map_err(|e| format!("{:?}", e))?;

    // TODO validations for linear alkane as we can't draw anything else

    // TODO (low prio) review usize as u32
    Ok(CarbonCount(molecule.order() as u32))
}
