// //! 3D shapes and models, loading 3d models from files, drawing 3D primitives.

use crate::{
    math::{vec2, vec3, Vec2, Vec3},
    scene::{self, Model, Node, NodeData, Scene, Transform},
    texture::Texture2D,
};
use miniquad::*;

pub struct CpuMesh(pub Vec<Vec3>, pub Vec<Vec2>, pub Vec<Vec3>, pub Vec<u16>);

pub fn sphere(radius: f32, rings: u32, slices: u32) -> CpuMesh {
    let scale = vec3(radius, radius, radius);
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];

    // (i, j + 1)   (i + 1, j + 1)
    //
    // (i, j)       (i + 1, j)
    //
    // i      j
    // i + 1, j + 1
    // i + 1, j
    //
    // i,     j
    // i    , j + 1
    // i + 1, j + 1

    let mut geometry = |v: &[_], i| {
        for ix in i {
            indices.push((vertices.len() + ix) as u16);
        }
        for &(v, uv, n) in v {
            vertices.push(v);
            uvs.push(uv);
            normals.push(n);
        }
    };
    for i in 0..rings + 1 {
        for j in 0..slices + 1 {
            use std::f32::consts::PI;

            let pi34 = PI / 2. * 3.;
            let pi2 = PI * 2.;
            let i = i as f32;
            let j = j as f32;
            let rings: f32 = rings as _;
            let slices: f32 = slices as _;

            let v1 = vec3(
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * i).sin(),
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
            );
            let uv1 = vec2(i / rings, j / slices);
            let v2 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv2 = vec2((i + 1.) / rings, (j + 1.) / slices);
            let v3 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).cos(),
            );
            let uv3 = vec2((i + 1.) / rings, j / slices);

            geometry(
                &[
                    ((v1 * scale), uv1, v1.normalize()),
                    ((v2 * scale), uv2, v2.normalize()),
                    ((v3 * scale), uv3, v3.normalize()),
                ],
                &[0, 1, 2],
            );

            let v1 = vec3(
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * i).sin(),
                (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
            );
            let uv1 = vec2(i / rings, j / slices);
            let v2 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv2 = vec2(i / rings, (j + 1.) / slices);
            let v3 = vec3(
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
                (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
            );
            let uv3 = vec2((i + 1.) / rings, (j + 1.) / slices);

            geometry(
                &[
                    ((v1 * scale), uv1, v1.normalize()),
                    ((v2 * scale), uv2, v2.normalize()),
                    ((v3 * scale), uv3, v3.normalize()),
                ],
                &[0, 1, 2],
            );
        }
    }

    CpuMesh(vertices, uvs, normals, indices)
}

pub fn square() -> CpuMesh {
    let width = 1.0;
    let length = 1.0;
    let indices = vec![0u16, 1, 2, 0, 2, 3];

    let vertices = vec![
        vec3(-width / 2., 0., -length / 2.),
        vec3(-width / 2., 0., length / 2.),
        vec3(width / 2., 0., length / 2.),
        vec3(width / 2., 0., -length / 2.),
    ];
    let uvs = vec![vec2(0., 1.), vec2(0., 0.), vec2(1., 0.), vec2(1., 1.)];
    let normals = vec![
        vec3(0., 1., 0.),
        vec3(0., 1., 0.),
        vec3(0., 1., 0.),
        vec3(0., 1., 0.),
    ];

    CpuMesh(vertices, uvs, normals, indices)
}

impl crate::QuadGl {
    pub fn mesh(
        &self,
        CpuMesh(vertices, uvs, normals, indices): CpuMesh,
        texture: Option<Texture2D>,
    ) -> Model {
        let mut quad_ctx = self.quad_ctx.lock().unwrap();

        let vertex_buffer = quad_ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );
        let normals_buffer = quad_ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&normals),
        );
        let uvs_buffer = quad_ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&uvs),
        );
        let index_buffer = quad_ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );
        let shader = shadermagic::transform(
            crate::scene::shader::FRAGMENT,
            crate::scene::shader::VERTEX,
            &crate::scene::shader::meta(),
            &shadermagic::Options {
                defines: vec![],
                ..Default::default()
            },
        )
        .unwrap();
        let shader = shadermagic::choose_appropriate_shader(&shader, &quad_ctx.info());
        if let miniquad::ShaderSource::Glsl { fragment, vertex } = shader {
            //miniquad::warn!("{}", fragment);
        };
        let shader = quad_ctx
            .new_shader(shader, scene::shader::meta())
            .unwrap_or_else(|e| panic!("Failed to load shader: {}", e));

        let pipeline = quad_ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout::default(),
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_position", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_uv", VertexFormat::Float2, 1),
                VertexAttribute::with_buffer("in_normal", VertexFormat::Float3, 2),
                VertexAttribute::with_buffer("in_inst", VertexFormat::Float3, 3),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );

        let instancing = vec![vec3(0.0, 0.0, 0.0)];
        let instancing_buffer =
            quad_ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, unsafe {
                BufferSource::slice(&instancing[..])
            });

        let data = NodeData {
            vertex_buffers: vec![vertex_buffer, uvs_buffer, normals_buffer, instancing_buffer],
            index_buffer,

        };
        let material = scene::Material2 {
            color: [1.0, 1.0, 1.0, 1.0],
            base_color_texture: texture,
            emissive_texture: None,
            normal_texture: None,
            occlusion_texture: None,
            metallic_roughness_texture: None,
            metallic: 0.01,
            roughness: 0.8,
            shader: scene::Shader::default(quad_ctx.as_mut()),
        };

        let mut aabb = crate::scene::AABB {
            min: vec3(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            max: vec3(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX),
        };
        for vertex in &vertices {
            aabb.min = aabb.min.min(*vertex);
            aabb.max = aabb.max.max(*vertex);
        }
        Model {
            nodes: vec![Node {
                name: "root".to_string(),
                data: vec![data],
                materials: vec![material],
                transform: Transform::default(),
            }],
            aabb,
        }
    }
}

// use crate::{color::Color, get_context};

// use crate::{quad_gl::DrawMode, texture::Texture2D};
// use glam::{vec2, vec3, Vec2, Vec3};

// #[derive(Clone, Debug, Copy)]
// pub struct Vertex {
//     pub position: Vec3,
//     pub uv: Vec2,
//     pub color: Color,
// }

// impl From<Vertex> for crate::quad_gl::VertexInterop {
//     fn from(vertex: Vertex) -> crate::quad_gl::VertexInterop {
//         (
//             vertex.position.into(),
//             vertex.uv.into(),
//             vertex.color.into(),
//         )
//     }
// }

// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u16>,
//     pub texture: Option<Texture2D>,
// }

// pub fn draw_mesh(mesh: &Mesh) {
//     let context = get_context();

//     context.gl.texture(mesh.texture.as_ref());
//     context.gl.draw_mode(DrawMode::Triangles);
//     context.gl.geometry(&mesh.vertices[..], &mesh.indices[..]);
// }

// fn draw_quad(vertices: [(Vec3, Vec2, Color); 4]) {
//     let context = get_context();
//     let indices = [0, 1, 2, 0, 2, 3];
//     let quad = [
//         (
//             [vertices[0].0.x, vertices[0].0.y, vertices[0].0.z],
//             [vertices[0].1.x, vertices[0].1.y],
//             vertices[0].2.into(),
//         ),
//         (
//             [vertices[1].0.x, vertices[1].0.y, vertices[1].0.z],
//             [vertices[1].1.x, vertices[1].1.y],
//             vertices[1].2.into(),
//         ),
//         (
//             [vertices[2].0.x, vertices[2].0.y, vertices[2].0.z],
//             [vertices[2].1.x, vertices[2].1.y],
//             vertices[2].2.into(),
//         ),
//         (
//             [vertices[3].0.x, vertices[3].0.y, vertices[3].0.z],
//             [vertices[3].1.x, vertices[3].1.y],
//             vertices[3].2.into(),
//         ),
//     ];

//     context.gl.draw_mode(DrawMode::Triangles);
//     context.gl.geometry(&quad[..], &indices);
// }

// pub fn draw_line_3d(start: Vec3, end: Vec3, color: Color) {
//     let context = get_context();
//     let uv = [0., 0.];
//     let color: [f32; 4] = color.into();
//     let indices = [0, 1];

//     let line = [
//         ([start.x, start.y, start.z], uv, color),
//         ([end.x, end.y, end.z], uv, color),
//     ];
//     context.gl.texture(None);
//     context.gl.draw_mode(DrawMode::Lines);
//     context.gl.geometry(&line[..], &indices);
// }

// /// Draw a grid centered at (0, 0, 0)
// pub fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
//     let half_slices = (slices as i32) / 2;
//     for i in -half_slices..half_slices + 1 {
//         let color = if i == 0 { axes_color } else { other_color };

//         draw_line_3d(
//             vec3(i as f32 * spacing, 0., -half_slices as f32 * spacing),
//             vec3(i as f32 * spacing, 0., half_slices as f32 * spacing),
//             color,
//         );
//         draw_line_3d(
//             vec3(-half_slices as f32 * spacing, 0., i as f32 * spacing),
//             vec3(half_slices as f32 * spacing, 0., i as f32 * spacing),
//             color,
//         );
//     }
// }

// pub fn draw_plane(center: Vec3, size: Vec2, texture: Option<&Texture2D>, color: Color) {
//     let v1 = (
//         (center + vec3(-size.x, 0., -size.y)).into(),
//         vec2(0., 0.),
//         color,
//     );
//     let v2 = (
//         (center + vec3(-size.x, 0., size.y)).into(),
//         vec2(0., 1.),
//         color,
//     );
//     let v3 = (
//         (center + vec3(size.x, 0., size.y)).into(),
//         vec2(1., 1.),
//         color,
//     );
//     let v4 = (
//         (center + vec3(size.x, 0., -size.y)).into(),
//         vec2(1., 0.),
//         color,
//     );

//     {
//         let context = get_context();
//         context.gl.texture(texture);
//     }
//     draw_quad([v1, v2, v3, v4]);
// }

// /// Draw an affine (2D) parallelogram at given position, as two triangles.
// ///
// /// The drawn parallelogram will have the vertices: `offset`, `offset + e1`, `offset + e2` and `offset + e1 + e2`
// ///
// /// # Arguments
// ///
// /// * `offset` - Offset of the first point from the origin
// /// * `e1`, `e2` - Base vectors for the parallelogram
// /// * `texture` - Optional [Texture2D] to apply, which will be streched on the entire shape (todo!
// /// support custom uv values per vertex)
// /// * `color` - The [Color] to draw the parallelogram
// ///
// /// # Examples
// ///
// /// Draw an axis aligned rectangle
// /// ```no_run
// /// # use macroquad::prelude::*;
// /// draw_affine_parallelogram(Vec3::ZERO, 3. * Vec3::X, 5. * Vec3::Z, None, RED);
// /// ```
// pub fn draw_affine_parallelogram(
//     offset: Vec3,
//     e1: Vec3,
//     e2: Vec3,
//     texture: Option<&Texture2D>,
//     color: Color,
// ) {
//     let v1 = (offset.into(), vec2(0., 0.), color);
//     let v2 = ((offset + e1).into(), vec2(0., 1.), color);
//     let v3 = ((offset + e1 + e2).into(), vec2(1., 1.), color);
//     let v4 = ((offset + e2).into(), vec2(1., 0.), color);

//     {
//         let context = get_context();
//         context.gl.texture(texture);
//     }
//     draw_quad([v1, v2, v3, v4]);
// }

// /// Draw an affine (3D) parallelepiped at given position, using six parallelograms.
// ///
// /// The drawn parallelepiped will be built from the followwing parallelograms:
// ///
// /// * `offset, offset + e1, offset + e2`
// /// * `offset, offset + e2, offset + e3`
// /// * `offset, offset + e1, offset + e3`
// /// * `offset, offset + e1 + e2, offset + e1 + e3`
// /// * `offset, offset + e2 + e1, offset + e2 + e3`
// /// * `offset, offset + e3 + e1, offset + e3 + e2`
// ///
// /// # Arguments
// ///
// /// * `offset` - Offset of the first point from the origin
// /// * `e1`, `e2`, `e3` - Base vectors for the parallelepiped
// /// * `texture` - Optional [Texture2D] to apply, which will repeat on each face (todo!
// /// support custom uv values per vertex, multiple textures?)
// /// * `color` - The [Color] to draw the parallelepiped (todo! support color per face?)
// ///
// /// # Examples
// ///
// /// Draw an axis aligned cube
// /// ```no_run
// /// # use macroquad::prelude::*;
// /// draw_affine_parallelepiped(Vec3::ZERO, 3. * Vec3::X, 2. * Vec3::Y, 5. * Vec3::Z, None, RED);
// /// ```
// pub fn draw_affine_parallelepiped(
//     offset: Vec3,
//     e1: Vec3,
//     e2: Vec3,
//     e3: Vec3,
//     texture: Option<&Texture2D>,
//     color: Color,
// ) {
//     let texture_base = texture.into();
//     draw_affine_parallelogram(offset, e1, e2, texture_base, color);
//     draw_affine_parallelogram(offset, e1, e3, texture_base, color);
//     draw_affine_parallelogram(offset, e2, e3, texture_base, color);

//     draw_affine_parallelogram(offset + e1, e2, e3, texture_base, color);
//     draw_affine_parallelogram(offset + e2, e1, e3, texture_base, color);
//     draw_affine_parallelogram(offset + e3, e1, e2, texture_base, color);
// }

// pub fn draw_cube(position: Vec3, size: Vec3, texture: Option<&Texture2D>, color: Color) {
//     let context = get_context();
//     context.gl.texture(texture.into());

//     let (x, y, z) = (position.x, position.y, position.z);
//     let (width, height, length) = (size.x, size.y, size.z);

//     // Front face
//     let bl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
//     let bl_uv = vec2(0., 0.);
//     let br_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
//     let br_uv = vec2(1., 0.);

//     let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
//     let tr_uv = vec2(1., 1.);

//     let tl_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
//     let tl_uv = vec2(0., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);

//     // Back face
//     let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
//     let bl_uv = vec2(0., 0.);
//     let br_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
//     let br_uv = vec2(1., 0.);

//     let tr_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
//     let tr_uv = vec2(1., 1.);

//     let tl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
//     let tl_uv = vec2(0., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);

//     // Top face
//     let bl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
//     let bl_uv = vec2(0., 1.);
//     let br_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
//     let br_uv = vec2(0., 0.);

//     let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
//     let tr_uv = vec2(1., 0.);

//     let tl_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
//     let tl_uv = vec2(1., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);

//     // Bottom face
//     let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
//     let bl_uv = vec2(0., 1.);
//     let br_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
//     let br_uv = vec2(0., 0.);

//     let tr_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
//     let tr_uv = vec2(1., 0.);

//     let tl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
//     let tl_uv = vec2(1., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);

//     // Right face
//     let bl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
//     let bl_uv = vec2(0., 1.);
//     let br_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
//     let br_uv = vec2(0., 0.);

//     let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
//     let tr_uv = vec2(1., 0.);

//     let tl_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
//     let tl_uv = vec2(1., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);

//     // Left face
//     let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
//     let bl_uv = vec2(0., 1.);
//     let br_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
//     let br_uv = vec2(0., 0.);

//     let tr_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
//     let tr_uv = vec2(1., 0.);

//     let tl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
//     let tl_uv = vec2(1., 1.);

//     draw_quad([
//         (bl_pos, bl_uv, color),
//         (br_pos, br_uv, color),
//         (tr_pos, tr_uv, color),
//         (tl_pos, tl_uv, color),
//     ]);
// }

// pub fn draw_cube_wires(position: Vec3, size: Vec3, color: Color) {
//     let (x, y, z) = (position.x, position.y, position.z);
//     let (width, height, length) = (size.x, size.y, size.z);

//     // Front Face

//     // Bottom Line
//     draw_line_3d(
//         vec3(x - width / 2., y - height / 2., z + length / 2.),
//         vec3(x + width / 2., y - height / 2., z + length / 2.),
//         color,
//     );

//     // Left Line
//     draw_line_3d(
//         vec3(x + width / 2., y - height / 2., z + length / 2.),
//         vec3(x + width / 2., y + height / 2., z + length / 2.),
//         color,
//     );

//     // Top Line
//     draw_line_3d(
//         vec3(x + width / 2., y + height / 2., z + length / 2.),
//         vec3(x - width / 2., y + height / 2., z + length / 2.),
//         color,
//     );

//     // Right Line
//     draw_line_3d(
//         vec3(x - width / 2., y + height / 2., z + length / 2.),
//         vec3(x - width / 2., y - height / 2., z + length / 2.),
//         color,
//     );

//     // Back Face
//     // Bottom Line
//     draw_line_3d(
//         vec3(x - width / 2., y - height / 2., z - length / 2.),
//         vec3(x + width / 2., y - height / 2., z - length / 2.),
//         color,
//     );

//     // Left Line
//     draw_line_3d(
//         vec3(x + width / 2., y - height / 2., z - length / 2.),
//         vec3(x + width / 2., y + height / 2., z - length / 2.),
//         color,
//     );

//     // Top Line
//     draw_line_3d(
//         vec3(x + width / 2., y + height / 2., z - length / 2.),
//         vec3(x - width / 2., y + height / 2., z - length / 2.),
//         color,
//     );

//     // Right Line
//     draw_line_3d(
//         vec3(x - width / 2., y + height / 2., z - length / 2.),
//         vec3(x - width / 2., y - height / 2., z - length / 2.),
//         color,
//     );

//     // Top Face
//     // Left Line
//     draw_line_3d(
//         vec3(x - width / 2., y + height / 2., z + length / 2.),
//         vec3(x - width / 2., y + height / 2., z - length / 2.),
//         color,
//     );

//     // Right Line
//     draw_line_3d(
//         vec3(x + width / 2., y + height / 2., z + length / 2.),
//         vec3(x + width / 2., y + height / 2., z - length / 2.),
//         color,
//     );

//     // Bottom Face
//     // Left Line
//     draw_line_3d(
//         vec3(x - width / 2., y - height / 2., z + length / 2.),
//         vec3(x - width / 2., y - height / 2., z - length / 2.),
//         color,
//     );

//     // Right Line
//     draw_line_3d(
//         vec3(x + width / 2., y - height / 2., z + length / 2.),
//         vec3(x + width / 2., y - height / 2., z - length / 2.),
//         color,
//     );
// }

// #[derive(Debug, Clone)]
// pub struct DrawSphereParams {
//     pub rings: usize,
//     pub slices: usize,
//     pub draw_mode: DrawMode,
// }

// impl Default for DrawSphereParams {
//     fn default() -> DrawSphereParams {
//         DrawSphereParams {
//             rings: 16,
//             slices: 16,
//             draw_mode: DrawMode::Triangles,
//         }
//     }
// }

// pub fn draw_sphere(center: Vec3, radius: f32, texture: Option<&Texture2D>, color: Color) {
//     draw_sphere_ex(center, radius, texture, color, Default::default());
// }

// pub fn draw_sphere_wires(center: Vec3, radius: f32, texture: Option<&Texture2D>, color: Color) {
//     let params = DrawSphereParams {
//         draw_mode: DrawMode::Lines,
//         ..Default::default()
//     };
//     draw_sphere_ex(center, radius, texture, color, params);
// }

// pub fn draw_sphere_ex(
//     center: Vec3,
//     radius: f32,
//     texture: Option<&Texture2D>,
//     color: Color,
//     params: DrawSphereParams,
// ) {
//     let context = get_context();

//     let rings = params.rings;
//     let slices = params.slices;

//     let color: [f32; 4] = color.into();
//     let scale = vec3(radius, radius, radius);

//     context.gl.texture(texture.into());
//     context.gl.draw_mode(params.draw_mode);

//     for i in 0..rings + 1 {
//         for j in 0..slices {
//             use std::f32::consts::PI;

//             let pi34 = PI / 2. * 3.;
//             let pi2 = PI * 2.;
//             let i = i as f32;
//             let j = j as f32;
//             let rings: f32 = rings as _;
//             let slices: f32 = slices as _;

//             let v1 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * i).sin(),
//                 (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
//             );
//             let uv1 = [i / rings, j / slices];
//             let v2 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
//             );
//             let uv2 = [(i + 1.) / rings, (j + 1.) / slices];
//             let v3 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * (j * pi2 / slices).cos(),
//             );
//             let uv3 = [(i + 1.) / rings, j / slices];

//             context.gl.geometry(
//                 &[
//                     ((v1 * scale + center).into(), uv1, color),
//                     ((v2 * scale + center).into(), uv2, color),
//                     ((v3 * scale + center).into(), uv3, color),
//                 ],
//                 &[0, 1, 2],
//             );

//             let v1 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * i).sin(),
//                 (pi34 + (PI / (rings + 1.)) * i).cos() * (j * pi2 / slices).cos(),
//             );
//             let uv1 = [i / rings, j / slices];
//             let v2 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i)).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i)).cos() * ((j + 1.) * pi2 / slices).cos(),
//             );
//             let uv2 = [i / rings, (j + 1.) / slices];
//             let v3 = vec3(
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).sin(),
//                 (pi34 + (PI / (rings + 1.)) * (i + 1.)).cos() * ((j + 1.) * pi2 / slices).cos(),
//             );
//             let uv3 = [(i + 1.) / rings, (j + 1.) / slices];

//             context.gl.geometry(
//                 &[
//                     ((v1 * scale + center).into(), uv1, color),
//                     ((v2 * scale + center).into(), uv2, color),
//                     ((v3 * scale + center).into(), uv3, color),
//                 ],
//                 &[0, 1, 2],
//             );
//         }
//     }
// }
