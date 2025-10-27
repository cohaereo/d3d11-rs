struct VSOutput {
    float4 position : SV_POSITION;
    float4 color : COLOR;
};

static const float4 vertices[3] = {
    float4(0.0, 0.5, 0.0, 1.0),
    float4(0.5, -0.5, 0.0, 1.0),
    float4(-0.5, -0.5, 0.0, 1.0)
};

static const float4 colors[3] = {
    float4(1.0, 0.0, 0.0, 1.0),
    float4(0.0, 1.0, 0.0, 1.0),
    float4(0.0, 0.0, 1.0, 1.0)
};

VSOutput VSMain(uint vertexID : SV_VertexID) {
    VSOutput output;
    output.position = vertices[vertexID];
    output.color = colors[vertexID];
    return output;
}

float4 PSMain(VSOutput input) : SV_Target {
    return input.color;
}
