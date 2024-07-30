use macroquad::{
    math::vec2,
    quad_gl::{
        camera::Environment,
        color::{self, Color},
    },
    window::next_frame,
};

mod orbit_camera;

async fn game(ctx: macroquad::Context) {
    let mut scene = ctx.new_scene();
    let mut helmet = ctx
        .resources
        .load_gltf("assets/DamagedHelmet.gltf")
        .await
        .unwrap();
    let helmet = scene.add_model(&helmet);
    let skybox = ctx
        .resources
        .load_cubemap(
            "assets/skybox/skybox_px.png",
            "assets/skybox/skybox_nx.png",
            "assets/skybox/skybox_py.png",
            "assets/skybox/skybox_ny.png",
            "assets/skybox/skybox_pz.png",
            "assets/skybox/skybox_nz.png",
        )
        .await
        .unwrap();
    let mut orbit = orbit_camera::OrbitCamera::new();
    orbit.camera.environment = Environment::Skybox(skybox);
    let mut canvas = ctx.new_canvas();
    loop {
        ctx.clear_screen(color::WHITE);
        canvas.clear();
        orbit.orbit(&ctx);
        scene.draw(&orbit.camera);
        canvas.draw();
        next_frame().await
    }
}

fn main() {
    macroquad::start(Default::default(), |ctx| game(ctx));
}
