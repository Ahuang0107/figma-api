/// [Figma documentation](https://www.figma.com/developers/api#rectangle-type)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Rectangle {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
