mod hex;
mod influx;
mod types;

use crate::types::{InstantaneousDemand, Message, PriceCluster};

use anyhow::{anyhow, Result};
use clap::{App, Arg};
use futures::prelude::*;
use influxdb2::Client;
use influxdb2::models::DataPoint;
use serde_xml_rs::{from_str, to_string};
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Write};
use std::time::Duration;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {
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
            Arg::with_name("org")
                .long("org")
                .takes_value(true)
                .help("InfluxDB Organization"),
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

    let org = matches.value_of("org").ok_or(anyhow!("No org specified"))?;

    let token = matches
        .value_of("token")
        .ok_or(anyhow!("No token specified"))?;

    let device = matches
        .value_of("device")
        .ok_or(anyhow!("No device specified"))?;

    let bucket = "emu2";
    let client = Client::new(host, org, token);

    // open serial port
    let mut port = serialport::new(device, 115200)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .flow_control(FlowControl::Hardware)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(1))
        .open()?;

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut read: Vec<u8> = vec![0; 5000];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                read.extend_from_slice(&serial_buf[0..t]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                let size = read.len();
                if size > 0 {
                    io::stdout().write_all(&read[..size])?;
                    let s = String::from_utf8_lossy(&read);
                    match from_str::<Vec<Message>>(&s) {
                        Ok(items) => {
                            println!("converting points");
                            let points = items.into_iter().map(TryFrom::try_from).collect::<Result<Vec<DataPoint>, _>>()?;
                            println!("sending to influxdb");
                            let res = client.write(bucket, stream::iter(points)).await;
                            match res {
                                Ok(()) => println!("wrote data!"),
                                Err(e) => println!("error: {:?}", e),
                            }
                        }
                        Err(e) => println!("error parsing: {:?}", e),
                    }
                    read.clear();
                }
            }
            Err(e) => return Err(anyhow!("{:?}", e)),
        }
    }

    Ok(())
}
