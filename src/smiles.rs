use crate::ui::resource::CarbonCount;
use chemcore::daylight::read_smiles;
use gamma::graph::Graph;

pub fn process_smiles(str: String) -> Result<CarbonCount, String> {
    match parse_smiles(str) {
        Ok(carbon_count) => {
            let current = carbon_count.0;
            println!(
                "Parsed carbon count: {}, changed: {}",
                carbon_count.0,
                current != carbon_count.0
            );
            Ok(carbon_count)
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
