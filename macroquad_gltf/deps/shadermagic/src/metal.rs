use std::collections::HashMap;

fn replace_types(l: &str) -> String {
    l.replace("float", "float")
        .replace("vec2", "float2")
        .replace("vec3", "float3")
        .replace("vec4", "float4")
        .replace("mat3", "float3x3")
        .replace("mat4", "float4x4")
}
fn replace_functions(l: &str) -> String {
    l.replace("dFdx", "dfdx").replace("dFdy", "dfdy")
}
fn eat_string(line: &mut String, l: &str) {
    *line = line.trim().strip_prefix(l).unwrap().to_string()
}
fn get_i32(line: &mut String) -> i32 {
    let mut l = line
        .trim()
        .split(|c: char| c.is_whitespace() || c == '(' || c == ')');
    let res = l.next().unwrap().to_string();
    *line = line.trim().strip_prefix(&res).unwrap().to_string();
    res.parse::<i32>().unwrap()
}
fn get_string(line: &mut String) -> String {
    let mut l = line
        .trim()
        .split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ';');
    let res = l.next().unwrap().to_string();
    *line = line.trim().strip_prefix(&res).unwrap().to_string();
    res
}
fn emit_uniforms_struct(processed: &mut String, meta: &miniquad::ShaderMeta) {
    processed.push_str("struct Uniforms {\n");
    for uniform in &meta.uniforms.uniforms {
        use miniquad::UniformType::*;

        let type_ = match uniform.uniform_type {
            Float1 => "float",
            Float2 => "float2",
            Float3 => "float3",
            Float4 => "float4",
            Int1 => "int",
            Int2 => "int2",
            Int3 => "int3",
            Int4 => "int4",
            Mat4 => "float4x4",
        };
        processed.push_str(&format!("    {} {};\n", type_, uniform.name));
    }
    processed.push_str("};\n");
}
fn emit_vertex_struct(processed: &mut String, vertex: &str) -> Vec<(String, String)> {
    let mut attributes = vec![];
    processed.push_str("struct Vertex {\n");
    for attribute in vertex.lines().filter(|l| l.contains("attribute")) {
        let attribute = replace_types(attribute).replace(";", "").trim().to_string();
        let mut attribute = attribute.split(' ');
        let type_ = attribute.nth(1).unwrap();
        let name = attribute.nth(0).unwrap();
        let loc = attribute.nth(1).unwrap();
        attributes.push((name.to_string(), type_.to_string()));
        processed.push_str(&format!("    {} {} {};\n", type_, name, loc));
    }
    processed.push_str("};\n");
    attributes
}

fn emit_rasterizer_data_struct(processed: &mut String, vertex: &str) -> Vec<String> {
    processed.push_str("struct RasterizerData {\n");
    processed.push_str(&"    float4 position [[position]];\n");

    let mut outs = vec![];
    for varying in vertex.lines().filter(|l| l.contains("varying")) {
        let varying = replace_types(varying).replace(";", "").trim().to_string();
        let mut varying = varying.split(' ');
        let type_ = varying.nth(1).unwrap();
        let name = varying.nth(0).unwrap();
        let loc = varying.nth(1).unwrap();
        outs.push(name.to_string());
        processed.push_str(&format!("    {} {} {};\n", type_, name, loc));
    }
    processed.push_str("};\n");
    outs
}
fn collect_texture_types(fragment: &str) -> HashMap<String, String> {
    let mut res = HashMap::new();
    for uniform in fragment.lines().filter(|l| l.contains("uniform sampler")) {
        let mut uniform = uniform.trim().split(" ");
        let type_ = uniform.nth(1).unwrap();
        let name = uniform.nth(0).unwrap().replace(";", "");

        res.insert(name.to_string(), type_.to_string());
    }
    res
}

fn count_braces(line: &str, brace: char) -> i32 {
    line.chars().filter(|c| *c == brace).count() as i32
}
pub fn metal(
    fragment: &str,
    vertex: &str,
    meta: &miniquad::ShaderMeta,
    options: &crate::Options,
) -> String {
    let mut processed = String::new();

    processed.push_str("#include <metal_stdlib>\n");
    processed.push_str("using namespace metal;\n");
    processed.push_str("#define __METAL 1\n");
    for define in &options.defines {
        processed.push_str(&format!("#define {} 1\n", define));
    }
    processed.push_str(
        "float3x3 sm_to_m3(float3 v0, float3 v1, float3 v2) {return float3x3(v0, v1, v2);}\n",
    );
    processed.push_str(
        "float3x3 sm_to_m3(float4x4 m) {return float3x3(m[0].xyz, m[1].xyz, m[2].xyz);}\n",
    );
    processed.push_str("#define sm_level(x) level(x)\n");

    emit_uniforms_struct(&mut processed, meta);
    let attributes = emit_vertex_struct(&mut processed, vertex);
    let outs = emit_rasterizer_data_struct(&mut processed, vertex);

    let mut in_main = false;
    let mut main_curly_braces: i32 = 0;
    for line in vertex.lines() {
        if line.contains("uniform") || line.contains("attribute") || line.contains("varying") {
            continue;
        }
        if line.contains("void main()") {
            in_main = true;
            main_curly_braces = count_braces(line, '{');
            processed.push_str("vertex RasterizerData vertexShader(\n");
            processed.push_str("    Vertex v [[stage_in]],\n");
            processed.push_str("    constant Uniforms& uniforms [[buffer(0)]]\n");
            processed.push_str(") {\n");
            processed.push_str("    RasterizerData msl_vertex_out;\n");
            continue;
        }

        let mut line = line.replace("mat3(", "sm_to_m3(");
        line = replace_types(&line).trim().to_string();
        if in_main {
            main_curly_braces += count_braces(&line, '{');
            main_curly_braces -= count_braces(&line, '}');
            line = line.replace("gl_Position", "msl_vertex_out.position");
            for (attribute, _) in &attributes {
                line = line.replace(&*attribute, &format!("v.{}", attribute));
                line = line.replace("v.v.", "v.");
            }
            for uniform in &meta.uniforms.uniforms {
                line = line.replace(&uniform.name, &format!("uniforms.{}", uniform.name));
                line = line.replace("uniforms.uniforms.", "uniforms.");
            }
            for out in &outs {
                line = line.replace(&*out, &format!("msl_vertex_out.{}", out));
                line = line.replace("msl_vertex_out.msl_vertex_out.", "msl_vertex_out.");
            }
            if main_curly_braces == 0 {
                if options.metal_flip_y {
                    processed.push_str("msl_vertex_out.position.y = -msl_vertex_out.position.y;\n");
                }
                processed.push_str("return msl_vertex_out;\n");
                in_main = false;
            }
        }

        processed.push_str(&line);
        processed.push_str("\n");
    }

    let mut in_main = false;
    let mut mrt = false;
    let mut mrt_targets = vec![];
    let sampler_types = collect_texture_types(fragment);
    processed.push_str("float2 textureSize(texture2d<float> t, int x) {return float2(t.get_width(), t.get_height());}\n");
    for line in fragment.lines() {
        if line.contains("uniform") || line.contains("attribute") || line.contains("varying") {
            continue;
        }
        if line.contains("layout") && line.contains("location") && line.contains("out") {
            let mut line = line.to_string();
            eat_string(&mut line, "layout");
            eat_string(&mut line, "(");
            eat_string(&mut line, "location");
            eat_string(&mut line, "=");
            let location = get_i32(&mut line);
            eat_string(&mut line, ")");
            eat_string(&mut line, "out");
            eat_string(&mut line, "vec4");
            let name = get_string(&mut line);
            mrt_targets.push((location, name));
            mrt = true;
            continue;
        }
        if line.contains("void main()") {
            in_main = true;
            if mrt {
                processed.push_str("struct FragmentOutput {\n");
                for (n, name) in &mrt_targets {
                    processed.push_str(&format!("    float4 {name} [[color({n})]];\n"));
                }
                processed.push_str("};\n");
            }
            let return_type = if mrt { "FragmentOutput" } else { "float4" };
            main_curly_braces = count_braces(line, '{');
            processed.push_str(&format!("fragment {return_type} fragmentShader(\n"));
            processed.push_str("    RasterizerData in[[stage_in]],\n");
            processed.push_str("    constant Uniforms& uniforms [[buffer(0)]],\n");
            for (n, image) in meta.images.iter().enumerate() {
                let type_ = match sampler_types.get(image).as_ref().map(|x| x.as_str()) {
                    None => "texture2d",
                    Some("sampler2D") => "texture2d",
                    Some("samplerCube") => "texturecube",
                    _ => unimplemented!(),
                };
                processed.push_str(&format!(
                    "    {}<float> {} [[texture({})]],\n",
                    type_, image, n
                ));
                processed.push_str(&format!("    sampler {}Smplr [[sampler({})]]", image, n));
                if n != meta.images.len() - 1 {
                    processed.push_str(",\n")
                }
            }
            processed.push_str(") {\n");
            processed.push_str(&format!("    {return_type} msl_out_color;\n"));
            continue;
        }

        let mut line = line.replace("mat3(", "sm_to_m3(");
        line = replace_types(&line);
        line = replace_functions(&line);
        if in_main {
            main_curly_braces += count_braces(&line, '{');
            main_curly_braces -= count_braces(&line, '}');
            line = line.replace("gl_FragColor", "msl_out_color");
            for (_, target) in &mrt_targets {
                line = line.replace(target, &format!("msl_out_color.{target}"));
            }
            for (attribute, _) in &attributes {
                line = line.replace(&*attribute, &format!("v.{}", attribute));
                line = line.replace("v.v.", "v.");
            }
            for uniform in &meta.uniforms.uniforms {
                line = line.replace(&uniform.name, &format!("uniforms.{}", uniform.name));
                line = line.replace("uniforms.uniforms.", "uniforms.");
            }
            for out in &outs {
                line = line.replace(&*out, &format!("in.{}", out));
                line = line.replace("in.in.", "in.");
            }
            for image in &meta.images {
                line = line.replace(
                    &format!("texture2D({}", image),
                    &format!("{}.sample({}Smplr", image, image),
                );
                line = line.replace(
                    &format!("textureCube({}", image),
                    &format!("{}.sample({}Smplr", image, image),
                );
                line = line.replace(
                    &format!("textureCubeLod({}", image),
                    &format!("{}.sample({}Smplr", image, image),
                );
            }
            if main_curly_braces == 0 {
                processed.push_str("return msl_out_color;\n");
                in_main = false;
            }
        }

        processed.push_str(&line);
        processed.push_str("\n");
    }

    processed
}
