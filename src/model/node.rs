use super::paint::Paint;
use super::rectangle::Rectangle;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    pub r#type: NodeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    /// Bounding box of the node in absolute space coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_bounding_box: Option<Rectangle>,
    /// An array of fill paints applied to the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<Paint>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Document,
    Canvas,
    Frame,
    Group,
    Rectangle,
}
