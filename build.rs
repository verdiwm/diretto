fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    pkg_config::Config::new()
        .probe("gbm")
        .expect("Failed to link to libgbm");
}
