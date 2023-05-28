use std::{
    collections::HashSet,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use tauri_bindgen_core::GeneratorBuilder;
use tauri_bindgen_gen_guest_ts::Builder as TsBuilder;
use tauri_bindgen_gen_host::Builder as RsHostBuilder;
const WIT_DIR_PATH: &str = "./wits";
const WIT_FILE_RS_PATH: &str = "./src/wits.rs";
const WIT_TS_DIR_PATH: &str = "../../src/wits";
const TS_FILE_HEADERS: &str = "// @ts-nocheck\n";
const RS_ATTR_HEADERS: &str = "#[allow(clippy::all)]\n";

fn main() {
    watch_wits().unwrap();
    tauri_build::build();
}

fn watch_wits() -> std::io::Result<()> {
    fs::create_dir_all(WIT_DIR_PATH)?;
    let wit_read_dir = fs::read_dir(WIT_DIR_PATH)?;
    fs::create_dir_all(WIT_TS_DIR_PATH)?;
    let mut rs_wit_codes = vec![];
    let ts_dir_path = Path::new(WIT_TS_DIR_PATH);
    let mut allowed_ts_files = HashSet::new();
    for wit_entry in wit_read_dir.flatten() {
        let wit_path = wit_entry.path();
        let iface =
            wit_parser::parse_and_resolve_str(&fs::read_to_string(wit_path).unwrap(), |_| false)
                .expect("wit file error");
        let rs_builder = RsHostBuilder {
            fmt: true,
            tracing: true,
            async_: false,
        };
        let rs_general_contents = rs_builder.build(iface.clone()).to_file().1;
        let mut rs_mod_code =
            String::with_capacity(RS_ATTR_HEADERS.len() + rs_general_contents.len());
        rs_mod_code.push_str(RS_ATTR_HEADERS);
        rs_mod_code.push_str(&rs_general_contents);
        rs_wit_codes.push(rs_mod_code);
        let ts_builder = TsBuilder::default();
        let (ts_file_name, ts_file_contents) = ts_builder.build(iface).to_file();
        let mut ts_file_code =
            String::with_capacity(TS_FILE_HEADERS.len() + ts_file_contents.len());
        ts_file_code.push_str(TS_FILE_HEADERS);
        ts_file_code.push_str(&ts_file_contents);
        write_file(ts_dir_path.join(&ts_file_name), ts_file_code)?;
        allowed_ts_files.insert(ts_file_name.to_owned());
    }
    write_file(WIT_FILE_RS_PATH, rs_wit_codes.join("\n\n"))?;
    let wit_ts_read_dir = fs::read_dir(WIT_TS_DIR_PATH)?;
    for ts_file_entry in wit_ts_read_dir.flatten() {
        if !allowed_ts_files.contains(&PathBuf::from(ts_file_entry.file_name())) {
            fs::remove_file(ts_file_entry.path())?;
        }
    }
    println!("cargo:rerun-if-changed={}/*.wit", WIT_DIR_PATH);
    Ok(())
}

fn write_file<P, C>(path: P, contents: C) -> std::io::Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let mut old_contents = None;
    if let Ok(mut content) = std::fs::File::open(path.as_ref()) {
        let mut buf = String::new();
        let result = content.read_to_string(&mut buf);
        if result.is_ok() {
            old_contents = Some(buf);
        }
    }
    if let Some(old_contents) = old_contents {
        if contents.as_ref() == old_contents.as_bytes() {
            return Ok(());
        }
    }
    fs::write(path, contents)
}
