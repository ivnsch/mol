use crate::ui::{CarbonCount, UiInputsEvent};
use bevy::prelude::{EventWriter, Query};
use chemcore::daylight::read_smiles;
use gamma::graph::Graph;

pub fn process_smiles(
    carbon_count_query: &mut Query<&CarbonCount>,
    my_events: &mut EventWriter<UiInputsEvent>,
    str: String,
) -> Result<(), String> {
    for e in carbon_count_query.iter_mut() {
        println!("Current carbon count: {}", e.0);
        match parse_smiles(str) {
            Ok(carbon_count) => {
                let current = e.0;
                println!(
                    "Parsed carbon count: {}, changed: {}",
                    carbon_count.0,
                    current != carbon_count.0
                );
                my_events.send(UiInputsEvent {
                    carbon_count: carbon_count.0,
                });
                return Ok(());
            }

            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn parse_smiles(str: String) -> Result<CarbonCount, String> {
    let molecule = read_smiles(&str, None).map_err(|e| format!("{:?}", e))?;

    // TODO validations for linear alkane as we can't draw anything else

    // TODO (low prio) review usize as u32
    Ok(CarbonCount(molecule.order() as u32))
}
