use pingen2_sdk::{
    AddressPosition, BatchDeliveryProduct, BatchIcon, DeliveryProduct, GroupingType, PaperType,
    PrintMode, PrintSpectrum, SplitPosition, SplitType, WebhookEventCategory,
};

#[test]
fn test_address_position() {
    assert_eq!(AddressPosition::Left.as_str(), "left");
    assert_eq!(AddressPosition::Right.as_str(), "right");
    assert_eq!(AddressPosition::Left.to_string(), "left");
}

#[test]
fn test_delivery_product() {
    assert_eq!(DeliveryProduct::Cheap.as_str(), "cheap");
    assert_eq!(DeliveryProduct::Fast.as_str(), "fast");
    assert_eq!(DeliveryProduct::Registered.as_str(), "registered");
    assert_eq!(DeliveryProduct::Bulk.as_str(), "bulk");
    assert_eq!(DeliveryProduct::Premium.as_str(), "premium");
    assert_eq!(DeliveryProduct::Fast.to_string(), "fast");
}

#[test]
fn test_print_mode() {
    assert_eq!(PrintMode::Simplex.as_str(), "simplex");
    assert_eq!(PrintMode::Duplex.as_str(), "duplex");
    assert_eq!(PrintMode::Simplex.to_string(), "simplex");
}

#[test]
fn test_print_spectrum() {
    assert_eq!(PrintSpectrum::Color.as_str(), "color");
    assert_eq!(PrintSpectrum::Grayscale.as_str(), "grayscale");
    assert_eq!(PrintSpectrum::Color.to_string(), "color");
}

#[test]
fn test_grouping_type() {
    assert_eq!(GroupingType::Merge.as_str(), "merge");
    assert_eq!(GroupingType::Zip.as_str(), "zip");
    assert_eq!(GroupingType::Merge.to_string(), "merge");
}

#[test]
fn test_split_type() {
    assert_eq!(SplitType::File.as_str(), "file");
    assert_eq!(SplitType::Page.as_str(), "page");
    assert_eq!(SplitType::Custom.as_str(), "custom");
    assert_eq!(SplitType::QrInvoice.as_str(), "qr_invoice");
    assert_eq!(SplitType::Page.to_string(), "page");
}

#[test]
fn test_split_position() {
    assert_eq!(SplitPosition::FirstPage.as_str(), "first_page");
    assert_eq!(SplitPosition::LastPage.as_str(), "last_page");
    assert_eq!(SplitPosition::LastPage.to_string(), "last_page");
}

#[test]
fn test_webhook_event_category() {
    assert_eq!(WebhookEventCategory::Issues.as_str(), "issues");
    assert_eq!(WebhookEventCategory::Sent.as_str(), "sent");
    assert_eq!(
        WebhookEventCategory::Undeliverable.as_str(),
        "undeliverable"
    );
    assert_eq!(WebhookEventCategory::Delivered.as_str(), "delivered");
    assert_eq!(
        WebhookEventCategory::ChannelSubscriptions.as_str(),
        "channel_subscriptions"
    );
    assert_eq!(WebhookEventCategory::Issues.to_string(), "issues");
}

#[test]
fn test_batch_icon() {
    assert_eq!(BatchIcon::Campaign.as_str(), "campaign");
    assert_eq!(BatchIcon::Megaphone.as_str(), "megaphone");
    assert_eq!(BatchIcon::WaveHand.as_str(), "wave-hand");
    assert_eq!(BatchIcon::Flash.as_str(), "flash");
    assert_eq!(BatchIcon::Rocket.as_str(), "rocket");
    assert_eq!(BatchIcon::Bell.as_str(), "bell");
    assert_eq!(BatchIcon::PercentTag.as_str(), "percent-tag");
    assert_eq!(BatchIcon::PercentBadge.as_str(), "percent-badge");
    assert_eq!(BatchIcon::Present.as_str(), "present");
    assert_eq!(BatchIcon::Receipt.as_str(), "receipt");
    assert_eq!(BatchIcon::Document.as_str(), "document");
    assert_eq!(BatchIcon::Information.as_str(), "information");
    assert_eq!(BatchIcon::Calendar.as_str(), "calendar");
    assert_eq!(BatchIcon::Newspaper.as_str(), "newspaper");
    assert_eq!(BatchIcon::Crown.as_str(), "crown");
    assert_eq!(BatchIcon::Virus.as_str(), "virus");
    assert_eq!(BatchIcon::Information.to_string(), "information");
}

#[test]
fn test_batch_delivery_product() {
    let bdp = BatchDeliveryProduct::new("CH", DeliveryProduct::Fast);
    assert_eq!(bdp.country, "CH");
    assert_eq!(bdp.delivery_product, "fast");
}

#[test]
fn test_paper_type() {
    assert_eq!(PaperType::Normal.as_str(), "normal");
    assert_eq!(PaperType::Qr.as_str(), "qr");
    assert_eq!(PaperType::SepaAt.as_str(), "sepa_at");
    assert_eq!(PaperType::SepaDe.as_str(), "sepa_de");
    assert_eq!(PaperType::Normal.to_string(), "normal");
}
