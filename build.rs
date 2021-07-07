use clap_generate::{
    generate_to,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
include!("src/app.rs");

const BIN_NAME: &str = "mtag";

fn main() {
    let mut app = new();
    app.set_bin_name(BIN_NAME);
    let outdir = env!("CARGO_OUT_DIR");
    generate_to::<Bash, _, _>(&mut app, BIN_NAME, outdir);
    generate_to::<Elvish, _, _>(&mut app, BIN_NAME, outdir);
    generate_to::<Fish, _, _>(&mut app, BIN_NAME, outdir);
    generate_to::<PowerShell, _, _>(&mut app, BIN_NAME, outdir);
    generate_to::<Zsh, _, _>(&mut app, BIN_NAME, outdir);
}
