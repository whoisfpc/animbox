#version 430 core
layout(location=0) in vec3 Position;
layout(location=1) in vec3 Normal;

out vec3 fragPosition;
out vec3 fragNormal;

uniform mat4 ModelMtx=mat4(1);
uniform mat4 ModelViewProjMtx=mat4(1);

void main() {
	gl_Position=ModelViewProjMtx * vec4(Position,1);

	fragPosition=vec3(ModelMtx * vec4(Position,1));
	fragNormal=vec3(ModelMtx * vec4(Normal,0));
}
