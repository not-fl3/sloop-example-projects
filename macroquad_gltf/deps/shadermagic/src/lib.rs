mod metal;

#[derive(Debug)]
pub struct Error {
    pub error: String,
    pub line: Option<u32>,
}

#[derive(Default, Debug)]
pub struct GlslOutput {
    pub vertex: String,
    pub fragment: String,
}

#[derive(Default)]
pub struct Output {
    pub v100: GlslOutput,
    pub v100_webgl: GlslOutput,
    pub v130: GlslOutput,
    pub v330: GlslOutput,
    pub v300es: GlslOutput,
    pub metal: String,
}

#[derive(Default, Debug, PartialEq)]
pub struct Options {
    pub precision: String,
    /// D3D12 and Metal
    /// NDC: +Y is up. Point(-1, -1) is at the bottom left corner.
    /// Framebuffer coordinate: +Y is down. Origin(0, 0) is at the top left corner.
    /// Texture coordinate: +Y is down. Origin(0, 0) is at the top left corner.
    /// OpenGL, OpenGL ES and WebGL

    /// NDC: +Y is up. Point(-1, -1) is at the bottom left corner.
    /// Framebuffer coordinate: +Y is up. Origin(0, 0) is at the bottom left corner.
    /// Texture coordinate: +Y is up. Origin(0, 0) is at the bottom left corner.
    ///
    /// Neither shadermagic nor miniquad got a proper solution for a different Y axis for
    /// framebuffer.s
    /// metal_flip_y makes metal vertex shader to automatically flip do something like
    /// `gl_Position.y = -gl_Position.y`, which helps to avoid certain `#ifdef __METAL`
    /// for shaders rendering to framebuffers
    pub metal_flip_y: bool,

    pub defines: Vec<String>,
}

enum ShaderKind {
    Vertex,
    Fragment,
}

fn lower_gl_missing_math() -> &'static str {
    r#"mat3 transpose(mat3 m) {
    return mat3(vec3(m[0].x, m[1].x, m[2].x),
                vec3(m[0].y, m[1].y, m[2].y),
                vec3(m[0].z, m[1].z, m[2].z));
}
"#
}

fn glsl_v100(input: &str, _kind: ShaderKind, defines: &[String]) -> String {
    let mut processed = String::new();

    processed.push_str("#version 100\n");
    processed.push_str("precision mediump float;\n");
    processed.push_str("float dFdx(float x) {return 0.0;}\n");
    processed.push_str("float dFdy(float x) {return 0.0;}\n");
    processed.push_str("vec2 dFdx(vec2 x) {return vec2(0.0);}\n");
    processed.push_str("vec2 dFdy(vec2 x) {return vec2(0.0);}\n");
    processed.push_str("vec3 dFdx(vec3 x) {return vec3(0.0);}\n");
    processed.push_str("vec3 dFdy(vec3 x) {return vec3(0.0);}\n");
    processed.push_str("#define NO_DERIVATIVES 1\n");
    processed.push_str("#define textureCubeLod(x, y, z) textureCube(x, y)\n");
    processed.push_str("#define sm_level(x) x\n");
    processed.push_str(lower_gl_missing_math());

    for define in defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str("#define __GL 1\n");

    for line in input.lines() {
        processed.push_str(&line);
        processed.push('\n');
    }

    processed
}

fn glsl_v100_webgl(input: &str, _kind: ShaderKind, defines: &[String]) -> String {
    let mut processed = String::new();

    processed.push_str("#version 100\n");
    processed.push_str("#extension GL_EXT_shader_texture_lod: enable\n");
    processed.push_str("#extension GL_OES_standard_derivatives: enable\n");
    processed.push_str("precision mediump float;\n");
    processed.push_str(lower_gl_missing_math());

    for define in defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str("#define __GL 1\n");
    processed.push_str("#define sm_level(x) x\n");

    for line in input.lines() {
        let line = line.replace("textureCubeLod", "textureCubeLodEXT");
        processed.push_str(&line);
        processed.push('\n');
    }

    processed
}

fn glsl_v130(input: &str, _kind: ShaderKind, defines: &[String]) -> String {
    let mut processed = String::new();

    processed.push_str("#version 130\n");
    processed.push_str("#define sm_level(x) x\n");
    processed.push_str(lower_gl_missing_math());

    for define in defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str("#define __GL 1\n");

    for line in input.lines() {
        processed.push_str(&line);
        processed.push('\n');
    }

    processed
}

fn glsl_v330(input: &str, kind: ShaderKind, defines: &[String]) -> String {
    let mut processed = String::new();

    processed.push_str("#version 330\n");
    for define in defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str("#define __GL 1\n");
    if let ShaderKind::Fragment = kind {
        processed.push_str("out vec4 output_FragColor;\n");
    }
    processed.push_str("#define sm_level(x) x\n");

    // #extension GL_EXT_shader_texture_lod: enable
    // #extension GL_OES_standard_derivatives : enable
    // precision mediump float;

    for line in input.lines() {
        let line = line
            .replace("attribute", "in")
            .replace("texture2D", "texture")
            .replace(
                "varying",
                match kind {
                    ShaderKind::Vertex => "out",
                    ShaderKind::Fragment => "in",
                },
            )
            .replace("textureCube", "texture")
            .replace("gl_FragColor", "output_FragColor");
        processed.push_str(&line);
        processed.push('\n');
    }
    processed
}

fn glsl_v300es(input: &str, kind: ShaderKind, defines: &[String]) -> String {
    let mut processed = String::new();

    processed.push_str("#version 300 es\n");
    processed.push_str("precision mediump float;\n");
    for define in defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str("#define __GL 1\n");
    if let ShaderKind::Fragment = kind {
        processed.push_str("out vec4 output_FragColor;\n");
    }
    processed.push_str("#define sm_level(x) x\n");

    for line in input.lines() {
        let line = line
            .replace("attribute", "in")
            .replace("texture2D", "texture")
            .replace(
                "varying",
                match kind {
                    ShaderKind::Vertex => "out",
                    ShaderKind::Fragment => "in",
                },
            )
            .replace("textureCube", "texture")
            .replace("gl_FragColor", "output_FragColor");
        processed.push_str(&line);
        processed.push('\n');
    }
    processed
}

pub fn transform(
    fragment: &str,
    vertex: &str,
    meta: &miniquad::ShaderMeta,
    options: &Options,
) -> Result<Output, Error> {
    let mut output = Output::default();
    output.v100 = GlslOutput {
        fragment: glsl_v100(fragment, ShaderKind::Fragment, &options.defines),
        vertex: glsl_v100(vertex, ShaderKind::Vertex, &options.defines),
    };
    output.v130 = GlslOutput {
        fragment: glsl_v130(fragment, ShaderKind::Fragment, &options.defines),
        vertex: glsl_v130(vertex, ShaderKind::Vertex, &options.defines),
    };
    output.v100_webgl = GlslOutput {
        fragment: glsl_v100_webgl(fragment, ShaderKind::Fragment, &options.defines),
        vertex: glsl_v100_webgl(vertex, ShaderKind::Vertex, &options.defines),
    };
    output.v330 = GlslOutput {
        fragment: glsl_v330(fragment, ShaderKind::Fragment, &options.defines),
        vertex: glsl_v330(vertex, ShaderKind::Vertex, &options.defines),
    };
    output.v300es = GlslOutput {
        fragment: glsl_v300es(fragment, ShaderKind::Fragment, &options.defines),
        vertex: glsl_v300es(vertex, ShaderKind::Vertex, &options.defines),
    };
    output.metal = metal::metal(fragment, vertex, meta, &options);
    Ok(output)
}

pub fn choose_appropriate_shader<'a>(
    shader: &'a Output,
    context_info: &miniquad::ContextInfo,
) -> miniquad::ShaderSource<'a> {
    use miniquad::{Backend, ShaderSource};

    match context_info.backend {
        Backend::OpenGl => {
            if context_info.glsl_support.v300es {
                ShaderSource::Glsl {
                    vertex: &shader.v300es.vertex,
                    fragment: &shader.v300es.fragment,
                }
            } else if context_info.glsl_support.v330 {
                ShaderSource::Glsl {
                    vertex: &shader.v330.vertex,
                    fragment: &shader.v330.fragment,
                }
            } else if context_info.glsl_support.v130 {
                ShaderSource::Glsl {
                    vertex: &shader.v130.vertex,
                    fragment: &shader.v130.fragment,
                }
            } else if context_info.glsl_support.v100_ext {
                ShaderSource::Glsl {
                    vertex: &shader.v100_webgl.vertex,
                    fragment: &shader.v100_webgl.fragment,
                }
            } else {
                ShaderSource::Glsl {
                    vertex: &shader.v100.vertex,
                    fragment: &shader.v100.fragment,
                }
            }
        }
        Backend::Metal => ShaderSource::Msl {
            program: &shader.metal,
        },
    }
}
