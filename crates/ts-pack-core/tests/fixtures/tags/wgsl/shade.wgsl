let PI: f32 = 3.14159;

struct Light {
    dir: vec3<f32>,
    power: f32,
}

type Color = vec3<f32>;

@group(0) @binding(0) var<uniform> sun: Light;

override scale: f32 = 1.0;

fn lambert(n: vec3<f32>, l: Light) -> f32 {
    let d = max(dot(n, l.dir), 0.0);
    return d * l.power;
}

@fragment
fn main(@location(0) n: vec3<f32>) -> @location(0) vec4<f32> {
    var c: Color = vec3<f32>(lambert(n, sun));
    return vec4<f32>(c, 1.0);
}
