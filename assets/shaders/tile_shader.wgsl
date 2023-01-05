struct Color {
    color: vec4<f32>
};

struct Value {
    value: vec4<f32>
};

struct InkSettings {
    base_color: vec4<f32>,
    ink_color: vec4<f32>,
    parchment_base: vec4<f32>,
    parchment_burn: vec4<f32>,
    parchment_dark: vec4<f32>,
    params: vec4<f32>,
    world_offset_and_wetness: vec4<f32>
}

@group(1) @binding(0)
var<uniform> settings: InkSettings;

#import noisy_bevy::prelude
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions


struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var position: vec3<f32> =  vertex.position;
    var normal: vec3<f32> = vertex.normal;

    var model = mesh.model;


    out.world_normal = mesh_normal_local_to_world(normal);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);

    let depth = out.clip_position.z * out.clip_position.w;

    out.color = vertex.color;
    let is_ink = vertex.color.w > 0.5;
    if is_ink && depth > 0.2 {
        let range = clamp((depth -  0.2) / 4., 0., 1.);        
        let move_distance = mix(0., 0.15, range);
        let direction = vertex.color.xzy;
        position = position + move_distance * direction;
        out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
        out.clip_position = mesh_position_world_to_clip(out.world_position);
    } 

    return out;
}

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
    @builtin(position) clip_position: vec4<f32>,
};

@fragment
fn fragment(
    in: FragmentInput
) -> @location(0) vec4<f32> {
    var parchment_base : vec4<f32> = settings.parchment_base;
    var parchment_burn : vec4<f32> = settings.parchment_burn;
    var parchment_dark : vec4<f32> = settings.parchment_dark;

    let is_ink = in.color.w > 0.5;
    let modify_wetness = in.color.w > 0.8 || (in.color.w > 0.2 && in.color.w < 0.5);

    var vertex_color : vec4<f32> = vec4<f32>(in.color.xyz, 1.);

    if is_ink {
        vertex_color = settings.ink_color;
    }
    if modify_wetness {
        vertex_color = vertex_color * settings.world_offset_and_wetness.w;
    }


    let world_position = in.world_position + vec4<f32>(settings.world_offset_and_wetness.xz, 0., 0.);
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

    let overlay_mixer = mix(mix(overlay_1, overlay_2, settings.params.z), overlay_3, settings.params.w);

    let overlay_color = mix(parchment_base, parchment_burn, overlay_mixer);

    let init_bg = mix(vertex_color * overlay_color, overlay_color, 1. - settings.params.y);
    let bg = mix(init_bg, init_bg * settings.base_color , 0.3);

    let depth = clamp(mix(-0.3, 1.2, clamp(world_position.y + 1., 0., 1.)), 0., 1.);
    var darkening : f32 = clamp(sin(max(0., (world_position.y + 0.0537) * 12. * 3.14159)), 0., 1.);

    if darkening > 0.95 {
        darkening = 1. - 0.95;
        darkening = mix(0.3, 1., darkening * 0.05);
    } else {
        darkening = 1.;
    }

    let xmod = abs(abs(world_position.x) % 1. - 0.5);
    let ymod = abs(abs(world_position.z) % 1. - 0.5);
    let xmod_2 = xmod * 10. % 1.;
    let ymod_2 = ymod * 10. % 1.;

    var darken : bool = false;

    if xmod > 0.49 && ymod_2 < 0.3 {
        darken = true;
    } else if ymod > 0.49 && xmod_2 < 0.3 {
        darken = true;
    }

    if in.world_normal.y > 0.9 && darken && darkening > 0.7{
            darkening = 0.5;
    }

    let ink = mix(vec4<f32>(1., 1., 1., 1.), mix(settings.ink_color, vec4<f32>(1., 1., 1., 1.), darkening), settings.params.x);

    let color = mix(bg * ink , bg  * parchment_dark, 1. - depth);

    
    return color;
}