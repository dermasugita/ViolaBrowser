use crate::Vcf2bedpe;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn main(vcf2bedpe: &Vcf2bedpe) {
    let filepath = &vcf2bedpe.vcf[0];
    let vcf_file = File::open(filepath);
    let mut reader = BufReader::new(vcf_file.unwrap());
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}