#version 300 es

// from https://github.com/wilk10/shader_practice/tree/main/src/shaders/fire

precision highp float;
precision highp sampler2DArray;

in vec2 uv;
out vec4 o_Target;


uniform sampler2D FireTexture_texture;

uniform FireMaterial {
    vec4 _base_color;
    float _flame_height;
    float _distorsion_level;
    float _time;
};

vec2 random2( vec2 p ) {
    return fract(
        sin(
            vec2(
                dot(p, vec2(127.1, 311.7)),
                dot(p, vec2(269.5, 183.3))
            )
        ) * 43758.5453
    );
}

float cellularNoise(vec2 uv, float scale_factor) {
    // Scale
    uv *= scale_factor;

    // Tile the space
    vec2 i_st = floor(uv);
    vec2 f_st = fract(uv);

    float m_dist = 1.;  // minimum distance

    for (int y= -1; y <= 1; y++) {
        for (int x= -1; x <= 1; x++) {
            // Neighbor place in the grid
            vec2 neighbor = vec2(float(x),float(y));

            // Random position from current + neighbor place in the grid
            vec2 point = random2(i_st + neighbor);

			// Vector between the pixel and the point
            vec2 diff = neighbor + point - f_st;

            // Distance to the point
            float dist = length(diff);

            // Keep the closer distance
            m_dist = min(m_dist, dist);
        }
    }
    return m_dist;
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

vec2 animateVertically(vec2 uv, float time, float factor) {
    vec2 vertical_animation = vec2(factor * time, factor * time);
    return uv + vertical_animation;
}

vec4 encodeSRGB(vec4 linearRGB_in)
{
    vec3 linearRGB = linearRGB_in.rgb;
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return vec4(mix(a, b, c), linearRGB_in.a);
}


void main() {
    vec2 secondary_simple_noise_uv = animateVertically(uv, _time, 0.5);
    float secondary_simple_noise = simpleNoise(secondary_simple_noise_uv, _distorsion_level);

    vec2 cellular_uv = animateVertically(uv, _time, 0.25);
    vec2 lerped_cellular_uv = mix(cellular_uv, vec2(secondary_simple_noise), vec2(0.5));

    float cellular = cellularNoise(lerped_cellular_uv, _distorsion_level);

    vec2 main_simple_noise_uv = animateVertically(uv, _time, 0.3);
    float main_simple_noise = simpleNoise(main_simple_noise_uv, _distorsion_level);

    float total_noise = main_simple_noise * cellular;
    vec2 vertical_lerped_uv = mix(uv, vec2(total_noise), vec2(_flame_height, _flame_height));
    vertical_lerped_uv += vec2(_flame_height / 2.0, _flame_height / 2.0);

    vec2 distortion = vec2(vertical_lerped_uv);
    vec4 image = texture(FireTexture_texture, distortion);
    vec4 result = clamp(image, vec4(0.), vec4(1.));
    result *= _base_color;
    result *= vec4(vec3(10.), 1.);
    o_Target = encodeSRGB(result);
}