struct Color {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> base_color: Color;
@group(1) @binding(1)
var<uniform> ink_color: Color;
@group(1) @binding(2)
var symbol_texture: texture_2d<f32>;
@group(1) @binding(3)
var symbol_sampler: sampler;
@group(1) @binding(4)
var overlay_texture: texture_2d<f32>;
@group(1) @binding(5)
var overlay_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let vertex_color = color;
    let symbol_color = textureSample(symbol_texture, symbol_sampler, uv);
    let overlay_color = textureSample(overlay_texture, overlay_sampler, (world_position.xz / 30.) - 0.5);

    let init_bg = mix(vertex_color * overlay_color, overlay_color, 0.4);
    let bg = mix(init_bg, init_bg * base_color.color , 0.7);
    let ink = mix(ink_color.color,vec4<f32>(1., 1., 1., 1.), symbol_color);
    return ink * bg;
}