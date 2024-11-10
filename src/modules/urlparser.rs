use reqwest::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Parser {
    url: String,
}


impl Parser {
    fn new(url: String) -> Parser {
        Parser { url }
    }

    async fn fetch(&self) -> Result<(), Error> {
        if (self.url.is_empty()) {
            return Ok(());
        }

        let response = reqwest::get(self.url.as_str()).await?;
        let bytes = response.bytes().await?;
        let archive = "archive.zip";
        let mut file = File::create(archive)?;
        file.write_all(&bytes)?;

        Ok(())
    }
}


fn extract(path: &Path, mut target: &Path) {
    let zipfile = std::fs::File::open(&path).unwrap();
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    if !target.exists() {
        fs::create_dir_all(target).unwrap();
    }

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        println!("Filename: {} {:?}", file.name(), file.sanitized_name());
        if (file.is_dir()) {
            println!("file path: {:?}", file.name_raw());
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target).unwrap();
        } else {
            let filepath = target.join(Path::new(file.name()));
            let mut target_file = if !filepath.exists() {
            }

        }
    }
}