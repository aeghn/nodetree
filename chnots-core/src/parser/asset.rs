use std::path::PathBuf;

use chin_tools::utils::pathutils::split_uuid_to_file_name;

use crate::config::Config;

pub fn asset_path_by_uuid(config: &Config, id: &str) -> PathBuf {
    let filename_parts = split_uuid_to_file_name(&id);

    let save_filepath = std::path::Path::new(&config.common.asset_base_dir)
        .join(filename_parts.0)
        .join(filename_parts.1)
        .join(filename_parts.2);
    save_filepath
}
