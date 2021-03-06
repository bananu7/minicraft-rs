#version 410 core

uniform sampler2D colorMap;
uniform sampler2D normalMap;

uniform vec3 eye;
uniform float time;

in TESOut {
    vec3 modelspacePosition;
    vec3 normal;
    vec2 texCoord;
    vec3 color;
} vertex;

out vec4 outColor;

float CalcDirectionalLightFactor(vec3 lightDirection, vec3 normal) {
    float DiffuseFactor = dot(normalize(normal), -lightDirection);

    if (DiffuseFactor > 0) {
        return DiffuseFactor;
    }
    else {
        return 0.0;
    }
}

float CalcPointLightFactor(vec3 lightPosition, vec3 normal, vec3 position) {
    vec3 LightDirection = position - lightPosition;
    float Distance = length(LightDirection);
    LightDirection = normalize(LightDirection);

    float Attenuation = 0.1 * Distance + //linear
                        0.001 * Distance * Distance; //exponential

    return CalcDirectionalLightFactor(LightDirection, normal) / Attenuation;
}

void main() {
    vec3 vnormal = vertex.normal;
    vec3 tnormal = texture(normalMap, vertex.texCoord).xyz * 2.0 - 1.0;

    vec3 normal;

    // reorient the texture normal basing on which face we're rendering
    if (abs(vnormal.x) > 0) {
        normal = vec3(vnormal.x, 1.0, vnormal.x) * tnormal.zyx;
    }
    else if (abs(vnormal.y) > 0) {
        normal = vec3(1.0, vnormal.y, vnormal.y) * tnormal.xzy;
    }
    else {
        normal = vec3(1.0, 1.0, vnormal.z) * tnormal.xyz;
    }

    normal = normalize(normal);

    // ------------------------

    vec3 vcolor = vertex.color;
    vec3 tcolor = texture(colorMap, vertex.texCoord).xyz;

    vec3 color = tcolor;

    // ------------------------

    struct DirectionalLight {
        vec3 Color;
        vec3 AmbientIntensity;
        vec3 DiffuseIntensity;
        vec3 Direction;
    } Light0;

    struct PointLight {
        vec3 Color;
        vec3 AmbientIntensity;
        vec3 DiffuseIntensity;
        vec3 Position;
    } Light1;

    Light0.Color = vec3(1.0, 1.0, 1.0);
    Light0.AmbientIntensity = vec3(0.01, 0.01, 0.01);
    Light0.DiffuseIntensity = vec3(0.8, 0.8, 0.8);
    Light0.Direction = vec3(-10, -100, -100);

    Light1.Color = vec3(1.0, 1.0, 1.0);
    Light1.AmbientIntensity = vec3(0.1, 0.1, 0.1);
    Light1.DiffuseIntensity = vec3(0.8, 0.8, 0.8);
    Light1.Position = vec3(8.0 * sin(time) + 8.0, 10.0, 8.0 * cos(time) + 8.0);

    vec3 MaterialDiffuseReflectivity = vec3(1.0, 1.0, 1.0);

    vec3 AmbientColor = Light0.AmbientIntensity * Light0.Color;
    //vec3 DiffuseColor = Light0.Color * Light0.DiffuseIntensity * CalcDirectionalLightFactor(Light0.Direction, normal);
    vec3 DiffuseColor = 
          Light1.Color 
        * Light1.DiffuseIntensity 
        * CalcPointLightFactor(Light1.Position, normal, vertex.modelspacePosition);

    vec3 computedColor = (DiffuseColor + AmbientColor) * color;

    //-------------------------------------------------------

    outColor = vec4(computedColor, 1.0);
    //outColor = vec4(1.0, 0.0, 0.0, 1.0);
    //outColor = vec4(vertex.texCoord, 0.0, 1.0);
    //outColor = vec4(normal, 1.0);
    //outColor = vec4(color, 1.0);
}
