#version 450

layout(location = 0) in vec2 _uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform LaserMaterial {
    vec4 _base_color;
    float _width;
    float _time;
};

precision mediump float;

float hash(vec2 p) { return fract(1e4 * sin(17.0 * p.x + p.y * 0.1) * (0.1 + abs(sin(p.y * 13.0 + p.x)))); }

float simpleNoise(vec2 uv, float scale_factor) {
    // Scale
    uv *= scale_factor;

    vec2 i = floor(uv);
    vec2 f = fract(uv);

    // Four corners in 2D of a tile
    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));

    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

vec4 f(vec2 uv) {
    return vec4(
        simpleNoise(uv, 20.0),
        1.0,
        1.0,
        1.0
    );
}

#define rot(a) mat2(C=cos(a),S=-sin(a),-S,C)
#define rand(co) fract(sin(mod(dot(co ,vec2(12.9898,78.233)),3.14))*43758.5453)

const highp float pi = 3.141592653589793;


void main() {
    vec2 uv = (2. * _uv - vec2(1.0, 1.0));

    if (uv.x < 0.) {
		uv.x += _time / 4.;
    } else {
		uv.x -= _time / 4.;
	}

    float C,S,
          dist = length(2.*fract(uv)-1.),
          r = rand(floor(uv));

    vec2 uv2 = _uv * 2. - 1.;
    uv2.x *= 0.5;

    uv2 *= mix(f((uv + r) * rot(pi * r * 2.)),
            f(uv * rot(pi - 1.)),
            smoothstep(0.5, .93, dist)).rg / 2.;

    vec3 col = _base_color.rgb * smoothstep(_width / 1000.0, 0., abs(uv2.x));

    float alpha = max(col.r, max(col.g, col.b));

    o_Target = vec4(col, alpha);
}
