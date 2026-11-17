//! WebGL2 Renderer

use crate::bike::Bike;
use crate::camera::Camera;
use crate::ui::UI;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation, WebGlVertexArrayObject};
use nalgebra::{Matrix4, Vector3, Point3};
use std::rc::Rc;
use js_sys::Float32Array;
use wasm_bindgen::JsCast;

pub struct Renderer {
    gl: Rc<WebGl2RenderingContext>,
    canvas: HtmlCanvasElement,
    
    // Shaders
    bike_program: WebGlProgram,
    ground_program: WebGlProgram,
    skybox_program: WebGlProgram,
    
    // Buffers
    bike_vao: WebGlVertexArrayObject,
    ground_vao: WebGlVertexArrayObject,
    skybox_vao: WebGlVertexArrayObject,
    
    // Uniform locations
    bike_uniforms: BikeUniforms,
    ground_uniforms: GroundUniforms,
    skybox_uniforms: SkyboxUniforms,
    
    // Camera matrices
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    
    // Time
    time: f32,
}

struct BikeUniforms {
    u_model: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    u_color: WebGlUniformLocation,
    u_time: WebGlUniformLocation,
    u_light_dir: WebGlUniformLocation,
    u_camera_pos: WebGlUniformLocation,
}

struct GroundUniforms {
    u_model: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    u_color: WebGlUniformLocation,
    u_time: WebGlUniformLocation,
}

struct SkyboxUniforms {
    u_view: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    u_time: WebGlUniformLocation,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        web_sys::console::log_1(&"[RENDERER] Getting webgl2 context...".into());
        let ctx = canvas.get_context("webgl2")?;
        let gl = match ctx {
            Some(c) => c.dyn_into::<WebGl2RenderingContext>().map_err(|_| {
                JsValue::from_str("Failed to cast to WebGl2RenderingContext")
            })?,
            None => return Err(JsValue::from_str("WebGL2 not supported - get_context returned null")),
        };
        web_sys::console::log_1(&"[RENDERER] WebGL2 context obtained!".into());

        let gl = Rc::new(gl);
        
        // Enable depth testing and culling
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.depth_func(WebGl2RenderingContext::LEQUAL);
        gl.clear_color(0.53, 0.81, 0.92, 1.0); // Sky blue
        gl.clear_depth(1.0);
        web_sys::console::log_1(&"[RENDERER] GL state set".into());

        // Compile shaders
        web_sys::console::log_1(&"[RENDERER] Compiling bike shader...".into());
        let bike_program = compile_program(&gl, BIKE_VERTEX_SHADER, BIKE_FRAGMENT_SHADER)?;
        web_sys::console::log_1(&"[RENDERER] Compiling ground shader...".into());
        let ground_program = compile_program(&gl, GROUND_VERTEX_SHADER, GROUND_FRAGMENT_SHADER)?;
        web_sys::console::log_1(&"[RENDERER] Compiling skybox shader...".into());
        let skybox_program = compile_program(&gl, SKYBOX_VERTEX_SHADER, SKYBOX_FRAGMENT_SHADER)?;
        web_sys::console::log_1(&"[RENDERER] All shaders compiled!".into());

        // Get uniform locations - use expect with message instead of unwrap
        let get_uniform = |prog: &WebGlProgram, name: &str| -> WebGlUniformLocation {
            gl.get_uniform_location(prog, name)
                .unwrap_or_else(|| panic!("Uniform '{}' not found", name))
        };
        let bike_uniforms = BikeUniforms {
            u_model: get_uniform(&bike_program, "u_model"),
            u_view: get_uniform(&bike_program, "u_view"),
            u_projection: get_uniform(&bike_program, "u_projection"),
            u_color: get_uniform(&bike_program, "u_color"),
            u_time: get_uniform(&bike_program, "u_time"),
            u_light_dir: get_uniform(&bike_program, "u_light_dir"),
            u_camera_pos: get_uniform(&bike_program, "u_camera_pos"),
        };

        let ground_uniforms = GroundUniforms {
            u_model: get_uniform(&ground_program, "u_model"),
            u_view: get_uniform(&ground_program, "u_view"),
            u_projection: get_uniform(&ground_program, "u_projection"),
            u_color: get_uniform(&ground_program, "u_color"),
            u_time: get_uniform(&ground_program, "u_time"),
        };

        let skybox_uniforms = SkyboxUniforms {
            u_view: get_uniform(&skybox_program, "u_view"),
            u_projection: get_uniform(&skybox_program, "u_projection"),
            u_time: get_uniform(&skybox_program, "u_time"),
        };
        web_sys::console::log_1(&"[RENDERER] Uniforms found".into());

        // Create geometry
        web_sys::console::log_1(&"[RENDERER] Creating geometry...".into());
        let bike_vao = create_bike_geometry(&gl)?;
        let ground_vao = create_ground_geometry(&gl)?;
        let skybox_vao = create_skybox_geometry(&gl)?;
        web_sys::console::log_1(&"[RENDERER] Geometry created!".into());

        Ok(Self {
            gl,
            canvas: canvas.clone(),
            bike_program,
            ground_program,
            skybox_program,
            bike_vao,
            ground_vao,
            skybox_vao,
            bike_uniforms,
            ground_uniforms,
            skybox_uniforms,
            view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
            time: 0.0,
        })
    }

    pub fn begin_frame(&mut self) {
        let gl = &self.gl;
        let width = self.canvas.width() as i32;
        let height = self.canvas.height() as i32;
        gl.viewport(0, 0, width, height);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn end_frame(&mut self) {
        // Frame end - could add post-processing here
    }

    pub fn render_scene(&mut self, bike: &Bike, camera: &Camera) {
        self.time += 1.0 / 60.0;
        
        // Update camera matrices
        self.view_matrix = camera.get_view_matrix();
        self.projection_matrix = camera.get_projection_matrix(
            self.canvas.width() as f32 / self.canvas.height() as f32
        );

        // Render skybox first
        self.render_skybox();

        // Render ground
        self.render_ground();

        // Render bike
        self.render_bike(bike);

        // Render wheel traces
        self.render_wheel_traces(bike);
    }

    fn render_skybox(&mut self) {
        let gl = &self.gl;
        gl.use_program(Some(&self.skybox_program));
        gl.bind_vertex_array(Some(&self.skybox_vao));
        gl.depth_func(WebGl2RenderingContext::LEQUAL); // Skybox at infinity
        
        // Remove translation from view matrix for skybox
        let mut view = self.view_matrix;
        view[(0, 3)] = 0.0;
        view[(1, 3)] = 0.0;
        view[(2, 3)] = 0.0;
        
        gl.uniform_matrix4fv_with_f32_array(Some(&self.skybox_uniforms.u_view), false, &view.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.skybox_uniforms.u_projection), false, &self.projection_matrix.as_slice());
        gl.uniform1f(Some(&self.skybox_uniforms.u_time), self.time);
        
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 36);
        gl.depth_func(WebGl2RenderingContext::LESS);
    }

    fn render_ground(&mut self) {
        let gl = &self.gl;
        gl.use_program(Some(&self.ground_program));
        gl.bind_vertex_array(Some(&self.ground_vao));
        
        let model = Matrix4::identity();
        gl.uniform_matrix4fv_with_f32_array(Some(&self.ground_uniforms.u_model), false, &model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.ground_uniforms.u_view), false, &self.view_matrix.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.ground_uniforms.u_projection), false, &self.projection_matrix.as_slice());
        gl.uniform4f(Some(&self.ground_uniforms.u_color), 0.2, 0.5, 0.2, 1.0);
        gl.uniform1f(Some(&self.ground_uniforms.u_time), self.time);
        
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }

    fn render_bike(&mut self, bike: &Bike) {
        let gl = &self.gl;
        gl.use_program(Some(&self.bike_program));
        gl.bind_vertex_array(Some(&self.bike_vao));
        
        let model = self.create_bike_model_matrix(bike);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bike_uniforms.u_model), false, &model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bike_uniforms.u_view), false, &self.view_matrix.as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bike_uniforms.u_projection), false, &self.projection_matrix.as_slice());
        gl.uniform4f(Some(&self.bike_uniforms.u_color), 0.8, 0.2, 0.2, 1.0); // Red bike
        gl.uniform1f(Some(&self.bike_uniforms.u_time), self.time);
        gl.uniform3f(Some(&self.bike_uniforms.u_light_dir), -0.5, -1.0, -0.3);
        gl.uniform3f(Some(&self.bike_uniforms.u_camera_pos), 
            self.view_matrix[(0, 3)], self.view_matrix[(1, 3)], self.view_matrix[(2, 3)]);
        
        // Draw bike body
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 36); // Cube for body
        
        // Draw wheels
        let (front_wheel, rear_wheel) = bike.get_wheel_positions();
        self.draw_wheel(&front_wheel, bike.wheel_rotation, bike.transform.rotation);
        self.draw_wheel(&rear_wheel, bike.wheel_rotation, bike.transform.rotation);
    }

    fn create_bike_model_matrix(&self, bike: &Bike) -> Matrix4<f32> {
        let mut model = Matrix4::identity();
        model.append_translation_mut(&bike.transform.position);
        // Convert quaternion to rotation matrix
        let rot_matrix = bike.transform.rotation.to_homogeneous();
        model = model * rot_matrix;
        model.append_nonuniform_scaling_mut(&Vector3::new(2.0, 0.8, 0.6)); // Bike body dimensions
        model
    }

    fn draw_wheel(&mut self, position: &Vector3<f32>, rotation: f32, bike_rotation: nalgebra::UnitQuaternion<f32>) {
        let gl = &self.gl;
        
        let mut model = Matrix4::identity();
        model.append_translation_mut(position);
        // Convert quaternion to rotation matrix
        let rot_matrix = bike_rotation.to_homogeneous();
        model = model * rot_matrix;
        // Rotate wheel around X axis for rolling
        let wheel_spin = nalgebra::UnitQuaternion::from_axis_angle(&nalgebra::Vector3::x_axis(), rotation);
        let wheel_rot = wheel_spin.to_homogeneous();
        model = model * wheel_rot;
        model.append_nonuniform_scaling_mut(&nalgebra::Vector3::new(0.35, 0.1, 0.35));
        
        gl.uniform_matrix4fv_with_f32_array(Some(&self.bike_uniforms.u_model), false, &model.as_slice());
        gl.uniform4f(Some(&self.bike_uniforms.u_color), 0.1, 0.1, 0.1, 1.0); // Black wheels
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 36, 36); // Second cube for wheel
    }

    fn render_wheel_traces(&mut self, _bike: &Bike) {
        // TODO: Implement wheel trace rendering for drift/wheelie marks
    }

    pub fn render_ui(&mut self, ui: &mut UI, bike: &Bike) {
        // UI é renderizada via overlay HTML/CSS, não WebGL.
        // As barras de input (throttle/freio/direção) são atualizadas aqui.
        ui.update_dom(bike);
    }
}

fn compile_program(
    gl: &WebGl2RenderingContext,
    vert_src: &str,
    frag_src: &str,
) -> Result<WebGlProgram, JsValue> {
    let vert_shader = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;
    let frag_shader = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;
    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);
    
    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(JsValue::from_str(&gl.get_program_info_log(&program).unwrap_or("Unknown error".into())))
    }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    
    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(JsValue::from_str(&gl.get_shader_info_log(&shader).unwrap_or("Unknown error".into())))
    }
}

fn create_bike_geometry(gl: &WebGl2RenderingContext) -> Result<WebGlVertexArrayObject, JsValue> {
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(&vao));

    // Bike body (cube) + wheel (cube) vertices
    let vertices = create_cube_vertices(1.0);
    let vertices2 = create_cube_vertices(1.0);
    let mut all_vertices = Vec::with_capacity(vertices.len() + vertices2.len());
    all_vertices.extend_from_slice(&vertices);
    all_vertices.extend_from_slice(&vertices2);

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &Float32Array::from(&all_vertices[..]),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    // Position attribute (location 0)
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 0);
    
    // Normal attribute (location 1)
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 3 * 4);

    gl.bind_vertex_array(None);
    Ok(vao)
}

fn create_ground_geometry(gl: &WebGl2RenderingContext) -> Result<WebGlVertexArrayObject, JsValue> {
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(&vao));

    // Large ground plane
    let size = 500.0;
    let vertices: [f32; 6 * 6] = [
        // Positions          // Normals
        -size, 0.0, -size,    0.0, 1.0, 0.0,
         size, 0.0, -size,    0.0, 1.0, 0.0,
         size, 0.0,  size,    0.0, 1.0, 0.0,
        -size, 0.0, -size,    0.0, 1.0, 0.0,
         size, 0.0,  size,    0.0, 1.0, 0.0,
        -size, 0.0,  size,    0.0, 1.0, 0.0,
    ];

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &Float32Array::from(&vertices[..]),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 0);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 6 * 4, 3 * 4);

    gl.bind_vertex_array(None);
    Ok(vao)
}

fn create_skybox_geometry(gl: &WebGl2RenderingContext) -> Result<WebGlVertexArrayObject, JsValue> {
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(&vao));

    let vertices: [f32; 36 * 3] = [
        // Front
        -1.0, -1.0,  1.0,  1.0, -1.0,  1.0,  1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,  1.0,  1.0,  1.0, -1.0,  1.0,  1.0,
        // Back
        -1.0, -1.0, -1.0, -1.0,  1.0, -1.0,  1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,  1.0,  1.0, -1.0,  1.0, -1.0, -1.0,
        // Top
        -1.0,  1.0, -1.0, -1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,  1.0,  1.0,  1.0,  1.0,  1.0, -1.0,
        // Bottom
        -1.0, -1.0, -1.0,  1.0, -1.0, -1.0,  1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,  1.0, -1.0,  1.0, -1.0, -1.0,  1.0,
        // Right
         1.0, -1.0, -1.0,  1.0,  1.0, -1.0,  1.0,  1.0,  1.0,
         1.0, -1.0, -1.0,  1.0,  1.0,  1.0,  1.0, -1.0,  1.0,
        // Left
        -1.0, -1.0, -1.0, -1.0, -1.0,  1.0, -1.0,  1.0,  1.0,
        -1.0, -1.0, -1.0, -1.0,  1.0,  1.0, -1.0,  1.0, -1.0,
    ];

    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &Float32Array::from(&vertices[..]),
        WebGl2RenderingContext::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 3 * 4, 0);

    gl.bind_vertex_array(None);
    Ok(vao)
}

fn create_cube_vertices(size: f32) -> [f32; 216] {
    let s = size / 2.0;
    [
        // Front face
        -s, -s,  s,  0.0,  0.0,  1.0,
         s, -s,  s,  0.0,  0.0,  1.0,
         s,  s,  s,  0.0,  0.0,  1.0,
        -s, -s,  s,  0.0,  0.0,  1.0,
         s,  s,  s,  0.0,  0.0,  1.0,
        -s,  s,  s,  0.0,  0.0,  1.0,
        // Back face
        -s, -s, -s,  0.0,  0.0, -1.0,
        -s,  s, -s,  0.0,  0.0, -1.0,
         s,  s, -s,  0.0,  0.0, -1.0,
        -s, -s, -s,  0.0,  0.0, -1.0,
         s,  s, -s,  0.0,  0.0, -1.0,
         s, -s, -s,  0.0,  0.0, -1.0,
        // Top face
        -s,  s, -s,  0.0,  1.0,  0.0,
        -s,  s,  s,  0.0,  1.0,  0.0,
         s,  s,  s,  0.0,  1.0,  0.0,
        -s,  s, -s,  0.0,  1.0,  0.0,
         s,  s,  s,  0.0,  1.0,  0.0,
         s,  s, -s,  0.0,  1.0,  0.0,
        // Bottom face
        -s, -s, -s,  0.0, -1.0,  0.0,
         s, -s, -s,  0.0, -1.0,  0.0,
         s, -s,  s,  0.0, -1.0,  0.0,
        -s, -s, -s,  0.0, -1.0,  0.0,
         s, -s,  s,  0.0, -1.0,  0.0,
        -s, -s,  s,  0.0, -1.0,  0.0,
        // Right face
         s, -s, -s,  1.0,  0.0,  0.0,
         s,  s, -s,  1.0,  0.0,  0.0,
         s,  s,  s,  1.0,  0.0,  0.0,
         s, -s, -s,  1.0,  0.0,  0.0,
         s,  s,  s,  1.0,  0.0,  0.0,
         s, -s,  s,  1.0,  0.0,  0.0,
        // Left face
        -s, -s, -s, -1.0,  0.0,  0.0,
        -s, -s,  s, -1.0,  0.0,  0.0,
        -s,  s,  s, -1.0,  0.0,  0.0,
        -s, -s, -s, -1.0,  0.0,  0.0,
        -s,  s,  s, -1.0,  0.0,  0.0,
        -s,  s, -s, -1.0,  0.0,  0.0,
    ]
}

// Shaders
const BIKE_VERTEX_SHADER: &str = r#"#version 300 es
in vec3 a_position;
in vec3 a_normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform float u_time;

out vec3 v_world_pos;
out vec3 v_normal;
out vec3 v_view_dir;

void main() {
    mat4 model = u_model;
    vec4 world_pos = model * vec4(a_position, 1.0);
    v_world_pos = world_pos.xyz;
    v_normal = mat3(transpose(inverse(model))) * a_normal;
    v_view_dir = (u_view * world_pos).xyz;
    gl_Position = u_projection * u_view * world_pos;
}
"#;

const BIKE_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

in vec3 v_world_pos;
in vec3 v_normal;
in vec3 v_view_dir;

uniform vec4 u_color;
uniform float u_time;
uniform vec3 u_light_dir;
uniform vec3 u_camera_pos;

out vec4 fragColor;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(u_light_dir);
    vec3 view_dir = normalize(u_camera_pos - v_world_pos);
    
    // Ambient
    float ambient = 0.3;
    
    // Diffuse
    float diff = max(dot(normal, light_dir), 0.0);
    
    // Specular
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    
    // Fresnel for chrome effect
    float fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), 3.0);
    
    vec3 color = u_color.rgb * (ambient + diff * 0.7) + vec3(spec * 0.5) + vec3(fresnel * 0.2);
    
    // Add some procedural detail
    float detail = sin(v_world_pos.x * 10.0 + u_time) * sin(v_world_pos.z * 10.0) * 0.02;
    color += detail;
    
    fragColor = vec4(color, u_color.a);
}
"#;

const GROUND_VERTEX_SHADER: &str = r#"#version 300 es
in vec3 a_position;
in vec3 a_normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform float u_time;

out vec3 v_world_pos;
out vec3 v_normal;

void main() {
    vec4 world_pos = u_model * vec4(a_position, 1.0);
    v_world_pos = world_pos.xyz;
    v_normal = a_normal;
    gl_Position = u_projection * u_view * world_pos;
}
"#;

const GROUND_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

in vec3 v_world_pos;
in vec3 v_normal;

uniform vec4 u_color;
uniform float u_time;

out vec4 fragColor;

void main() {
    // Grid pattern
    float grid = 0.0;
    float cell_size = 20.0;
    vec2 grid_coord = v_world_pos.xz / cell_size;
    vec2 grid_frac = fract(grid_coord);
    
    float line_width = 0.02;
    if (grid_frac.x < line_width || grid_frac.x > 1.0 - line_width ||
        grid_frac.y < line_width || grid_frac.y > 1.0 - line_width) {
        grid = 0.3;
    }
    
    // Distance fade
    float dist = length(v_world_pos.xz);
    float fade = smoothstep(100.0, 400.0, dist);
    
    vec3 color = mix(u_color.rgb, u_color.rgb * 0.5, fade);
    color += vec3(grid * (1.0 - fade));
    
    // Subtle animation
    color += sin(v_world_pos.x * 0.1 + u_time * 0.5) * 0.01;
    
    fragColor = vec4(color, 1.0);
}
"#;

const SKYBOX_VERTEX_SHADER: &str = r#"#version 300 es
in vec3 a_position;

uniform mat4 u_view;
uniform mat4 u_projection;
uniform float u_time;

out vec3 v_pos;

void main() {
    v_pos = a_position;
    vec4 pos = u_projection * u_view * vec4(a_position, 1.0);
    gl_Position = pos.xyww; // Force depth to 1.0
}
"#;

const SKYBOX_FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

in vec3 v_pos;

uniform float u_time;

out vec4 fragColor;

// Simple fbm for clouds - declared BEFORE main
float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.5;
    for (int i = 0; i < 4; i++) {
        value += amplitude * sin(p.x * 10.0 + sin(p.y * 10.0));
        p *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}

void main() {
    vec3 dir = normalize(v_pos);
    
    // Sky gradient
    vec3 horizon = vec3(0.5, 0.7, 0.9);
    vec3 zenith = vec3(0.2, 0.4, 0.7);
    
    float sun_intensity = max(dot(dir, normalize(vec3(0.3, 0.8, 0.2))), 0.0);
    vec3 sun_color = vec3(1.0, 0.9, 0.7) * pow(sun_intensity, 32.0) * 0.5;
    
    vec3 sky = mix(zenith, horizon, smoothstep(0.0, 0.3, dir.y + 0.5));
    sky += sun_color;
    
    // Clouds
    float clouds = fbm(dir.xz * 0.02 + u_time * 0.001) * 0.3;
    sky = mix(sky, vec3(1.0, 1.0, 1.0) * 0.8, clouds * smoothstep(0.0, 0.5, dir.y + 0.5));
    
    // Stars (at night - high up)
    float stars = pow(max(sin(dir.x * 100.0) * sin(dir.y * 100.0) * sin(dir.z * 100.0), 0.0), 100.0);
    sky += vec3(stars) * smoothstep(0.5, 1.0, dir.y);
    
    fragColor = vec4(sky, 1.0);
}
"#;

use wasm_bindgen::JsValue;