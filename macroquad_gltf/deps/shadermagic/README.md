# shadermagic

`shadermagic` is a crate I wish existed. Think [Therkla's hlslparser](https://github.com/Thekla/hlslparser), but rust native.

`shadermagic` _pretend_ to be a compiler from pseudo-glsl to different glsl variants and Metal's MSL.

If you want something that actually works, check [naga](https://github.com/gfx-rs/naga), [spirvcross](https://github.com/KhronosGroup/SPIRV-Cross), [sokol-shdc](https://github.com/floooh/sokol-tools/blob/master/docs/sokol-shdc.md), [glslang](https://github.com/KhronosGroup/glslang), [glslcc](https://github.com/septag/glslcc), [hlsl parser](https://github.com/unknownworlds/hlslparser), [hlslparser fork](https://github.com/Thekla/hlslparser/blob/master/src/MSLGenerator.cpp) or even [nanoshredder](https://github.com/not-fl3/nanoshredder).

Another relevant case study is [this emscripten hack](https://github.com/emscripten-core/emscripten/blob/1336355ab0bc040c9122ef8b93aae40366920fce/src/library_webgl.js#L3065). `shadermagic` is a slightly more advanced version of the same idea.

`shadermagic` takes _some undocumented almost #version 130_ shader and apply a ton of String::replace to make `plain version 100`, `version 100 with webgl1 extensions`, `130`, `330`,  `300 es` and `metal's MSL`.

Metal is a bit of a special case, with a bit of extra code on top of `String::replace`. But it is still very sipmple string manipulations. `shadermagic` knows nothing about AST and shaders semantics.

`shadermagic` will never work well on arbitary glsl input. No amount of `string::replace` could replace a compiler. However, it might be possible to design shaders specifically for `shadermagic`, and it might take less work than hand-writing for each target. Or not! I really hope I put enough warnings here.

## How to make glsl->metal works

Enough with warnings, tips to keep this bunch of hacks afloat:
- don't access uniforms outside the `main()` function, pass them as arguments.
- put the main function last, after everything else in the shader
- annotate attribute in the vertex shader with // [[attribute(N)]]
- annotate varying in the vertex/fragment shader with // [[user(locnN)]]
- avoid functions with the same names in vertex/fragment shaders
- remember that String::replace is very stupid and does not take any context into account. Avoid naming things like "mymat3", avoid things like "BaseColor" and "Color". Yes, this is that bad.
- avoid .s/.t/.p/.q, use .x/.y/.z/.w instead

