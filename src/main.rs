use jack::*;
use raspjack::*;
use std::io;

fn main() {
    // Create client
    let (client, _status) = Client::new("raspjack", ClientOptions::NO_START_SERVER).unwrap();

    // Register new ports.
    let in_left_port = client
        .register_port("rust_in_l", AudioIn::default())
        .unwrap();
    let in_right_port = client
        .register_port("rust_in_r", AudioIn::default())
        .unwrap();
    let mut out_left_port = client
        .register_port("rust_out_l", AudioOut::default())
        .unwrap();
    let mut out_right_port = client
        .register_port("rust_out_r", AudioOut::default())
        .unwrap();

    let process_callback = move |_: &Client, ps: &ProcessScope| -> Control {
        let out_left_signal = out_left_port.as_mut_slice(ps);
        let out_right_signal = out_right_port.as_mut_slice(ps);
        let in_left_signal = as_slice_mut(&in_left_port, ps);
        let in_right_signal = as_slice_mut(&in_right_port, ps);
        process_signal(in_right_signal);
        process_signal(in_right_signal);
        out_left_signal.copy_from_slice(&in_left_signal);
        out_right_signal.copy_from_slice(&in_right_signal);
        Control::Continue
    };
    let process = ClosureProcessHandler::new(process_callback);

    // activate the client and start the processing
    let active_client = client.activate_async(Notifications, process).unwrap();
    let c = active_client.as_client();
    println!("");

    // keep reading user commands and take appropriate action
    let mut user_input = String::new();

    loop {
        io::stdin().read_line(&mut user_input).ok();
        process_commands(c, &user_input.trim());
        println!("");
        user_input = String::new();
    }
}

fn process_commands(c: &Client, command: &str) {
    match command {
        "help" => list_commands(),
        "list ports" => list_ports(c),
        "list flags" => list_flags(c),
        "list inputs" => list_inputs(c),
        "list outputs" => list_outputs(c),
        "list ports full" => list_ports_full(c),
        "list connections" => list_connections(c),
        _ if command.starts_with("connect") => connect_ports(c, command),
        _ if command.starts_with("disconnect") => disconnect_ports(c, command),
        _ => {
            println!("{} is invalid command", command);
        }
    }
}

fn process_signal(s: &mut [f32]) {
    s.iter_mut().for_each(|x| *x *= 1.5);
}
