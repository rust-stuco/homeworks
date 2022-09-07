use glium::glutin;
use glium::Surface;

mod ppm;
mod ppm_refsol;

use crate::ppm_refsol::parse;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_coords);

fn main() {
    // 1. Load image from filename on command line
    let args: Vec<String> = std::env::args().collect();
    let image_filename = args.get(1).map(String::to_owned).unwrap_or(String::from("image.ppm"));
    let raw_image_bytes =
        std::fs::read(&image_filename).expect(&format!("Could not read file `{}`", image_filename));

    // 2. Parse the image (this is your code!)
    let image = parse(&raw_image_bytes).unwrap();

    // 3. Initialize the OpenGL context
    // Sample initialization from https://docs.rs/glium/latest/glium/
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(
            image.width,
            image.height,
        ))
        .with_title(image_filename);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // 4. Load the image as a texture and draw it
    let program =
        glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let image_texture = glium::texture::SrgbTexture2d::new(
        &display,
        glium::texture::RawImage2d {
            data: std::borrow::Cow::<'static, _>::Owned(image.pixels),
            width: image.width,
            height: image.height,
            format: glium::texture::ClientFormat::U8U8U8,
        },
    )
    .unwrap();

    let shape = vec![
        // Triangle 1
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        // Triangle 2
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = glium::uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &image_texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}

// Shaders to draw an image from https://glium.github.io/glium/book/tuto-06-texture.html
const VERTEX_SHADER: &str = r#"#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 140
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}
"#;

/// Testbench module
#[cfg(test)]
mod tests {
    use super::*;

    static TESTCASES: [(&[u8], (u32, u32)); 4] = [
        (include_bytes!("../image.ppm"), (100, 200)),
        (include_bytes!("../tiny_image.ppm"), (3, 3)),
        (include_bytes!("../lots_of_comments.ppm"), (165, 121)),
        (include_bytes!("../big_image.ppm"), (416, 600)),
    ];

    #[test]
    fn test_parse_good_images() {
        for (bytes, (expected_width, expected_height)) in TESTCASES.iter() {
            let parsed = parse(bytes).unwrap();
            assert_eq!(parsed.width, *expected_width);
            assert_eq!(parsed.height, *expected_height);
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_bad_magic() {
        let _ = parse(b"P7\n3 3\n255\nabcabcabcdefdefdefghighighi").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_bad_width() {
        let _ = parse(b"P6\na 3\n255\nabcabcabcdefdefdefghighighi").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_bad_height() {
        let _ = parse(b"P6\n3 b\n255\nabcabcabcdefdefdefghighighi").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_bad_maxnum() {
        let _ = parse(b"P6\n3 3\nc55\nabcabcabcdefdefdefghighighi").unwrap();
    }
}
