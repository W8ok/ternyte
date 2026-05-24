use crate::{
    components::{base::*, gate::*, ui::*},
    input,
    sdl::{
        Sdl,
        camera::Camera,
        event::{Key, MouseButton},
    },
};
use hecs::*;

mod gates;
pub const GRID_SIZE: f32 = 16.0;

pub fn new(sdl: &mut Sdl, world: &mut World) {
    let (width, height) = sdl.get_window_size();

    const GATES: [(GateType, &str, ButtonAction); 8] = [
        (GateType::AND, "AND", ButtonAction::AddGateAND),
        (GateType::OR, "OR", ButtonAction::AddGateOR),
        (GateType::XOR, "XOR", ButtonAction::AddGateXOR),
        (GateType::NOT, "NOT", ButtonAction::AddGateNOT),
        (GateType::NAND, "NAND", ButtonAction::AddGateNAND),
        (GateType::NOR, "NOR", ButtonAction::AddGateNOR),
        (GateType::XNOR, "XNOR", ButtonAction::AddGateXNOR),
        (GateType::BUF, "BUF", ButtonAction::AddGateBUF),
    ];
    for (n, (gate_type, label, action)) in GATES.iter().enumerate() {
        world.spawn((
            Button,
            Rect {
                x: 20.,
                y: 20. + 95. * n as f32,
                w: 200.,
                h: 75.,
            },
            *action,
            Color::DARKGRAY,
            Text(label),
            Editor,
            Ui,
        ));
    }

    world.spawn((
        Button,
        Rect {
            x: 20.,
            y: height as f32 - 95.,
            w: 200.,
            h: 75.,
        },
        ButtonAction::Exit,
        Color::DARKGRAY,
        Text("Exit"),
        Editor,
        Ui,
    ));
}

pub fn placement(world: &mut World) {
    gates::place_gate(world);
    gates::select_gate(world);
}

fn camera_movement(camera: &mut Camera) {
    // Perhaps i should remove the keyboard movement...
    let speed = if input::key_pressed(Key::Shift) {
        20.0
    } else {
        10.0
    };

    if input::key_pressed(Key::W) {
        camera.y -= speed / camera.zoom;
    }
    if input::key_pressed(Key::S) {
        camera.y += speed / camera.zoom;
    }
    if input::key_pressed(Key::A) {
        camera.x -= speed / camera.zoom;
    }
    if input::key_pressed(Key::D) {
        camera.x += speed / camera.zoom;
    }

    static mut LAST_POS: (f32, f32) = (0.0, 0.0);

    if input::mouse_pressed(MouseButton::Middle) {
        let pos = input::mouse_pos();
        unsafe {
            camera.x -= (pos.x - LAST_POS.0) / camera.zoom;
            camera.y -= (pos.y - LAST_POS.1) / camera.zoom;
            LAST_POS = (pos.x, pos.y);
        }
    } else {
        let pos = input::mouse_pos();
        unsafe {
            LAST_POS = (pos.x, pos.y);
        }
    }
}

#[rustfmt::skip]
pub fn interact(sdl: &mut Sdl, world: &mut World) -> bool {
    camera_movement(&mut sdl.camera);

    let result = world
        .query::<(Entity, &ButtonAction)>()
        .with::<(&Button, &Interacted, &Editor)>()
        .into_iter()
        .next()
        .map(|(button, action)| (button, *action));

    if let Some((button, action)) = result {
        world.remove_one::<Interacted>(button).unwrap();

        match action {
            ButtonAction::Exit => return false,
            ButtonAction::AddGateAND => gates::add_gate(GateType::AND, 2, Text("AND"), world),
            ButtonAction::AddGateOR => gates::add_gate(GateType::OR, 2, Text("OR"), world),
            ButtonAction::AddGateXOR => gates::add_gate(GateType::XOR, 2, Text("XOR"), world),
            ButtonAction::AddGateNOT => gates::add_gate(GateType::NOT, 1, Text("NOT"), world),
            ButtonAction::AddGateNAND => gates::add_gate(GateType::NAND, 2, Text("NAND"), world),
            ButtonAction::AddGateNOR => gates::add_gate(GateType::NOR, 2, Text("NOR"), world),
            ButtonAction::AddGateXNOR => gates::add_gate(GateType::XNOR, 2, Text("XNOR"), world),
            ButtonAction::AddGateBUF => gates::add_gate(GateType::BUF, 1, Text("BUF"), world),
        }
    }

    return true;
}

pub fn hover(world: &mut World) {
    let pos = input::mouse_pos();

    for (color, button) in world
        .query_mut::<(&mut Color, &Rect)>()
        .with::<(&Button, &Editor)>()
        .into_iter()
    {
        if button.contains(pos.x, pos.y) {
            *color = Color::GRAY;
        } else {
            *color = Color::DARKGRAY;
        }
    }
}

pub fn click(world: &mut World) {
    static mut CLICKED: bool = false;
    let pos = input::mouse_pos();
    if input::mouse_pressed(MouseButton::Left) {
        unsafe { CLICKED = true };

        for (color, button) in world
            .query_mut::<(&mut Color, &Rect)>()
            .with::<(&Button, &Editor)>()
            .into_iter()
        {
            if button.contains(pos.x, pos.y) {
                *color = Color::LIGHTGRAY;
            }
        }
    } else if unsafe { CLICKED } {
        unsafe { CLICKED = false };

        let to_interact: Vec<Entity> = world
            .query::<(Entity, &Rect)>()
            .with::<(&Button, &Editor)>()
            .into_iter()
            .filter_map(|(entity, button)| button.contains(pos.x, pos.y).then_some(entity))
            .collect();

        for entity in to_interact {
            world.insert(entity, (Interacted,)).unwrap();
        }
    }
}

fn grid(sdl: &mut Sdl) {
    sdl.render.color(&Color::DARKGRAY);

    let (w, h) = sdl.get_window_size();
    let camera = &sdl.camera;

    let left = camera.x;
    let top = camera.y;
    let right = left + w as f32 / camera.zoom;
    let bottom = top + h as f32 / camera.zoom;

    // I have no real idea wtf this does... :3
    let spacing = GRID_SIZE;
    let snap = |v: f32| (v / spacing).floor() * spacing;

    let mut x = snap(left);
    while x <= right {
        sdl.render.line(x, top, x, bottom);
        x += spacing;
    }

    let mut y = snap(top);
    while y <= bottom {
        sdl.render.line(left, y, right, y);
        y += spacing;
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    sdl.camera.start();
    {
        grid(sdl);
        gates::render(sdl, world);
    }
    sdl.camera.end();

    let (w, h) = sdl.get_window_size();
    let side_panel = Rect {
        x: 0.0,
        y: 0.0,
        w: 240.0,
        h: h as f32,
    };

    sdl.render.color(&Color::DARKGRAY);
    sdl.render.rect(&side_panel);
    sdl.render.color(&Color::BLACK);
    sdl.render.rect_line(&side_panel);

    sdl.text.color(Color::WHITE);

    for (rect, color, text) in world
        .query::<(&Rect, &Color, &Text)>()
        .with::<(&Button, &Editor)>()
        .iter()
    {
        sdl.render.color(color);
        sdl.render.rect(rect);
        sdl.render.color(&Color::BLACK);
        sdl.render.rect_line(rect);

        const PADDING: f32 = 10.0;
        sdl.text.size(rect.h - PADDING * 4.);
        sdl.text
            .render(text.0, rect.x + PADDING, rect.y + PADDING * 2.);
    }
}
