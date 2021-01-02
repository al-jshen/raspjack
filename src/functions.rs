use jack::*;

const COMMANDS: [&str; 8] = [
    "list ports",
    "list ports full",
    "list inputs",
    "list outputs",
    "list connections",
    "list flags",
    "connect () ()",
    "disconnect () ()",
];

pub fn list_commands() {
    println!("{:?}", COMMANDS);
}

fn get_n_ports(c: &Client) -> usize {
    c.ports(None, None, PortFlags::empty()).len()
}

fn get_ports(c: &Client) -> Vec<Port<Unowned>> {
    (1..=get_n_ports(c))
        .map(|id| c.port_by_id(id as u32).unwrap())
        .collect()
}

fn get_input_ports(c: &Client) -> Vec<Port<Unowned>> {
    get_ports(c)
        .into_iter()
        .filter(|p| p.flags().contains(jack::PortFlags::IS_INPUT))
        .collect::<Vec<_>>()
}

fn get_output_ports(c: &Client) -> Vec<Port<Unowned>> {
    get_ports(c)
        .into_iter()
        .filter(|p| p.flags().contains(jack::PortFlags::IS_INPUT))
        .collect::<Vec<_>>()
}

fn get_input_ids(c: &Client) -> Vec<usize> {
    let ports = get_ports(c);
    (0..ports.len())
        .into_iter()
        .filter(|p| ports[p.clone()].flags().contains(jack::PortFlags::IS_INPUT))
        .map(|p| p + 1)
        .collect::<Vec<_>>()
}

fn get_output_ids(c: &Client) -> Vec<usize> {
    let ports = get_ports(c);
    (0..ports.len())
        .into_iter()
        .filter(|p| {
            ports[p.clone()]
                .flags()
                .contains(jack::PortFlags::IS_OUTPUT)
        })
        .map(|p| p + 1)
        .collect::<Vec<_>>()
}

pub fn list_ports(c: &Client) {
    for portid in 1..=get_n_ports(c) {
        let port = c.port_by_id(portid as u32).unwrap();
        println!("port {}: {}", portid, port.name().unwrap());
    }
}

pub fn list_ports_full(c: &Client) {
    for p in get_ports(c) {
        println!("{:?}", p);
    }
}

pub fn connect_ports(c: &Client, command: &str) {
    let ids: Vec<u32> = command[8..]
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let port1 = c.port_by_id(ids[0]);
    let port2 = c.port_by_id(ids[1]);

    if port1.is_none() {
        println!("port {} could not be found", ids[0]);
    }
    if port2.is_none() {
        println!("port {} could not be found", ids[1]);
    }
    if c.connect_ports(port1.as_ref().unwrap(), port2.as_ref().unwrap())
        .is_err()
    {
        if c.connect_ports(port2.as_ref().unwrap(), port1.as_ref().unwrap())
            .is_err()
        {
            println!("could not connect");
        }
    }
}

pub fn disconnect_ports(c: &Client, command: &str) {
    let ids: Vec<u32> = command[11..]
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let port1 = c.port_by_id(ids[0]);
    let port2 = c.port_by_id(ids[1]);

    if port1.is_none() {
        println!("port {} could not be found", ids[0]);
    }
    if port2.is_none() {
        println!("port {} could not be found", ids[1]);
    }

    if c.disconnect_ports(port1.as_ref().unwrap(), port2.as_ref().unwrap())
        .is_err()
    {
        if c.disconnect_ports(port2.as_ref().unwrap(), port1.as_ref().unwrap())
            .is_err()
        {
            println!("could not disconnect");
        }
    }
}

pub fn list_connections(c: &Client) {
    let ports = get_ports(c);
    let outputids = get_output_ids(c);
    let inputids = get_input_ids(c);
    for oid in &outputids {
        for iid in &inputids {
            if ports[iid - 1]
                .is_connected_to(ports[oid - 1].name().unwrap().as_str())
                .unwrap()
            {
                println!("{} -> {}", oid, iid)
            }
        }
    }
}

pub fn list_flags(c: &Client) {
    let ports = get_ports(c);
    for p in 0..ports.len() {
        println!("{}: {:?}", p + 1, ports[p].flags());
    }
}

pub fn list_inputs(c: &Client) {
    for portid in 1..=get_n_ports(c) {
        let port = c.port_by_id(portid as u32).unwrap();
        if port.flags().contains(PortFlags::IS_INPUT) {
            println!("port {}: {}", portid, port.name().unwrap());
        }
    }
}

pub fn list_outputs(c: &Client) {
    for portid in 1..=get_n_ports(c) {
        let port = c.port_by_id(portid as u32).unwrap();
        if port.flags().contains(PortFlags::IS_OUTPUT) {
            println!("port {}: {}", portid, port.name().unwrap());
        }
    }
}
