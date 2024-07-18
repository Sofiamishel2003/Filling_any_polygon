mod color;
mod framebuffer;
mod bmp;

use framebuffer::Framebuffer;
use nalgebra_glm::Vec3;
use color::Color;

/// Dibuja un polígono conectando los vértices dados con líneas.
/// Los vértices deben proporcionarse en el orden en que se deben conectar.
fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3], line_color: Color) {
    if vertices.len() < 3 {
        println!("Se necesitan al menos 3 vértices para dibujar un polígono");
        return;
    }

    framebuffer.set_current_color(line_color.to_hex());

    for i in 0..vertices.len() {
        let next = (i + 1) % vertices.len();
        let start = vertices[i];
        let end = vertices[next];

        // Convierte de glm::Vec3 a (isize, isize) para el dibujo del framebuffer
        framebuffer.line(
            start.x as isize,
            start.y as isize,
            end.x as isize,
            end.y as isize,
        );
    }
}

/// Rellena un polígono usando el algoritmo de escaneo de líneas.
fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3], fill_color: Color) {
    if vertices.len() < 3 {
        println!("Se necesitan al menos 3 vértices para llenar un polígono");
        return;
    }

    let mut edges: Vec<(isize, isize, isize, isize)> = Vec::new();

    for i in 0..vertices.len() {
        let next = (i + 1) % vertices.len();
        let start = vertices[i];
        let end = vertices[next];
        edges.push((start.x as isize, start.y as isize, end.x as isize, end.y as isize));
    }

    let min_y = vertices.iter().map(|v| v.y as isize).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y as isize).max().unwrap();

    framebuffer.set_current_color(fill_color.to_hex());

    for y in min_y..=max_y {
        let mut intersections: Vec<isize> = Vec::new();
        for &(x1, y1, x2, y2) in &edges {
            if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }
        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                framebuffer.line(intersections[i], y, intersections[i + 1], y);
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Limpia el framebuffer con un fondo blanco
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Define los vértices del polígono
    let vertices = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    // Define los colores de línea y relleno
    let line_color = Color::new(255, 255, 255); // Blanco
    let fill_color = Color::new(255, 255, 0); // Amarillo

    // Dibuja el polígono
    draw_polygon(&mut framebuffer, &vertices, line_color);

    // Rellena el polígono
    fill_polygon(&mut framebuffer, &vertices, fill_color);

    // Verifica el color de un punto
    if let Some(color) = framebuffer.get_point_color(400, 300) {
        println!("Color en (400, 300): {:06X}", color);
    }

    // Prueba los métodos from_hex y multiply
    let color = Color::from_hex(0xFF5733);
    let multiplied_color = color.multiply(1.5);
    println!("Color multiplicado: {:?}", multiplied_color);

    // Guarda el framebuffer como un archivo BMP
    let result = framebuffer.render_buffer("output.bmp");
    if result.is_err() {
        println!("Error al renderizar el framebuffer: {:?}", result);
    } else {
        println!("Framebuffer renderizado a output.bmp");
    }
}
