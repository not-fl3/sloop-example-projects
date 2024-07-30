fn main() -> Result<(), ()> {
    let nanoserde_derive = sloop::DependencyBuilder::new("deps/nanoserde/derive")
        .proc_macro(true)
        .with_feature("json")
        .crate_name("nanoserde_derive")
        .build()?;

    let nanoserde = sloop::DependencyBuilder::new("deps/nanoserde")
        .with_feature("std")
        .with_feature("json")
        .with_dependency(&nanoserde_derive)
        .build()?;

    sloop::Builder::new()
        .binary()
        .name("ProcMacro")
        .with_dependency(&nanoserde)
        .build()
}
