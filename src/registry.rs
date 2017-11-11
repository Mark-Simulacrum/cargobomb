use crates_index;
use dirs::CRATES_IO_INDEX;
use errors::*;
use std::path::PathBuf;

pub fn crates_index_registry() -> Result<crates_index::Index> {
    let index = crates_index::Index::new(PathBuf::from(&*CRATES_IO_INDEX));
    if index.exists() {
        info!("Fetching latest 'crates.io-index' repository commits");
        index.update()?;
    } else {
        info!("Cloning 'crates.io-index' repository");
        index.retrieve()?;
    }
    Ok(index)
}
