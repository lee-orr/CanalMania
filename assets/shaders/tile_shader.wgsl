struct Color {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> base_color: Color;
@group(1) @binding(1)
var<uniform> ink_color: Color;

#import noisy_bevy::prelude

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var parchment_base : vec4<f32> = vec4<f32>(0.775822, 0.637597, 0.351533, 1.);
    var parchment_burn : vec4<f32> = vec4<f32>(0.423268, 0.246201, 0.076185, 1.);
    var parchment_dark : vec4<f32> = vec4<f32>(0.22, 0.1, 0.1, 1.);

    let vertex_color = color;


    var test_position : vec3<f32> = world_position.xyz * 0.3;
    var overlay_1: f32 = simplex_noise_3d(test_position);

    for(var i: f32 = 1.; i < 5.; i += 1.) {
        let position = test_position * 2. * i;
        let val = simplex_noise_3d(position);
        overlay_1 = mix(overlay_1, val, 0.2);
    }

    test_position = world_position.xyz * 1.5;
    var overlay_2: f32 = simplex_noise_3d(test_position);
    for(var i: f32 = 1.; i < 6.; i += 1.) {
        let position = test_position * 3. * i;
        let val = simplex_noise_3d(position);
        overlay_2 = mix(overlay_2, val, 0.4);
    }

    test_position = world_position.xyz * 1.5;
    var overlay_3: f32 = simplex_noise_3d(test_position);
    for(var i: f32 = 1.; i < 8.; i += 1.) {
        let position = test_position * 3. * i;
        let val = simplex_noise_3d(position);
        overlay_3 = mix(overlay_3, val, 0.8);
    }

    let overlay_mixer = mix(mix(overlay_1, overlay_2, 0.3), overlay_3, 0.3);

    let overlay_color = mix(parchment_base, parchment_burn, overlay_mixer);

    let init_bg = mix(vertex_color * overlay_color, overlay_color, 0.3);
    let bg = mix(init_bg, init_bg * base_color.color , 0.3);
//    let ink = mix(ink_color.color,vec4<f32>(1., 1., 1., 1.), symbol_color);
    let depth = clamp(mix(-0.3, 1.2, clamp(world_position.y + 1., 0., 1.)), 0., 1.);
    let color = mix(bg , bg  * parchment_dark, 1. - depth);
    return color;
}