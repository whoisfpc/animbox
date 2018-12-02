#version 430 core
in vec3 fragPosition;
in vec3 fragNormal;

uniform vec3 AmbientColor=vec3(0.2);
uniform vec3 LightDirection=normalize(vec3(1,5,2));
uniform vec3 LightColor=vec3(1);
uniform vec3 DiffuseColor=vec3(0.5);

out vec3 finalColor;

void main() {
	// Compute irradiance (sum of ambient & direct lighting)
	vec3 irradiance=AmbientColor + LightColor * max(0,dot(LightDirection,fragNormal));

	// Diffuse reflectance
	vec3 reflectance=irradiance * DiffuseColor;

	// Gamma correction
	gl_FragColor=vec4(sqrt(reflectance),1);
}
