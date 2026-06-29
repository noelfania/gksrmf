fn main() {
    embed_resource::compile("assets/icons/key2gksrmf.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
