use std::{io, iter};

use freya::prelude::*;
use gilrs::{Gilrs, Button, Event, EventType};
use io_bluetooth::bt::{self, BtStream};
use tokio::sync::mpsc::unbounded_channel;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let event_handlers = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<Event>();

        (tx, Some(rx))
    });
    let event = use_state(&cx, String::new);
    let event_setter = event.setter();

    let moving = use_state(&cx, || false);
    let moving_setter = moving.setter();

    use_effect(&cx, (), move |_| {
        let sender =  event_handlers.0.clone();
        let mut rx =  event_handlers.1.take().unwrap();

        async move {
            tokio::task::spawn_blocking(move || {

                // CONTROLLER
                let mut gilrs = Gilrs::new().unwrap();
    
                // Iterate over all connected gamepads
                for (_id, gamepad) in gilrs.gamepads() {
                    println!("{} is {:?}", gamepad.name(), gamepad.power_info());
                }
            
                let mut active_gamepad = None;
            
                loop {
                    // Examine new events
                    while let Some(event) = gilrs.next_event() {
                        sender.send(event).unwrap();
                        active_gamepad = Some(event.id);
                    }
            
                    // You can also use cached gamepad state
                    if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                        if gamepad.is_pressed(Button::South) {
                            println!("Button South is pressed (XBox - A, PS - X)");
                        }
                    }
                }
            });

            // BLUETOOTH
            let devices = bt::discover_devices().unwrap();
            println!("Devices:");
            for (idx, device) in devices.iter().enumerate() {
                println!("{}: {}", idx, *device);
            }
        
            if devices.len() == 0 {
                panic!("No Bluetooth devices found.");
            }
        
            let device_idx = request_device_idx(devices.len()).unwrap();
        
            let socket = BtStream::connect(iter::once(&devices[device_idx]), bt::BtProtocol::RFCOMM).unwrap();
        
            match socket.peer_addr() {
                Ok(name) => println!("Peername: {}.", name.to_string()),
                Err(err) => println!("An error occured while retrieving the peername: {:?}", err),
            }
        
            match socket.local_addr() {
                Ok(name) => println!("Socket name: {}", name.to_string()),
                Err(err) => println!("An error occured while retrieving the sockname: {:?}", err),
            }

            while let Some(Event { id, event, time }) = rx.recv().await {
                let data = format!("{:?} New event from {}: {:?}", time, id, event);
                match event {
                    EventType::ButtonChanged(btn, _pressed, _code) => {
                        match btn {
                            Button::LeftTrigger => {
                                socket.send("F".as_bytes()).unwrap();
                                moving_setter(true);
                            }
                            Button::LeftTrigger2 => {
                                socket.send("S".as_bytes()).unwrap();
                                moving_setter(false);
                            }
                            _ => {
                                println!("wrong button 1")
                            }
                        }
                    }
                    _ => {
                        println!("wrong button 0")
                    }
                }
                println!("{data}");
                event_setter(data);
            }
        }
    });

    let (txt, bg) = if *moving.get() {
        ("MOVING", "green")
    } else {
        ("STOPPED", "red")
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            display: "center",
            direction: "both",
            background: "{bg}",
            label {
                color: "white",
                font_size: "70",
                font_style: "bold",
                "{txt}"
            }
        }
    )
}

fn request_device_idx(len: usize) -> io::Result<usize> {
    println!("Please specify the index of the Bluetooth device you want to connect to:");

    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if let Ok(idx) = buffer.trim_end().parse::<usize>() {
            if idx < len {
                return Ok(idx);
            }
        }
        buffer.clear();
        println!("Invalid index. Please try again.");
    }
}