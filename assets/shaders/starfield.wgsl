
fn hash(p: vec2<f32>) -> vec2<f32> {
	// return fract(sin(vec2(dot(p,vec2(127.1,311.7)),dot(p,vec2(269.5,183.3))))*43758.5453);
	return fract(sin(vec2<f32>(dot(p,vec2<f32>(127.1,311.7)),dot(p,vec2<f32>(269.5,183.3))))*43758.5453);
}

fn hash22(p: vec2<f32>) -> vec2<f32>
{
	var p3: vec3<f32> = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 = p3 + dot(p3, p3.yzx+19.19);
    return fract((p3.xx+p3.yz)*p3.zy);
}

fn noise(p: vec2<f32>) -> f32 {
    #ifdef USE_VORONOI
    
    let n = floor(p);
    let f = fract(p);

    var mg: vec2<f32> = vec2<f32>(0.0, 0.0);
    var mr: vec2<f32> = vec2<f32>(0.0, 0.0);

    let md: f32 = 8.0;
    for(int j = -1; j <= 1; ++j) {
        for(int i = -1; i <= 1; ++i) {
            vec2 g = vec2(float(i), float(j));
            vec2 o = hash22(n + g);

            vec2 r = g + o - f;
            float d = dot(r, r);

            if(d < md) {
                md = d;
                mr = r;
                mg = g;
            }
        }
    }
    return md;
    
    #else
    
    let n: vec2<f32> = floor(p);
    let f: vec2<f32> = fract(p);

    var md: f32 = 1.0;

    // Scale a bit to move from cell edges
    let o: vec2<f32> = hash22(n)*0.96 + 0.02;

    let r: vec2<f32> = o - f;
    let d: f32 = dot(r, r);

    md = min(d, md);

    return md;
    
    #endif
}

fn starfield(samplePosition: vec2<f32>, threshold: f32) -> vec3<f32> {
    let starValue: f32 = noise(samplePosition);
    
    var power: f32 = max(1.0 - (starValue / threshold), 0.0);
    power = power * power * power;
    
    #ifdef SHOW_CELLS
    power += starValue;
    #endif
    
    return vec3<f32>(power);
}

struct CustomMaterial {
    pos: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;


@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {

    var finalColor: vec3<f32> = vec3<f32>(0.0,0.0,0.0);
    let sCoord: vec2<f32> = (position.xy / 800.0)*5.0;
    var pos: vec2<f32> = material.pos.xy / 100.0;
    pos.y = -pos.y;
    
    // Add starfields
    for (var i: i32 = 1; i <= 10; i++) {
        let fi: f32 = f32(i);
        let inv: f32 = sqrt(1.0/fi);
    	finalColor += starfield((sCoord + vec2(fi*100.0, -fi*50.0)) * (1.0 + fi * 0.2) + pos, 0.0005)*inv;
    }
    
    return vec4<f32>(finalColor, 1.0);

    // return material.color;
}