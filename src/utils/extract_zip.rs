use std::path::Path;
use std::io::prelude::*;

use crate::types::ErrBox;

// todo: consolidate with code in dprint (maybe move to crate?)

pub fn extract_zip(zip_bytes: &[u8], dir_path: &Path) -> Result<(), ErrBox> {
    // adapted from https://github.com/mvdnes/zip-rs/blob/master/examples/extract.rs
    let reader = std::io::Cursor::new(&zip_bytes);
    let mut zip = zip::ZipArchive::new(reader)?;

    // todo: consider parallelizing this
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let file_name = file.sanitized_name();
        let file_path = dir_path.join(file_name);

        if !file.is_dir() {
            if let Some(parent_dir_path) = file_path.parent() {
                std::fs::create_dir_all(&parent_dir_path)?
            }
            let mut file_bytes = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut file_bytes)?;
            std::fs::write(&file_path, &file_bytes)?;
        } else {
            std::fs::create_dir_all(&file_path)?
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            use std::fs;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&file_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}