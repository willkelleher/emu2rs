use serde::{Deserialize};
use crate::hex::*;

/**
 * These are properties shared by multiple events.
 *
 * Other events will inherit these to avoid duplication.
 *
 *    <DeviceMacId>0xd8d5b90000011943</DeviceMacId>
 *    <MeterMacId>0x00135005001c495d</MeterMacId>
 *    <TimeStamp>0x2a74a349</TimeStamp>
 */

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Base {
    #[serde(deserialize_with = "from_hex_string")]
    pub device_mac_id: String,
    #[serde(deserialize_with = "from_hex_string")]
    pub meter_mac_id: String,
    #[serde(deserialize_with = "from_hex")]
    pub time_stamp: u32,
}

/**
 * Event: InstantaneousDemand
 *
 * This tells us what our current usage us in Watts.
 *
 *  <InstantaneousDemand>
 *    <DeviceMacId>0xd8d5b90000011943</DeviceMacId>
 *    <MeterMacId>0x00135005001c495d</MeterMacId>
 *    <TimeStamp>0x2a74a349</TimeStamp>
 *    <Demand>0x0004da</Demand>
 *    <Multiplier>0x00000001</Multiplier>
 *    <Divisor>0x000003e8</Divisor>
 *    <DigitsRight>0x03</DigitsRight>
 *    <DigitsLeft>0x0f</DigitsLeft>
 *    <SuppressLeadingZero>Y</SuppressLeadingZero>
 *  </InstantaneousDemand>
 */


#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InstantaneousDemand {
    #[serde(deserialize_with = "from_hex_string")]
    pub device_mac_id: String,
    #[serde(deserialize_with = "from_hex_string")]
    pub meter_mac_id: String,
    #[serde(deserialize_with = "from_hex")]
    pub time_stamp: u32,
    #[serde(deserialize_with = "from_hex")]
    pub demand: u32, // Watts
    #[serde(deserialize_with = "from_hex")]
    pub multiplier: u32,
    #[serde(deserialize_with = "from_hex")]
    pub divisor: u32,
    #[serde(deserialize_with = "from_hex")]
    pub digits_right: u32,
    #[serde(deserialize_with = "from_hex")]
    pub digits_left: u32,
    #[serde(deserialize_with = "from_hex_bool")]
    pub suppress_leading_zero: bool,
}

/**
 * Event: PriceCluster
 *
 * This tells us what the current price is.
 *
 *  <PriceCluster>
 *       <DeviceMacId>0xd8d5b90000011943</DeviceMacId>
 *       <MeterMacId>0x00135005001c495d</MeterMacId>
 *       <TimeStamp>0x2a74a910</TimeStamp>
 *       <Price>0x000025e1</Price>
 *       <Currency>0x0348</Currency>
 *       <TrailingDigits>0x05</TrailingDigits>
 *       <Tier>0x00</Tier>
 *       <StartTime>0x2a7382d0</StartTime>
 *       <Duration>0x05a0</Duration>
 *       <RateLabel></RateLabel>
 *  </PriceCluster>
*/

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PriceCluster {
    #[serde(deserialize_with = "from_hex_string")]
    pub device_mac_id: String,
    #[serde(deserialize_with = "from_hex_string")]
    pub meter_mac_id: String,
    #[serde(deserialize_with = "from_hex")]
    pub time_stamp: u32,
    #[serde(deserialize_with = "from_hex")]
    pub price: u32,
    #[serde(deserialize_with = "from_hex")]
    pub currency: u32,
    #[serde(deserialize_with = "from_hex")]
    pub trailing_digits: u32,
    #[serde(deserialize_with = "from_hex")]
    pub start_time: u32,
    #[serde(deserialize_with = "from_hex")]
    pub duration: u32,
    #[serde(deserialize_with = "from_hex_string")]
    pub rate_label: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Message {
    InstantaneousDemand(InstantaneousDemand),
    PriceCluster(PriceCluster),
}
