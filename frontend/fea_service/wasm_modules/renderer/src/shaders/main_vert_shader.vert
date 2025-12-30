attribute vec3 a_vertex_position;
attribute float a_is_to_scale;
attribute vec3 a_reference_point;
attribute vec3 a_vertex_displacement;
attribute vec4 a_vertex_color;
uniform float u_displacement_magnitude;
uniform float u_point_size;
uniform vec3 u_shift;
uniform float u_scale;
uniform float u_d_scale;
uniform vec2 u_screen_size;
uniform mat4 u_model_view_matrix;
uniform mat4 u_projection_matrix;

varying lowp vec4 v_color;

//
// This code essentially moves the vertex to a pixel center.
// 
vec4 compute_pixel_location(in vec4 point, in vec2 screen_size) 
{
    vec2 offset = vec2(0.5, 0.5);
    //
    // Remove perspective divide from xy
    //
    vec2 pt = point.xy / point.w;
    //
    // Convert to pixel space
    //
    vec2 xyf = vec2((pt / 2.0 + 0.5) * screen_size);
    //
    // truncate to nearest pixel
    //
    vec2 xyr = floor(xyf + offset);
    //
    // Move to xy pixel center and convert back 
    //
    vec2 res = (xyr + offset) * 2.0 / screen_size - 1.0;
    //
    // Add perspective divide back into xy and remake gl_position
    //
    vec4 p = vec4(res * point.w, point.z, point.w);
    return p;
}

void main(void) 
{
    vec3 vertex_position;
    if (a_is_to_scale == 1.0) 
    {
        vertex_position = (a_vertex_position + (a_vertex_displacement * u_displacement_magnitude)) * u_scale - u_shift;
    }
    else
    {
        vertex_position = (a_reference_point + (a_vertex_displacement * u_displacement_magnitude)) * u_scale - u_shift + 
            a_vertex_position / u_d_scale;
    }
    v_color = a_vertex_color;
    gl_PointSize = u_point_size;
    gl_Position = u_projection_matrix * u_model_view_matrix * vec4(vertex_position, 1.0);
    gl_Position = compute_pixel_location(gl_Position, u_screen_size);
}
