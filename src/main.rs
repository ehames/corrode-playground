#[macro_use] extern crate nickel;
#[macro_use] extern crate tempfile;

use std::str;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use nickel::{Nickel, HttpRouter, FormBody};
use tempfile::NamedTempFile;

fn create_temp_file_with_content(ccode: &str) -> NamedTempFile {
    let mut tf = NamedTempFile::new_in(Path::new("/tmp")).unwrap();
    tf.write_all(ccode.as_bytes()).expect("Cannot write file");
    tf
}

fn read_file_content(path: PathBuf) -> String {
    let mut rf = fs::File::open(path).expect("cannot open file");
    let mut rustcode = String::new();
    rf.read_to_string(&mut rustcode).expect("cannot read file");
    rustcode
}

fn transpile(tf: &NamedTempFile) -> String {

    let cfile = tf.path().with_extension("c");
    let rsfile = tf.path().with_extension("rs");

    fs::rename(tf.path(), &cfile).expect("cannot rename tmp file with c extension");

    Command::new("/usr/local/bin/docker")
                         .arg("run")
                         .arg("--rm")
                         .arg("-v")
                         .arg("/tmp:/tmp")
                         .arg("corrode:latest")
                         .arg("/root/.local/bin/corrode")
                         .arg(&cfile)
                         .output()
                         .expect("failed to execute process");

    let rustcode = read_file_content(rsfile);
    rustcode
}

fn main() {
    let mut server = Nickel::new();

    server.post("/corrode", middleware! { |request, response|

        let form_body = try_with!(response, request.form_body());
        let ccode = form_body.get("ccode").unwrap_or("tu hna");

        let tf = create_temp_file_with_content(ccode);
        let rustcode = transpile(&tf);

        format!("rustcode : {} ", rustcode)
    });

    server.listen("127.0.0.1:6767");
}
