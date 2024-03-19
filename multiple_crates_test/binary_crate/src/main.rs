// its binary is example how to create few WORKSPACES with separate crates and import outside lib crate inside main
// see toml file as well
use lib_crate::this_lib_crate;

fn main() {
    println!("Invoke from main: {}", this_lib_crate());
}
