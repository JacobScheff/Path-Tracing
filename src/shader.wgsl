// Vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    return output;
}

// Fragment shader
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Uniforms {
    hello_world: u32,
};

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate color based on hello_world
    let color = vec4<f32>(
        f32(uniforms.hello_world) / 255.0,
        f32(uniforms.hello_world) / 255.0,
        f32(uniforms.hello_world) / 255.0,
        1.0,
    );
    return color;
}