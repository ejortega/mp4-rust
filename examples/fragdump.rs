use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::SeekFrom;

use mp4::Result;
use mp4::mp4box::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: fragdump <filename_in> <filename_out>");
        std::process::exit(1);
    }

    if let Err(err) = dump(&args[1], &args[2]) {
        let _ = writeln!(io::stderr(), "{}", err);
    }
}

fn dump<P: AsRef<Path>>(filename: &P, out: &P) -> Result<()> {
    let f = File::open(filename)?;
    let size = f.metadata()?.len();
    let mut reader = BufReader::new(f);
    
    let start = reader.seek(SeekFrom::Current(0))?;
    
    let mut styp = None;
    let mut sidx = None;
    let mut moof = None;
    let mut mdat = None;

    let mut current = start;
    while current < size {
        let header = BoxHeader::read(&mut reader)?;
        let BoxHeader { name, size: s } = header;

        match name {
            BoxType::SidxBox => {
                println!(" sidx in len {}", s);
                sidx = Some(SidxBox::read_box(&mut reader, s)?);
            }
            BoxType::MoofBox => {
                println!(" moof in len {}", s);
                moof = Some(MoofBox::read_box(&mut reader, s)?);
            }
            BoxType::MdatBox => {
                println!(" mdat in len {}", s);
                let mut vec_mdat = vec![0; (s - 8) as usize];
                reader.read_exact(&mut vec_mdat)?;
                mdat = Some(vec_mdat);
            }
            BoxType::StypBox => {
                println!(" styp in len {}", s);
                styp = Some(FtypBox::read_box(&mut reader, s)?);
            }
            b => {
                println!("WARN: got unexpected box {:?}", b);
                skip_box(&mut reader, s)?;
            }
        }

        current = reader.seek(SeekFrom::Current(0))?;
    }

    let styp = styp.unwrap();
    let sidx = sidx.unwrap();
    let moof = moof.unwrap();
    let mdat = mdat.unwrap();

let mut vec = File::create(out)?;
//    let mut vec = Vec::new();
    let t = styp.write_box(&mut vec)?;
//    println!(" styp out len {} vs {}", t, vec.len());
//    let mut vec = Vec::new();
    let t = sidx.write_box(&mut vec)?;
//    println!(" sidx out len {} vs {}", t, vec.len());
//    let mut vec = Vec::new();
    let t = moof.write_box(&mut vec)?;
//    println!(" moof out len {} vs {}", t, vec.len());
    let mdat_hdr = BoxHeader::new(BoxType::MdatBox, mdat.len() as u64 + 8);
//    let mut vec = Vec::new();
    let t = mdat_hdr.write(&mut vec)?;
    let t = t + vec.write(&mdat)? as u64;
//    println!(" mdat out len {} vs {}", t, vec.len());

    Ok(())
}
