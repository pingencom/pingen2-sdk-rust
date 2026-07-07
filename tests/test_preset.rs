use pingen2_sdk::PresetRelationship;

#[test]
fn test_preset_relationship_new() {
    let preset = PresetRelationship::new("my-preset-id");
    let value = preset.to_value();

    assert_eq!(value["preset"]["data"]["id"], "my-preset-id");
    assert_eq!(value["preset"]["data"]["type"], "presets");
}

#[test]
fn test_preset_relationship_different_id() {
    let preset = PresetRelationship::new("85dce039-ebb7-4de8-b953-4f22afae789e");
    let value = preset.to_value();

    assert_eq!(
        value["preset"]["data"]["id"],
        "85dce039-ebb7-4de8-b953-4f22afae789e"
    );
}
