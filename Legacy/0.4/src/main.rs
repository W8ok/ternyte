#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Makes windows not open the cmd
#![allow(dead_code, unused)]

use sdl3::{keyboard::Keycode, mouse::MouseButton, pixels::Color};

mod sdl;
use sdl::{SDL, resource::Resource};

mod input;
use input::Input;

mod components;
use components::*;

mod utils;

// TODO
// Refactor everything SDL to make a more monolithic architecture
// Side panels with selection logic
// Reorganize sdl module

#[repr(u8)]
pub enum GateType {
    Switch,
    Button,
    Buffer,
    NOT,
    AND,
    NAND,
    XOR,
    XNOR,
    OR,
    NOR,
}

pub struct Gate {
    pub pos: Position,
    typ: GateType, // type was taken :c
                   //output: bool,
                   //inputs: u8,
                   //id: usize,
                   //output_net: usize,
}

struct Wire {
    start: Position,
    end: Position,
    id: usize,
}

struct WireNet {
    wires: Vec<Wire>,
    connected_inputs: Vec<(usize, u8)>, // GateID and input bits
    state: bool,
}

struct Circuit {
    gates: Vec<Gate>,
}

impl Circuit {
    pub fn add_gate(&mut self, typ: GateType, pos: Position) {
        self.gates.push(Gate { pos, typ });
    }
}

const SPRITE_SIZE: f32 = 32.0;

fn main() {
    let mut sdl = SDL::init();
    sdl.render.resource = &mut sdl.resource as *mut Resource;
    let mut input = Input::new(&sdl.render);
    let mut cir = Circuit { gates: Vec::new() };

    sdl.resource.load_texture("assets/gates.png");
    sdl.resource.load_font("assets/font.ttf");

    log!(info, "Initialization Complete");

    loop {
        if !sdl.event.handle(&mut input, &mut sdl.render.window) {
            break;
        }

        sdl.render.camera.update(&input);

        if input.is_mouse_clicked(MouseButton::Left) {
            let pos = input.get_mouse_pos();
            let snapped_pos = Position::new(
                (pos.x / (SPRITE_SIZE / 2.0)).floor() * (SPRITE_SIZE / 2.0),
                (pos.y / (SPRITE_SIZE / 2.0)).floor() * (SPRITE_SIZE / 2.0),
            );
            cir.add_gate(GateType::NAND, snapped_pos);
        }

        if input.is_mouse_clicked(MouseButton::Right) {
            let pos = input.get_mouse_pos();
            let snapped_pos = Position::new(
                (pos.x / (SPRITE_SIZE / 2.0)).floor() * (SPRITE_SIZE / 2.0),
                (pos.y / (SPRITE_SIZE / 2.0)).floor() * (SPRITE_SIZE / 2.0),
            );
            cir.add_gate(GateType::NOT, snapped_pos);
        }

        sdl.render.set_color(Color::BLACK);
        sdl.render.clear();

        sdl.render.set_color(Color::RED);
        sdl.gui.grid(&mut sdl.render);

        // Render loop
        for gate in &cir.gates {
            let sprite: Position = match gate.typ {
                GateType::Buffer => Position::new(0.0 * SPRITE_SIZE, 0.0 * SPRITE_SIZE),
                GateType::NOT => Position::new(0.0 * SPRITE_SIZE, 1.0 * SPRITE_SIZE),
                GateType::AND => Position::new(1.0 * SPRITE_SIZE, 0.0 * SPRITE_SIZE),
                GateType::NAND => Position::new(1.0 * SPRITE_SIZE, 1.0 * SPRITE_SIZE),
                GateType::OR => Position::new(2.0 * SPRITE_SIZE, 0.0 * SPRITE_SIZE),
                GateType::NOR => Position::new(2.0 * SPRITE_SIZE, 1.0 * SPRITE_SIZE),
                GateType::XOR => Position::new(3.0 * SPRITE_SIZE, 0.0 * SPRITE_SIZE),
                GateType::XNOR => Position::new(3.0 * SPRITE_SIZE, 1.0 * SPRITE_SIZE),
                _ => Position::zero(),
            };
            sdl.render.texture(
                &sdl.resource.textures[0],
                Some(&Rect::new(sprite.x, sprite.y, SPRITE_SIZE, SPRITE_SIZE)),
                &Rect::new(gate.pos.x, gate.pos.y, SPRITE_SIZE, SPRITE_SIZE),
            );
        }

        sdl.render
            .text("Hello", Position { x: 200.0, y: 200.0 }, 64.0, Color::GREEN);

        sdl.render.present();
    }

    log!(info, "Exiting");
}
