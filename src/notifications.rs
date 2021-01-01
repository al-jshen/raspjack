use jack::*;

pub struct Notifications;

impl NotificationHandler for Notifications {
    fn shutdown(&mut self, status: ClientStatus, reason: &str) {
        println!("shutdown with status {:?} because \"{}\"", status, reason);
    }

    fn freewheel(&mut self, _: &Client, is_enabled: bool) {
        println!(
            "freewheel mode is {}",
            if is_enabled { "on" } else { "off" }
        );
    }

    fn buffer_size(&mut self, _: &Client, sz: Frames) -> Control {
        println!("buffer size changed to {}", sz);
        Control::Continue
    }

    fn sample_rate(&mut self, _: &Client, srate: Frames) -> Control {
        println!("sample rate changed to {}", srate);
        Control::Continue
    }

    fn client_registration(&mut self, _: &Client, name: &str, is_reg: bool) {
        println!(
            "{} client with name \"{}\"",
            if is_reg { "registered" } else { "unregistered" },
            name
        );
    }

    fn port_registration(&mut self, _: &Client, port_id: PortId, is_reg: bool) {
        println!(
            "{} port with id {}",
            if is_reg { "registered" } else { "unregistered" },
            port_id
        );
    }

    fn port_rename(
        &mut self,
        _: &Client,
        port_id: PortId,
        old_name: &str,
        new_name: &str,
    ) -> Control {
        println!("renamed port {} from {} to {}", port_id, old_name, new_name);
        Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &Client,
        port_id_a: PortId,
        port_id_b: PortId,
        are_connected: bool,
    ) {
        println!(
            "{} ports {} and {}",
            if are_connected {
                "connected"
            } else {
                "disconnected"
            },
            port_id_a,
            port_id_b,
        );
    }
}
