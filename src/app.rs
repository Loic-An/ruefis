use uefi::table::{SystemTable, Boot};
pub fn run(_system_table:&'static SystemTable<Boot>) -> Result<(), ()>{
    Ok(())
}