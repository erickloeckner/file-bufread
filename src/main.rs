use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;

const PROJECT: &str = "file-bufread";

struct ReqCount {
    name: String,
    count: u32,
}

fn main() -> io::Result<()> {
    let home_str = env::var("HOME").unwrap();
    let mut home_path = PathBuf::from(home_str);
    home_path.push(PROJECT);
    home_path.push("log.txt");
    
    let f = File::open(home_path.to_str().unwrap())?;
    let mut f = BufReader::new(f);
    
    let mut per_ip = req_per_ip(&mut f);
    per_ip.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    
    let mut per_uri = req_per_uri(&mut f);
    per_uri.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    
    println!("IP: # of requests");
    print_map(&per_ip, 10);
    println!("-----------------");
    
    println!("URI: # of requests");
    print_map(&per_uri, 10);
    println!("-----------------");
    
    Ok(())
}

fn req_per_ip(buf: &mut BufReader<File>) -> Vec<ReqCount> {
    let mut out: Vec<ReqCount> = Vec::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        let line_c = line.unwrap();
        let field = line_c.split(' ').nth(0);
        match field {
            Some(v) => {
                vec_get(&mut out, v);
            },
            None => (),
        }
    }
    out
}

fn req_per_uri(buf: &mut BufReader<File>) -> Vec<ReqCount> {
    let mut out: Vec<ReqCount> = Vec::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        let line_c = line.unwrap();
        let field = line_c.split(' ').nth(6);
        match field {
            Some(v) => {
                vec_get(&mut out, v);
            },
            None => (),
        }
    }
    out
}

fn print_map(map: &Vec<ReqCount>, num: usize) {
    for i in map.iter().take(num) {
        println!("{}: {}", i.name, i.count);
    }
}

fn vec_get(v: &mut Vec<ReqCount>, s: &str) {
    let pos_ref = v.iter().enumerate().filter(|i| (i.1).name == s).next();
    
    match pos_ref {
        Some(i) => { 
            let pos = i.0;
            v[pos].count += 1;
        },
        None => {
            v.push(ReqCount {name: String::from(s), count: 1});
        },
    }
}
