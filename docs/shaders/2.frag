precision mediump float;
uniform vec2 u_resolution;
uniform float u_time;

float voronoi(vec2 x) {
    vec2 n = floor(x);
    vec2 f = fract(x);

    float m = 8.0;
    for(int j = -1; j <= 1; j++) {
        for(int i = -1; i <= 1; i++) {
            vec2 g = vec2(float(i), float(j));
            vec2 o = fract(sin(vec2(dot(n + g, vec2(13.0, 7.0)), dot(n + g, vec2(7.0, 13.0)))) * 43758.5453);
            o = 0.5 + 0.5 * sin(u_time + 6.2831 * o);
            vec2 r = g + o - f;
            float d = dot(r, r);
            m = min(m, d);
        }
    }
    return sqrt(m);
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    uv.x *= u_resolution.x / u_resolution.y;
    uv *= 5.0;

    float v = voronoi(uv);
    float pulse = 0.5 + 0.5 * sin(u_time * 2.0 - v * 10.0);

    vec3 col = vec3(0.0, 0.3, 0.5);
    col += vec3(0.0, 0.5, 0.5) * pulse * (1.0 - v);
    col += vec3(0.0, 0.8, 0.8) * pow(1.0 - v, 10.0);

    gl_FragColor = vec4(col, 1.0);
}
