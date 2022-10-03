use crate::types::{Message, Message::InstantaneousDemand, Message::PriceCluster};
use influxdb2::models::DataPoint;
use influxdb2::models::data_point::DataPointError;
use std::convert::TryFrom;

impl TryFrom<Message> for DataPoint {
    type Error = DataPointError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        match value {
            InstantaneousDemand(v) => {
                DataPoint::builder("instantaneous_demand")
                .tag("meter", v.meter_mac_id)
                .field("demand", v.demand as i64)
                .build()
            },
            PriceCluster(v) => {
                DataPoint::builder("price_cluster")
                    .tag("meter", v.meter_mac_id)
                    .field("price", v.price as i64)
                    .build()
            }
        }
    }
}

