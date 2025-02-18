use zip::read::ZipArchive;
use std::io::Cursor;
use std::io;
use std::fs;
use std::fs::Permissions;
use std::path::Path;

pub fn extract_zip(mut zip: ZipArchive<Cursor<Vec<u8>>>, base_path: &str) {
    fs::remove_dir_all("/etc/nodogsplash/htdocs").expect("Can't remove old files");
    fs::create_dir("/etc/nodogsplash/htdocs").expect("Can't create new directory");

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let final_path = match file.enclosed_name() {
            Some(path) => std::format!("{}{}", base_path, path.to_str().unwrap()),
            None => continue,
        };
        let outpath = Path::new(&final_path);
        
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}