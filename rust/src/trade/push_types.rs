use anyhow::{Context, Result};
use longbridge_proto::trade::Notification;
use prost::Message;
use rust_decimal::Decimal;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::trade::{
    cmd_code, serde_utils, OrderSide, OrderStatus, OrderTag, OrderType, TriggerStatus,
};

/// Order changed message
#[derive(Debug, Deserialize)]
pub struct PushOrderChanged {
    /// Order side
    pub side: OrderSide,
    /// Stock name
    pub stock_name: String,
    /// Submitted quantity
    pub quantity: String,
    /// Order symbol
    pub symbol: String,
    /// Order type
    pub order_type: OrderType,
    /// Submitted price
    pub price: Decimal,
    /// Executed quantity
    pub executed_quantity: i64,
    /// Executed price
    pub executed_price: Decimal,
    /// Order ID
    pub order_id: String,
    /// Currency
    pub currency: String,
    /// Order status
    pub status: OrderStatus,
    /// Submitted time
    #[serde(with = "serde_utils::timestamp")]
    pub submitted_at: OffsetDateTime,
    /// Last updated time
    #[serde(with = "serde_utils::timestamp")]
    pub updated_at: OffsetDateTime,
    /// Order trigger price
    pub trigger_price: Option<Decimal>,
    /// Rejected message or remark
    pub msg: Option<String>,
    /// Order tag
    pub tag: OrderTag,
    /// Conditional order trigger status
    pub trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub trigger_at: Option<OffsetDateTime>,
    /// Trailing amount
    pub trailing_amount: Option<Decimal>,
    /// Trailing percent
    pub trailing_percent: Option<Decimal>,
    /// Limit offset amount
    pub limit_offset: Option<Decimal>,
    /// Account no
    pub account_no: String,
}

/// Push event
#[derive(Debug, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum PushEvent {
    /// Order changed
    #[serde(rename = "order_changed_lb")]
    OrderChanged(PushOrderChanged),
}

impl PushEvent {
    pub(crate) fn parse(command_code: u8, data: &[u8]) -> Result<PushEvent> {
        if command_code == cmd_code::PUSH_NOTIFICATION {
            let notification = Notification::decode(data).context("decode push notification")?;
            match notification.topic.as_str() {
                "private" => Ok(serde_json::from_slice::<PushEvent>(&notification.data)?),
                _ => anyhow::bail!("unknown topic: {}", notification.topic),
            }
        } else {
            anyhow::bail!("unknown command: {}", command_code)
        }
    }
}