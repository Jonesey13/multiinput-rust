#[macro_use]
extern crate glium;
extern crate multiinput;
extern crate time;

mod shader;
use multiinput::*;
use multiinput::RawEvent::*;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event::{Closed,KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::{DisplayBuild, Surface};
use glium::draw_parameters::LinearBlendingFactor;
use glium::draw_parameters::BlendingFunction;
use glium::glutin::ElementState::Pressed;
use std::fs::File;
use std::path::Path;



fn main(){
    let mut handler = Handler::new();

    let start_time = time::precise_time_s();
    let mut current_time = time::precise_time_s() - start_time;
    let mut exit: bool = false;
    while exit == false {
        handler.update_rendering();
        exit = handler.update_input();
        current_time = time::precise_time_s() - start_time;
    }
}

pub struct Handler<'a>{
    display: GlutinFacade,
    vertex_buffer: glium::VertexBuffer<Vertices>,
    program: glium::Program,
    draw_param: glium::draw_parameters::DrawParameters<'a>,
    players: Players,
    input_manager: RawInputManager,
}


impl<'a> Handler<'a>{

    pub fn new() -> Handler<'a>{
        let display = if cfg!(any(target_os = "macos", target_os = "windows")){
            glium::glutin::WindowBuilder::new().with_fullscreen(glium::glutin::get_primary_monitor()).build_glium().unwrap()
        }
        else{
            glium::glutin::WindowBuilder::new().with_dimensions(800,600).build_glium().unwrap()
        };

        implement_vertex!(Vertices, square, color);

        let shader = shader::Shader::new(vec!["shaders/square.vs", "shaders/square.frag", "shaders/square.geom"]);

        let program = glium::Program::from_source(&display, &shader.shaders[0], &shader.shaders[1], Some(&shader.shaders[2])).unwrap();

        let mut draw_param =  glium::draw_parameters::DrawParameters::default();
        draw_param.blending_function = Some( BlendingFunction::Addition{source: LinearBlendingFactor::SourceAlpha,
                                                                        destination:  LinearBlendingFactor::OneMinusSourceAlpha});

        let mut input_manager = RawInputManager::new().unwrap();
        input_manager.register_devices(DeviceType::Mice);
        input_manager.register_devices(DeviceType::Keyboards);
        input_manager.register_devices(DeviceType::Joysticks);
        let total_mice = input_manager.get_device_stats().number_of_mice;
        let mut vertices: Vec<Part> = Vec::new();
        for _ in 0..(total_mice){
            vertices.push(Part{pos:[0.0,0.0], color: [1.0,1.0,1.0,1.0]});
        }
        let players = Players{players: vertices};



        Handler{
            vertex_buffer: glium::VertexBuffer::empty(&display, 0).unwrap(),
            display: display,
            program: program,
            draw_param: draw_param,
            players: players,
            input_manager: input_manager,
        }
    }


    pub fn update_rendering(&mut self){
        let (w, h) = self.display.get_framebuffer_dimensions();
        let aspect_ratio = (w as f32) / (h as f32);
        let uniforms = uniform! {
            aspect_ratio: aspect_ratio,
        };

        let vertices: Vec<Part>  = self.players.players.clone();
        let shape: Vec<Vertices> = vertices.iter().map(|p| Vertices { square: [p.pos[0], p.pos[1], 0.01],
                                                                      color: p.color}).collect();
        self.vertex_buffer = glium::VertexBuffer::dynamic(&self.display, &shape).unwrap();

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &self.program,
                    &uniforms,
                    &self.draw_param)
            .unwrap();
        target.finish().unwrap();
    }

    pub fn update_input(&mut self) -> bool{
        while let Some(event) = self.input_manager.get_event(){
            match event{
                MouseMoveEvent(id, move_x, move_y) => {
                    self.players.players[id].pos[0] = self.players.players[id].pos[0] + (move_x as f64)/100.0;
                    self.players.players[id].pos[1] = self.players.players[id].pos[1] - (move_y as f64)/100.0;},
                KeyboardEvent(_,Escape,_) => return true,
                _  => (),
            }
        }
        return false;
    }

}

#[derive(Copy, Clone)]
pub struct Vertices {
    square: [f64; 3],
    color: [f64; 4]
}

#[derive(Clone)]
pub struct Players {
    players: Vec<Part>,
}

#[derive(Copy, Clone)]
pub struct Part{
    pos: [f64; 2],
    color: [f64; 4],
}
