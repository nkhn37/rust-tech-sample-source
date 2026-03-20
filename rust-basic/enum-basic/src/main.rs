use common::list_examples;

fn main() {
    list_examples(env!("CARGO_MANIFEST_DIR"), env!("CARGO_PKG_NAME"));
}
