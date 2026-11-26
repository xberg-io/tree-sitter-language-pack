#define_import_path my_shaders::lighting

#import bevy_pbr::mesh_view_bindings::view

let PI: f32 = 3.14159;

struct Light {
    dir: vec3<f32>,
    power: f32,
};

type Color = vec3<f32>;

override scale: f32 = 1.0;

@group(1) @binding(0) var<uniform> sun: Light;

fn lambert(n: vec3<f32>, l: Light) -> f32 {
    let d = max(dot(n, l.dir), 0.0);
    return d * l.power;
}

@fragment
fn fragment(@location(0) n: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(vec3<f32>(lambert(n, sun)), 1.0);
}
