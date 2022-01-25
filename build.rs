use std::env;

fn main()
{
    let mcas_build_dir = env::var("MCAS_LIB_DIR").expect("MCAS_LIB_DIR variable not set");
    println!("cargo:rustc-link-search=native=/usr/lib64/");
    println!("cargo:rustc-link-search=native=/usr/lib/");
    let search_native = "cargo:rustc-link-search=native=".to_owned() + &mcas_build_dir;
    println!("{}",search_native);
    println!("cargo:rustc-link-lib=common");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=numa");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=mcasapi");
}

