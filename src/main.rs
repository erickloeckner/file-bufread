use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;

const PROJECT: &str = "file-bufread";

fn main() -> io::Result<()> {
    let home_str = env::var("HOME").unwrap();
    let mut home_path = PathBuf::from(home_str);
    //~ println!("{:?}", home_path);
    
    home_path.push(PROJECT);
    home_path.push("log.txt");
    let f = File::open(home_path.to_str().unwrap())?;
    let mut f = BufReader::new(f);
    
    let per_ip = req_per_ip(&mut f);
    let mut per_ip_vec: Vec<_> = per_ip.iter().collect();
    per_ip_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    
    println!("IP: # of requests");
    print_map(&per_ip_vec, 10);
    println!("-----------------");
    
    let per_uri = req_per_uri(&mut f);
    let mut per_uri_vec: Vec<_> = per_uri.iter().collect();
    per_uri_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    
    println!("URI: # of requests");
    print_map(&per_uri_vec, 10);
    println!("-----------------");

    Ok(())
}

fn req_per_ip(buf: &mut BufReader<File>) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        let line_c = line.unwrap();
        let field = line_c.split(' ').nth(0).unwrap();
        match out.get_mut(field) {
            Some(v) => *v += 1,
            None => {
                out.insert(field.to_string(), 1);
                ()
            },
        }
    }
    out
}

fn req_per_uri(buf: &mut BufReader<File>) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        let line_c = line.unwrap();
        let field = line_c.split(' ').nth(6).unwrap();
        match out.get_mut(field) {
            Some(v) => *v += 1,
            None => {
                out.insert(field.to_string(), 1);
                ()
            },
        }
    }
    out
}

fn print_map(map: &Vec<(&String, &u32)>, num: usize) {
    for (key, val) in map.iter().take(num) {
        println!("{}: {}", key, val);
    }
}
