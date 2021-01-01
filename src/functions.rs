use jack::*;

const COMMANDS: [&str; 3] = [
    "list ports",
    "connect ports () ()",
    "disconnect ports () ()",
];

pub fn list_commands() -> Control {
    println!("{:?}", COMMANDS);
    Control::Continue
}

pub fn list_ports(c: &Client) -> Control {
    let ports = c.ports(None, None, PortFlags::empty());
    for portid in 1..=ports.len() {
        let port = c.port_by_id(portid as u32).unwrap();
        println!("port {}: {}", portid, port.name().unwrap());
    }
    Control::Continue
}

pub fn connect_ports(c: &Client, command: &str) -> Control {
    let ids: Vec<u32> = command[14..]
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let port1 = c.port_by_id(ids[0]);
    let port2 = c.port_by_id(ids[1]);

    if port1.is_none() {
        println!("port {} could not be found", ids[0]);
        return Control::Continue;
    }
    if port2.is_none() {
        println!("port {} could not be found", ids[1]);
        return Control::Continue;
    }
    let connect_result = c.connect_ports(&port1.unwrap(), &port2.unwrap());
    if connect_result.is_err() {
        println!("could not connect because of {:?}", connect_result);
    }
    Control::Continue
}

pub fn disconnect_ports(c: &Client, command: &str) -> Control {
    let ids: Vec<u32> = command[17..]
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let port1 = c.port_by_id(ids[0]);
    let port2 = c.port_by_id(ids[1]);

    if port1.is_none() {
        println!("port {} could not be found", ids[0]);
        return Control::Continue;
    }
    if port2.is_none() {
        println!("port {} could not be found", ids[1]);
        return Control::Continue;
    }

    let connect_result = c.disconnect_ports(&port1.unwrap(), &port2.unwrap());
    if connect_result.is_err() {
        println!("could not disconnect because of {:?}", connect_result);
    }
    Control::Continue
}
