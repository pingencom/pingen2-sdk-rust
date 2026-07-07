use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookEventCategory {
    Issues,
    Sent,
    Undeliverable,
    Delivered,
    ChannelSubscriptions,
}

impl WebhookEventCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Issues => "issues",
            Self::Sent => "sent",
            Self::Undeliverable => "undeliverable",
            Self::Delivered => "delivered",
            Self::ChannelSubscriptions => "channel_subscriptions",
        }
    }
}

impl fmt::Display for WebhookEventCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
