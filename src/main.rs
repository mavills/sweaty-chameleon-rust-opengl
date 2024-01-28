#[macro_use]
extern crate glium;
use std::{any::Any, u32};

use glium::{texture::ClientFormat, Surface};

// #![allow(unused_variables)]
// fn main() {
// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 2],
// }

// implement_vertex!(Vertex, position);
// }

fn main() {
    use freetype::face::LoadFlag;
    use freetype::Library;

    // Init the library
    let lib = Library::init().unwrap();
    // Load a font face
    let face = lib
        .new_face(
            "/home/stijn/documents/repositories/imgui/misc/fonts/DroidSans.ttf",
            0,
        )
        .unwrap();
    // Set the font size
    face.set_char_size(40 * 64, 40 * 64, 50, 50).unwrap();
    // Load a character
    face.load_char('A' as usize, LoadFlag::RENDER).unwrap();
    // Get the glyph instance
    let glyph = face.glyph();
    // do_something_with_bitmap(glyph.bitmap());

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);
    let vertex1 = Vertex {
        // bottom left
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        // top left
        position: [-0.5, 0.5],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        // top right
        position: [0.5, 0.5],
        tex_coords: [1.0, 1.0],
    };
    let vertex4 = Vertex {
        //bottom right
        position: [0.5, -0.5],
        tex_coords: [1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #1")
        .build(&event_loop);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleStrip,
        &[1 as u8, 0, 2, 3],
    )
    .unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform float x_off;

        void main() {
            v_tex_coords = tex_coords;
            vec2 pos = position;
            pos.x += x_off;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            vec4 texColor = texture(tex, v_tex_coords);
            // if(texColor.x < 0.001)
            //     discard;
            // color = vec4(1.0);
            color = vec4(texColor.rrr, texture(tex, v_tex_coords).x);
        }
    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    let bitmap = glyph.bitmap();

    let mut image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &bitmap.buffer(),
        (bitmap.width() as u32, bitmap.rows() as u32),
    );
    println!("{:?}", bitmap.buffer());
    image.format = ClientFormat::U8;
    let texture = glium::Texture2d::new(&display, image).unwrap();
    // let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    // Start rendering by creating a new frame
    let mut frame = display.draw();
    // Which we fill with an opaque blue color
    frame.clear_color(0.0, 0.0, 1.0, 1.0);

    frame
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    // By finishing the frame swap buffers and thereby make it visible on the window
    frame.finish().unwrap();

    let mut t: f32 = 0.0;
    // Now we wait until the program is closed
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    winit::event::WindowEvent::RedrawRequested => {
                        // first we update `t`
                        t += 0.02;
                        let x_off = t.sin() * 0.5;

                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 1.0, 1.0);
                        let uniforms = uniform! { 
                            x_off: x_off, tex: glium::uniforms::Sampler::new(&texture).magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),};
                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &uniforms,
                                &Default::default(),
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            };
        })
        .unwrap();
}
