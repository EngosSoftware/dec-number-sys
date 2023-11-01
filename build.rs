fn main() {
  let output_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let target_endian = if cfg!(target_endian = "little") { "1" } else { "0" };
  cc::Build::new()
    .define("DECLITEND", Some(target_endian))
    .file("./decNumber-icu-368/decimal128.c")
    .file("./decNumber-icu-368/decimal64.c")
    .file("./decNumber-icu-368/decimal32.c")
    .file("./decNumber-icu-368/decContext.c")
    .file("./decNumber-icu-368/decNumber.c")
    .file("./decNumber-icu-368/decSingle.c")
    .file("./decNumber-icu-368/decDouble.c")
    .file("./decNumber-icu-368/decQuad.c")
    .file("./decNumber-icu-368/decPacked.c")
    .out_dir(output_dir.join("lib"))
    .compile("decnumber-icu-368");
}
