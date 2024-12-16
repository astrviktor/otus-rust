pub mod socket;
pub mod thermo;

pub use socket::Socket;
pub use thermo::Thermo;

pub enum Device {
    SocketDevice(Socket),
    ThermoDevice(Thermo),
}

pub trait Info {
    fn info(&self) -> String;
}
