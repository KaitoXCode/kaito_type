#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_script = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_script",
            "params": {
                "data": {
                    "text": "script AAA"
                }
            }
        }),
    );
    req_create_script.await?.print().await?;

    let req_update_script = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_script",
            "params": {
                "id": 1000, // hard coded script id
                "data": {
                    "text": "script BB"
                }
            }
        }),
    );
    req_update_script.await?.print().await?;

    let req_delete_script = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_script",
            "params": {
                "id": 1001 // hardcoded script id
            }
        }),
    );
    req_delete_script.await?.print().await?;

    let req_list_scripts = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "list_scripts"
        }),
    );
    req_list_scripts.await?.print().await?;

    let req_get_script_rand = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "get_script_rand"
        }),
    );
    req_get_script_rand.await?.print().await?;

    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    // hc.do_get("/hello").await?.print().await?;

    Ok(())
}
