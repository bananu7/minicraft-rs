
pub const LIGHT_VERT_SHADER : &str = "
#version 410

uniform mat4 matrix;

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 vColor;

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
    Light0.AmbientIntensity = vec3(0.1, 0.1, 0.1);
    Light0.DiffuseIntensity = vec3(0.8, 0.8, 0.8);
    Light0.Direction = vec3(-10, -100, -100);

    Light1.Color = vec3(1.0, 1.0, 1.0);
    Light1.AmbientIntensity = vec3(0.1, 0.1, 0.1);
    Light1.DiffuseIntensity = vec3(0.8, 0.8, 0.8);
    Light1.Position = vec3(20, 20, 20);

    vec3 MaterialDiffuseReflectivity = vec3(1.0, 1.0, 1.0);

    vec3 AmbientColor = Light0.AmbientIntensity * Light0.Color;
    //vec3 DiffuseColor = Light0.Color * Light0.DiffuseIntensity * CalcDirectionalLightFactor(Light0.Direction, normal);
    vec3 DiffuseColor = Light1.Color * Light1.DiffuseIntensity * CalcPointLightFactor(Light1.Position, normal, position) ;

    gl_Position = matrix * vec4(position, 1.0);
    vColor = (DiffuseColor + AmbientColor) * color;
}
";
