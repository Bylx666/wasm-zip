fn main() {
  let mut args = std::env::args_os();
  args.next();
  let path = args.next().expect("需要传入文件路径");
  let mut m = walrus::Module::from_file(&path).expect("找不到该文件");
  let mut out_name = std::path::PathBuf::from(path);
  let mut name = out_name.file_stem().expect("文件后缀需要是wasm").to_os_string();
  name.push("_opt");
  out_name.set_file_name(name);
  let _ = m.emit_wasm_file(out_name);
}
