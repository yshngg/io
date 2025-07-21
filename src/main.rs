use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
    winit::WinitSettings,
};
use rdev::{Event, EventType, listen};
use std::thread;

// use windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExA;

fn main() {
    // SetWindowsHookExA();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(WinitSettings::game())
        .add_systems(Update, keyboard_events)
        .add_systems(Update, game_loop)
        .run();
}
fn game_loop() {
    println!("loop...");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    thread::spawn(|| {
        // This will block.
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.event_type {
        EventType::KeyPress(key) => println!("KeyPress: {:?}", key),
        EventType::KeyRelease(key) => println!("KeyRelease: {:?}", key),
        _ => (),
    };
}

fn keyboard_events(mut commands: Commands, mut evr_kbd: EventReader<KeyboardInput>) {
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                let key_text = format!("{:?}", ev.key_code);
                commands.spawn(Text::new(key_text));
                println!("Key press: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({:?})", ev.key_code, ev.logical_key);
            }
        }
    }
}
