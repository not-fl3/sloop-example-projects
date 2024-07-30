fn main() -> Result<(), ()> {
    let deps_opt_level = 3;

    let libc = sloop::DependencyBuilder::new("deps/libc")
        .edition("2015")
        .optimization(deps_opt_level)
        .build()?;
    let miniquad = sloop::DependencyBuilder::new("deps/miniquad")
        .with_feature("log-impl")
        .with_dependency(&libc)
        .optimization(deps_opt_level)
        .build()?;
    let mint = sloop::DependencyBuilder::new("deps/mint").build()?;
    let glam = sloop::DependencyBuilder::new("deps/glam-rs")
        .crate_name("glam")
        .with_dependency(&mint)
        .optimization(deps_opt_level)
        .build()?;
    let dolly = sloop::DependencyBuilder::new("deps/dolly")
        .with_dependency(&mint)
        .with_dependency(&glam)
        .optimization(deps_opt_level)
        .build()?;
    let ttfparser = sloop::DependencyBuilder::new("deps/ttf-parser")
        .with_feature("std")
        .with_feature("opentype-layout")
        .with_feature("apple-layout")
        .with_feature("variable-fonts")
        .with_feature("glyph-names")
        .optimization(deps_opt_level)
        .build()?;
    let fontdue = sloop::DependencyBuilder::new("deps/fontdue")
        .with_dependency(&ttfparser)
        .optimization(deps_opt_level)
        .build()?;
    let nanoserde_derive = sloop::DependencyBuilder::new("deps/nanoserde/derive")
        .proc_macro(true)
        .with_feature("json")
        .crate_name("nanoserde_derive")
        .optimization(deps_opt_level)
        .build()?;
    let nanoserde = sloop::DependencyBuilder::new("deps/nanoserde")
        .with_feature("std")
        .with_feature("json")
        .with_dependency(&nanoserde_derive)
        .optimization(deps_opt_level)
        .build()?;
    let nanogltf = sloop::DependencyBuilder::new("deps/nanogltf")
        .with_dependency(&nanoserde)
        .optimization(deps_opt_level)
        .build()?;
    let shadermagic = sloop::DependencyBuilder::new("deps/shadermagic")
        .with_dependency(&nanoserde)
        .with_dependency(&miniquad)
        .optimization(deps_opt_level)
        .build()?;
    let slotmap = sloop::DependencyBuilder::new("deps/slotmap")
        .optimization(deps_opt_level)
        .build()?;

    let zune_core = sloop::DependencyBuilder::new("deps/zune-image/zune-core")
        .with_feature("std")
        .optimization(deps_opt_level)
        .build()?;
    let simd_adler32 = sloop::DependencyBuilder::new("deps/simd-adler32")
        .optimization(deps_opt_level)
        .build()?;
    let zune_inflate = sloop::DependencyBuilder::new("deps/zune-image/zune-inflate")
        .with_feature("std")
        .with_feature("zlib")
        .with_feature("gzip")
        .with_dependency(&simd_adler32)
        .optimization(deps_opt_level)
        .build()?;
    let zune_jpeg = sloop::DependencyBuilder::new("deps/zune-image/zune-jpeg")
        .with_feature("std")
        .with_dependency(&zune_core)
        .optimization(deps_opt_level)
        .build()?;
    let zune_png = sloop::DependencyBuilder::new("deps/zune-image/zune-png")
        .with_feature("std")
        .with_dependency(&zune_core)
        .with_dependency(&zune_inflate)
        .optimization(deps_opt_level)
        .build()?;
    let quad_gl = sloop::DependencyBuilder::new("deps/quad-gl")
        .with_dependency(&zune_core)
        .with_dependency(&zune_png)
        .with_dependency(&zune_jpeg)
        .with_dependency(&slotmap)
        .with_dependency(&shadermagic)
        .with_dependency(&glam)
        .with_dependency(&fontdue)
        .with_dependency(&miniquad)
        .optimization(deps_opt_level)
        .build()?;
    let quad_rand = sloop::DependencyBuilder::new("deps/quad-rand")
        .optimization(deps_opt_level)
        .build()?;

    let macroquad = sloop::DependencyBuilder::new("deps/macroquad")
        .with_dependency(&quad_gl)
        .with_dependency(&quad_rand)
        .with_dependency(&glam)
        .with_dependency(&miniquad)
        .with_dependency(&nanogltf)
        .optimization(deps_opt_level)
        .build()?;

    sloop::Builder::new()
        .binary()
        .name("GltfOnTheSloop")
        .entrypoint("src/main.rs")
        .with_dependency(&dolly)
        .with_dependency(&macroquad)
        .build()
}
