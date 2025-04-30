#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D terrainTex;
uniform float screenHeight;

out vec4 finalColor;

void main() {
    float x = gl_FragCoord.x / float(textureSize(terrainTex, 0).x);
    float terrainY = texture(terrainTex, vec2(x, 0)).r * screenHeight;

    if (gl_FragCoord.y >= terrainY) {
        discard;
    } else {
        finalColor = vec4(0.2, 0.7, 0.3, 1.0);
    }
}
