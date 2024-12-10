
mod utils;
use utils::{parser, calibration_checker, errors::BridgeRepairError};

fn main() -> Result<(), BridgeRepairError> {
    let input = parser::parse_input("src/input/tst.txt")?;
    let result = calibration_checker::check_calibration(input)?;
    println!("{:?}", result);
    Ok(())
}
