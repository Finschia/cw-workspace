if variable::get("workspace") == "false" {
    file::delete("Cargo.toml");
    file::rename("Cargo-single.toml",  "Cargo.toml");
} else {
    file::delete("Cargo-single.toml");
}
