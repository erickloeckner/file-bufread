use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("/home/pi/rust/file-bufread/log.txt")?;
    let mut f = BufReader::new(f);
    
    let per_ip = req_per_ip(&mut f);
    let mut per_ip_vec: Vec<_> = per_ip.iter().collect();
    per_ip_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    
    println!("IP: # of requests");
    print_map(&per_ip_vec, 10);
    
    let per_uri = req_per_uri(&mut f);
    let mut per_uri_vec: Vec<_> = per_uri.iter().collect();
    per_uri_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    
    println!("URI: # of requests");
    print_map(&per_uri_vec, 10);
    //~ print_map(&per_uri_vec, 2);

    Ok(())
}

fn req_per_ip(buf: &mut BufReader<File>) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        for (field_i, field) in line.unwrap().split(' ').enumerate() {
            if field_i == 0 {
                match out.get_mut(field) {
                    Some(v) => *v += 1,
                    None => {
                        out.insert(field.to_string(), 1);
                        ()
                    },
                }
            }
        }
    }
    out
}

fn req_per_uri(buf: &mut BufReader<File>) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    buf.seek(io::SeekFrom::Start(0)).unwrap();
    
    for line in buf.by_ref().lines() {
        for (field_i, field) in line.unwrap().split(' ').enumerate() {
            if field_i == 6 {
                match out.get_mut(field) {
                    Some(v) => *v += 1,
                    None => {
                        out.insert(field.to_string(), 1);
                        ()
                    },
                }
            }
        }
    }
    out
}

fn print_map(map: &Vec<(&String, &u32)>, num: usize) {
    for (key, val) in map.iter().take(num) {
        println!("{}: {}", key, val);
    }
}
