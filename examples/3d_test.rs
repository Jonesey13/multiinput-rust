#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate num;
extern crate multiinput;

pub mod shader;


pub use multiinput::*;

fn main() {
    use nalgebra::{Mat4, Vec3, Iso3, Pnt3,ToHomogeneous, Inv, BaseFloat, Rotation, Translation, PerspMat3, cross, normalize};
    use num::Zero;

    use glium::{DisplayBuild, Surface, DrawParameters, BackfaceCullingMode, DepthTest};

    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);

    let display = glium::glutin::WindowBuilder::new()
        .with_title("".to_string())
        .with_dimensions(800, 600)
        .with_depth_buffer(24)
        .build_glium().unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3]
    }

    implement_vertex!(Vertex, position, normal);
    fn to_vec3((x, y, z): (f32, f32, f32)) -> Vec3<f32> {
        Vec3::new(x, y, z)
    }
    let shape: Vec<_> = [((0.0, 0.0, 10.0),
                         (0.0, 1.0, 10.0),
                          (1.0, 0.0, 10.0)),

                         ((1.0, 1.0, 10.0),
                         (1.0, 0.0, 10.0),
                          (0.0, 1.0, 10.0)),

                         ((1.0, 0.0, 10.0),
                         (1.0, 1.0, 10.0),
                          (1.0, 1.0, 11.0)),

                         ((1.0, 1.0, 10.0),
                         (1.0, 0.0, 10.0),
                          (1.0, 0.0, 11.0)),

                         ((0.0, 0.0, 11.0),
                         (0.0, 1.0, 10.0),
                          (0.0, 0.0, 10.0)),

                         ((0.0, 1.0, 11.0),
                         (0.0, 1.0, 10.0),
                          (0.0, 0.0, 11.0)),

                         ((0.0, 0.0, 10.0),
                         (1.0, 0.0, 10.0),
                          (0.0, 0.0, 11.0)),

                         ((1.0, 0.0, 10.0),
                         (1.0, 0.0, 11.0),
                          (0.0, 0.0, 11.0)),
                         ]
        .iter()
        .flat_map(|&(a, b, c)| {
            let (va, vb, vc) = (to_vec3(a), to_vec3(b), to_vec3(c));
            let n = normalize(&cross(&(vb - va), &(vc - va)));
            vec![(va, n), (vb, n), (vc, n)].into_iter()
        }).map(|(v, n)| Vertex {
            position: [v.x, v.y, v.z],
            normal: [n.x, n.y, n.z]
        })
        .collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let mut view = Iso3::new(Vec3::new(0.0, 0.0, 0.0), Vec3::zero());
    let proj = PerspMat3::new(-1.0, f32::pi()*0.2, 0.001, 100.0);

    let shader = shader::Shader::new(vec!["shaders/vertex.vs", "shaders/vertex.frag"]);
    let program = glium::Program::from_source(&display, &shader.shaders[0], &shader.shaders[1], None).unwrap();
    let mut light_source: Vec3<f32>  = Vec3::new(-1.0,-0.5,0.0);


    'outer: loop {
        //view.prepend_rotation_mut(&Vec3::new(0.0, f32::pi() / 600.0, 0.0));
        //light_source = light_source + Vec3::new(0.01,0.0,0.0);
        let wvp: Mat4<f32> = proj.to_mat() * view.inv().unwrap().to_homogeneous();
        let uniforms = uniform! {
            wvp: wvp,
            tri_colour: [0f32, 1f32, 0.4f32],
            light_source: light_source,
        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.clear_depth(1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &DrawParameters {
                        depth_test: DepthTest::IfLess,
                        depth_write: true,
                        ..std::default::Default::default()
                    }).unwrap();
        target.finish().unwrap();
        if let Some(event) = manager.get_event(){
            match event{
                RawEvent::KeyboardEvent(_,  KeyId::Escape, State::Pressed)
                    => break 'outer,
                RawEvent::KeyboardEvent(_,  KeyId::Left, State::Pressed)
                    => view.prepend_translation_mut(&Vec3::new(-0.1, 0.0, 0.0)),
                RawEvent::KeyboardEvent(_,  KeyId::Right, State::Pressed)
                    => view.prepend_translation_mut(&Vec3::new(0.1, 0.0, 0.0)),
                RawEvent::KeyboardEvent(_,  KeyId::Up, State::Pressed)
                    => view.prepend_translation_mut(&Vec3::new(0.0, 0.0, 0.1)),
                RawEvent::KeyboardEvent(_,  KeyId::Down, State::Pressed)
                    => view.prepend_translation_mut(&Vec3::new(0.0, 0.0, -0.1)),
                RawEvent::MouseMoveEvent(_,  move_x,  move_y)
                    => {view.prepend_rotation_mut(&Vec3::new(0.0, (move_x as f32 * 0.01), 0.0));
                    view.prepend_rotation_mut(&Vec3::new((move_y as f32 * 0.01), 0.0, 0.0))},
                _ => (),
            }
        }
    }
}
