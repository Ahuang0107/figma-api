use std::fs::read_to_string;

#[tokio::test]
async fn get_me() {
    let token = read_to_string("tests/example_token.private.txt").unwrap();
    let figma_api = figma_api::FigmaApi::new(&token);
    let me = figma_api.get_me().await.unwrap();
    assert_eq!(me.handle, "elase huang");
}

#[tokio::test]
async fn get_file() {
    let token = read_to_string("tests/example_token.private.txt").unwrap();
    let figma_api = figma_api::FigmaApi::new(&token);
    let file = figma_api.get_file("TdFAucbdVXiZDe0HPZeFvL").await.unwrap();
    assert_eq!(file.name, "Unpacking");
}

#[tokio::test]
async fn get_image_fills() {
    let token = read_to_string("tests/example_token.private.txt").unwrap();
    let figma_api = figma_api::FigmaApi::new(&token);
    let image_fills = figma_api
        .get_image_fills("TdFAucbdVXiZDe0HPZeFvL")
        .await
        .unwrap();
    assert_eq!(image_fills.status, 200);
}
