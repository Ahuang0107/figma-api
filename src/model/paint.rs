/// A solid color, gradient, or image texture that can be applied as fills or strokes
///
/// [Figma documentation](https://www.figma.com/developers/api#paint-type)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paint {
    pub r#type: PaintType,
    /// A reference to an image embedded in this node. To download the image using this reference, use the GET file images endpoint to retrieve the mapping from image references to image URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaintType {
    Solid,
    Image,
}
