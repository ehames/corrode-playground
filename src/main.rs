#[macro_use] extern crate nickel;
#[macro_use] extern crate tempfile;
#[macro_use] extern crate rustc_serialize;
#[macro_use] extern crate log; extern crate env_logger;

use std::str;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use rustc_serialize::json;

use nickel::{Nickel, HttpRouter, FormBody, StaticFilesHandler};
use tempfile::NamedTempFile;

#[derive(RustcDecodable, RustcEncodable)]
enum TranspilerStatus {
    Success,
    Failure,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TranspilerOutput {
    status: TranspilerStatus,
    rustcode: String,
    error: String,
}

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut tf = NamedTempFile::new_in(Path::new("/tmp")).unwrap();
    tf.write_all(content.as_bytes()).expect("Cannot write file");
    debug!("Content saved as {}", tf.path().display());
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

    let output = Command::new("/usr/local/bin/corrode")
                         .arg(&cfile)
                         .output()
                         .expect("failed to execute process");

    let mut result = TranspilerOutput{
        status: TranspilerStatus::Success,
        rustcode: "".to_string(),
        error: "".to_string()
    };
    if output.status.success() {
        result.status = TranspilerStatus::Success;
        result.rustcode = read_file_content(rsfile);
    } else {
        result.status = TranspilerStatus::Failure;
        result.error = str::from_utf8(&output.stderr).unwrap().to_string();

    }

    json::encode(&result).unwrap()
}

fn main() {
    let mut server = Nickel::new();

    env_logger::init().unwrap();

    error!("{}", "cannot start program");
    server.utilize(StaticFilesHandler::new("/webapp/"));
    server.post("/corrode", middleware! { |request, response|

        let form_body = try_with!(response, request.form_body());
        let ccode = form_body.get("ccode").unwrap_or("tu hna");
        debug!("C code in request '{:?}'", ccode);

        let tf = create_temp_file_with_content(ccode);
        let rustcode = transpile(&tf);

        format!("{}", rustcode)
    });

    server.listen("0.0.0.0:6767");
}
