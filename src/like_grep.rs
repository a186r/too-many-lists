use std::path::Path;
use std::fs::{DirEntry, File};
use std::{io, fs};
use std::io::BufRead;

//对当前目录进行递归、遍历、每当查找到文件的时候，回调一个函数
fn visit_dirs(dir: &Path, pattern: &String, cb: &fn(&DirEntry, &String)) -> io::Result<()>{
    if try!(fs::metadata(dir)).is_dir(){
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir(){
                try!(visit_dirs(&entry.path(), pattern, cb));
            }else{
                cb(&entry, pattern);
            }
        }
    }else {
        let entry = try!(try!(fs::read_dir(dir)).next().unwrap());
        cb(&entry, pattern);
    }
    Ok(())
}

fn call_back(de: &DirEntry, pt: &String){
    let mut f = File::open(de.path()).unwrap();
    let mut buf = io::BufReader::new(f);
    for line in io::BufReader::lines(buf){
        let line = line.unwrap_or("".to_string());
        if line.contains(pt){
            println!("{}", &line);
        }
    }
}