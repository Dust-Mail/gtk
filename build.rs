fn main() {
    glib_build_tools::compile_resources(
        &["data/gtk"],
        "data/gtk/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
}
