use crate::opts::Opt;
use crate::util;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use zip::write::FileOptions;
use std::path::PathBuf;
use zip::result::{ZipResult, ZipError};
use zip::CompressionMethod;
use zip::ZipWriter;

use walkdir::{WalkDir, DirEntry};

#[derive(Clone)]
pub struct Content {
    pub path: PathBuf,
    pub should_delete: bool
}

impl Content {
    pub fn delete(&self) {
        fs::remove_file(&self.path).expect("Error deleting file");
    }
}

fn zip_contents(args: &Vec<PathBuf>) -> Result<PathBuf, ZipError> {
    let temp = PathBuf::from("qr-transfer.zip");
    let file = File::create(&temp)?;
    let mut zip_file = ZipWriter::new(file);

    for arg in args.iter() {
        let walkdir = WalkDir::new(arg.to_str().unwrap());
        let it = walkdir.into_iter();

        zip_dir(&mut it.filter_map(|e| e.ok()), arg, &mut zip_file)?;
    }

    zip_file.finish()?;
    Result::Ok(temp)
}

fn zip_dir<T>(it: &mut Iterator<Item = DirEntry>, prefix: &PathBuf, writer: &mut ZipWriter<T>) -> ZipResult<()>
    where T: Seek+Write
{
    let options = FileOptions::default().compression_method(CompressionMethod::Bzip2);
    let mut buff = Vec::new();

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            writer.start_file_from_path(path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buff)?;
            writer.write_all(&*buff)?;
            buff.clear();
        } else if name.as_os_str().len() != 0 {
            println!("adding dir {:?} as {:?} ...", path, name);
            writer.add_directory_from_path(name, options)?;
        }
    }

    Result::Ok(())
}

pub fn get(opt: &Opt) -> Result<Content, Box<std::error::Error>> {
    let b =  util::should_be_zipped(opt)?;

    if b {
        let path = zip_contents(&opt.files)?;
        Ok(Content{path, should_delete: true})

    } else {
        let c = Content{path: opt.files[0].clone(), should_delete: false};
        Ok(c)
    }
}