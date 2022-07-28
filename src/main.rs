use anyhow::{anyhow, Result};
use clap::{App, Arg};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<()> {
    let matches = App::new("emu2rs")
        .version("0.1.0")
        .author("Will Kelleher <will@kelleher.io>")
        .about("Relays samples from a Rainfoest EMU-2 to InfluxDB")
        .arg(
            Arg::with_name("device")
                .long("device")
                .takes_value(true)
                .help("Serial device name"),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .takes_value(true)
                .help("InfluxDB Host"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .takes_value(true)
                .help("InfluxDB Token"),
        )
        .get_matches();

    let host = matches
        .value_of("host")
        .ok_or(anyhow!("No host specified"))?;

    let token = matches
        .value_of("token")
        .ok_or(anyhow!("No token specified"))?;

    let device = matches
        .value_of("device")
        .ok_or(anyhow!("No device specified"))?;

    // open serial port
    let mut port = serialport::new(device, 115200)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .flow_control(FlowControl::Hardware)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(10))
        .open()?;

    let mut serial_buf: Vec<u8> = vec![0; 5000];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(anyhow!("{:?}", e)),
        }
    }

    Ok(())
}
