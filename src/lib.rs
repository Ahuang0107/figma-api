use std::collections::HashMap;
use std::error::Error;

use reqwest::Client;

use model::Node;

pub mod model;

pub struct FigmaApi {
    base_url: String,
    figma_token: String,
    client: Client,
}

impl FigmaApi {
    pub fn new(token: &str) -> Self {
        Self {
            base_url: "https://api.figma.com".into(),
            figma_token: token.into(),
            client: Client::new(),
        }
    }
    pub async fn get_me(&self) -> Result<ResponseMe, Box<dyn Error>> {
        let url = self.base_url.clone() + "/v1/me";
        let request = self
            .client
            .get(url)
            .header("X-Figma-Token", &self.figma_token)
            .build()?;
        let res = self.client.execute(request).await?;
        Ok(res.json::<ResponseMe>().await?)
    }
    pub async fn get_file(&self, key: &str) -> Result<ResponseFile, Box<dyn Error>> {
        let url = self.base_url.clone() + &format!("/v1/files/{key}");
        let request = self
            .client
            .get(url)
            .header("X-Figma-Token", &self.figma_token)
            .build()?;
        let res = self.client.execute(request).await?;
        Ok(res.json::<ResponseFile>().await?)
    }
    pub async fn get_image_fills(&self, key: &str) -> Result<ResponseImageFills, Box<dyn Error>> {
        let url = self.base_url.clone() + &format!("/v1/files/{key}/images");
        let request = self
            .client
            .get(url)
            .header("X-Figma-Token", &self.figma_token)
            .build()?;
        let res = self.client.execute(request).await?;
        Ok(res.json::<ResponseImageFills>().await?)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ResponseMe {
    pub id: String,
    pub email: String,
    pub handle: String,
    pub img_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ResponseFile {
    pub name: String,
    pub document: Node,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ResponseImageFills {
    pub error: bool,
    pub status: usize,
    pub meta: Option<Images>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Images {
    pub images: HashMap<String, String>,
}
