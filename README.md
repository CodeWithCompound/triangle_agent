# Triangle Agent (Rust + macroquad)
![screenshot](preview_triangle_agent.png)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://opensource.org/licenses/MIT)

This is a tiny chaos simulator I made while learning Rust and macroquad.  
The idea started as “let me draw a triangle” and somehow evolved into:

- a spinning killer triangle,
- a grid-based world,
- walls you can draw and erase,
- mouse-controlled movement,
- and a death message that bullies you.

I cannot explain how this happened. But it works. Mostly.

---

## 🕹️ What it does

- Draws a grid on screen  
- Lets you place/remove walls with **left click**  
- Spawns a spinning triangle ("the agent")  
- Triangle chases your mouse with cursed physics  
- If it touches you → **game over, cobber**  
- Walls block the triangle (unless the triangle is feeling spicy)

---

## 🎮 Controls

- **Mouse** → triangle chases your cursor  
- **Left click** → place/remove a wall    

If the triangle catches your mouse:  
You die. Don’t ask why. It’s the lore.

---

## 🧠 What I learned from this

- macroquad basics: drawing, input, frame timing  
- using `Vec2` like a normal person  
- grid indexing without summoning demons  
- simple collision detection  
- movement based on distance & direction  
- quitting the window (`miniquad::window::quit`)  
- Rust enums used as tiles (`Tile::Empty`, `Tile::Wall`)  

Basically: everything hurts but in a fun way.

---

## 🗂️ Code Structure

- `Tile` enum → defines grid cells (empty/wall)  
- `grid: Vec<Vec<Tile>>` → 2D world  
- Mouse click → toggle tile  
- Triangle movement → normalized direction, speed scaling with distance  
- Collision → triangle refuses to walk through walls (good boy)

---

## 🚀 How to run
- Install Rust: https://rustup.rs

Make sure your Cargo.toml has the dependency:
macroquad = "*"

Run the project with:
cargo run --release














Run the game: cargo ru
