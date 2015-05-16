
fn main() {
  println!("cargo:rustc-link-search=native=/Users/yorkie/workspace/rustjs/out/Release");
  println!("cargo:rustc-link-lib=static=rustjs_deps");
}