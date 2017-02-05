use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    return text;
}

fn main() {
    let text = read_file("Integrase_Zn_blastp_1e-5_domains_A,B,C_HIV-1-gM-noRs_pol-aa_v3.txt");

    let lines = text.lines();
    let mut blast = HashMap::new(); 
    for line in lines {
        let xs: Vec<&str> = line.split("\t").collect();
        let mut blastsub = HashMap::new(); 
//        println!("query: {}, subject: {}, score: {}, identity: {}", xs[0], xs[1], xs[11], xs[2]);
        blastsub.insert(xs[1], xs[11]);
        blast.insert(xs[0], blastsub);
    }

    for (query, items) in &blast {
        for (subject, score) in items {
            println!("{} {} {}", query, subject, score);
        }
    }


}
