fn main() -> Result<(), ()> {
    let libc = sloop::DependencyBuilder::new("deps/libc")
        .edition("2015")
        .build()?;
    let miniquad = sloop::DependencyBuilder::new("deps/miniquad")
        .with_dependency(&libc)
        .build()?;

    sloop::Builder::new()
        .binary()
        .name("TriangleOnTheSloop")
        .entrypoint("src/triangle.rs")
        .with_dependency(&libc)
        .with_dependency(&miniquad)
        .build()
}
