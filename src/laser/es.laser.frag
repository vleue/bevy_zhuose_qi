#version 300 es

precision mediump float;

in vec2 uv;
out vec4 o_Target;

uniform LaserMaterial {
    vec4 _base_color;
    float _width;
    float _time;
};

vec4 encodeSRGB(vec4 linearRGB_in)
{
    vec3 linearRGB = linearRGB_in.rgb;
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return vec4(mix(a, b, c), linearRGB_in.a);
}

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
    vec2 uv2 = (2. * uv - vec2(1.0, 1.0));

    if (uv2.x < 0.) {
		uv2.x += _time / 4.;
    } else {
		uv2.x -= _time / 4.;
	}

    float C,S,
          dist = length(2.*fract(uv2)-1.),
          r = rand(floor(uv2));

    vec2 uv3 = uv * 2. - 1.;
    uv3.x *= 0.5;

    uv3 *= mix(f((uv2 + r) * rot(pi * r * 2.)),
            f(uv2 * rot(pi - 1.)),
            smoothstep(0.5, .93, dist)).rg / 2.;

    vec3 col = _base_color.rgb * smoothstep(_width / 1000.0, 0., abs(uv3.x));

    float alpha = max(col.r, max(col.g, col.b));

    o_Target = encodeSRGB(vec4(col, alpha));
}
