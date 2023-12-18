use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=Kaito").await?.print().await?;
    hc.do_get("/hello2/Kaito2").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "kaitoxcode",
            "pwd": "welcome"
        }),
    );
    let _ = req_login.await?.print().await?;

    let req_create_script = hc.do_post(
        "/api/scripts",
        json!({
            "text": "Script AAA"
        }),
    );
    let _ = req_create_script.await?.print().await?;
    // hc.do_delete("/api/scripts/1").await?.print().await?;
    hc.do_get("/api/scripts").await?.print().await?;
    Ok(())
}
