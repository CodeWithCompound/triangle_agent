use macroquad::{miniquad::window::quit, prelude::{*}};

#[macroquad::main("i have no clue what im doing help")]
async fn main() {
    let mut poly_r: f32 = 0.0;
    let mut cobber_pos: Vec2 = vec2(100.0, 100.0);
    let spacing: f32 = 40.0;

    const GRID_W: usize = 30;
    const GRID_H: usize = 20;
   #[derive(Clone, Copy)]
    enum Tile {
        Empty,
        Wall,
    }

let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Empty; GRID_W]; GRID_H];

    loop {
        clear_background(BLACK);

        let sw: f32 = screen_width();
        let sh: f32 = screen_height();
        let spacing_i: i32 = spacing as i32;

        // grid stuff
        for x in (0..sw as i32).step_by(spacing_i as usize) {
            draw_line(x as f32, 0.0, x as f32, sh, 1.0, GRAY);
        }

        for y in (0..sh as i32).step_by(spacing_i as usize) {
            draw_line(0.0, y as f32, sw, y as f32, 1.0, GRAY);
        }

        // 
        for gy in 0..GRID_H {
            for gx in 0..GRID_W {
                if let Tile::Wall = grid[gy][gx] {
                    let x = gx as f32 * spacing;
                    let y = gy as f32 * spacing;
                    draw_rectangle(x, y, spacing, spacing, DARKGRAY);
                    draw_rectangle_lines(x, y, spacing, spacing, 2.0,RED);
                }
            }
        }

        // felt like reinventing gravity ngl
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = Vec2::from(mouse_position());
            let gx = (mouse.x / spacing).floor() as i32;
            let gy = (mouse.y / spacing).floor() as i32;

            if gx >= 0 && gx < GRID_W as i32 && gy >= 0 && gy < GRID_H as i32 {
                let gx_u = gx as usize;
                let gy_u = gy as usize;

                grid[gy_u][gx_u] = match grid[gy_u][gx_u] {
                    Tile::Empty => Tile::Wall,
                    Tile::Wall => Tile::Empty,
                };
            }
        }

        // makes the triangle spin
        poly_r = (poly_r + 5.0) % 360.0;

        let mouse_pos = Vec2::from(mouse_position());

        // if position is not mouse then change that 
let mut direction: Vec2 = mouse_pos - cobber_pos;
let distance = direction.length();

// collision radius (triangle radius)
let hit_radius = 10.0;

if distance <= hit_radius {
    // you're dead
    println!("");
    println!("-----> you died because the triangle touched you!");
    println!("-----> hint: don't touch the triangle :)");
    println!("");
    quit();
} else if distance > 0.0 {
    direction /= distance;

    let speed = 100.0 + distance / 5.0;
    let dt = get_frame_time();

    let movement = direction * speed * dt;
    let new_pos = cobber_pos + movement;

    let nx = (new_pos.x / spacing).floor() as i32;
    let ny = (new_pos.y / spacing).floor() as i32;

    let mut blocked = false;

    if nx >= 0 && nx < GRID_W as i32 && ny >= 0 && ny < GRID_H as i32 {
        if let Tile::Wall = grid[ny as usize][nx as usize] {
            blocked = true;
        }
    }

    if !blocked {
        cobber_pos = new_pos;
    }
}


        // here we draw the triangle
        draw_poly(cobber_pos.x, cobber_pos.y, 3, 20.0, poly_r, BLACK);
        draw_poly_lines(cobber_pos.x, cobber_pos.y, 3, 20.0, poly_r, 5.0, SKYBLUE);

        // here we draw the info thats being displayed
        draw_text(
            &format!("mouse: {:.0}, {:.0}   FPS: {:.0}", mouse_pos.x, mouse_pos.y, get_fps()),
            20.0,
            40.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}
// this little project was fun but also very painful