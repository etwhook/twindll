use std::fs::OpenOptions;
use pelite::{pe::{Pe, PeFile}, FileMap};
use clap::{command , Arg};
use std::io::Write;
fn main() {
    let matches = command!()
    .arg(Arg::new("dll").long("dll").short('d'))
    .arg(Arg::new("output").long("output").short('o'))
    .arg(Arg::new("specialadd").long("specialadd").short('s'))
    .get_matches();
    let dll: &String = matches.get_one("dll").unwrap();
    let output: &String = matches.get_one("output").unwrap();
    let special_add: &String = matches.get_one("specialadd").unwrap();
    let mut output_file = OpenOptions::new()
    .create(true)
    .append(true)
    .write(true)
    .open(output).expect("Failed to open output file.");
    println!(r"
    .sSSSs.  SSSSS                                     
    .sSSSSSSSSSSSSSs. .sSSS       s.    SSSSS SSSSS SS SSSSS .sSSSSs.    SSSSS       SSSSS       
    SSSSS S SSS SSSSS S SSS       SSSs. S SSS S SSS  `sSSSSS S SSSSSSSs. S SSS       S SSS       
    SSSSS S  SS SSSSS S  SS       SSSSS S  SS S  SS    SSSSS S  SS SSSSS S  SS       S  SS       
    `:S:' S..SS `:S:' S..SS       SSSSS S..SS S..SS    SSSSS S..SS SSSSS S..SS       S..SS       
          S:::S       S:::S       SSSSS S:::S S:::S    SSSSS S:::S SSSSS S:::S       S:::S       
          S;;;S       S;;;S   S   SSSSS S;;;S S;;;S    SSSSS S;;;S SSSSS S;;;S       S;;;S       
          S%%%S       S%%%S  SSS  SSSSS S%%%S S%%%S    SSSSS S%%%S SSSS' S%%%S SSSSS S%%%S SSSSS 
          SSSSS       SSSSSsSS SSsSSSSS SSSSS SSSSS    SSSSS SSSSSsS;:'  SSSSSsSS;:' SSSSSsSS;:'  
    ");

    let pe_map = FileMap::open(dll).expect("Failed to read PE into memory.");
    let pe = PeFile::from_bytes(&pe_map).expect("Failed to get PE object.");
    let exports = pe.exports().expect("Failed to get Exports.");
    let query = exports.by().expect("Failed to get Export Iterator.");
    let mut count = 0;
    for result in query.iter_names() {
        if let (Ok(name) , Ok(_)) = result {
            let dllless = dll.strip_suffix(".dll").expect("Failed to strip .dll.").replace(r"\", r"\\"); 
            let comment = format!("#pragma comment(linker , \"/export:{}={}{}.{}\")\n",name, dllless,special_add,name);
            println!("[+] Proxied -> {name}");
            count += 1;
            output_file.write(comment.as_bytes()).unwrap();
        }
    }
    println!("[*] Redirected {count} Routines.");

}