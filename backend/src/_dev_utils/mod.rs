use tokio::sync::OnceCell;
use tracing::info;

use crate::ctx::Ctx;
use crate::model;
use crate::model::script::Script;
use crate::model::script::ScriptBmc;
use crate::model::script::ScriptForCreate;
use crate::model::ModelManager;

// region:      --- Modules
mod dev_db;

// endregion:   --- Modules

/// init dev env
/// TODO: rm this later, only to be used in early dev
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// init test env
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_scripts(
    ctx: &Ctx,
    mm: &ModelManager,
    texts: &[&str],
) -> model::Result<Vec<Script>> {
    let mut scripts = Vec::new();
    for text in texts {
        let id = ScriptBmc::create(
            ctx,
            mm,
            ScriptForCreate {
                text: text.to_string(),
            },
        )
        .await?;
        let script = ScriptBmc::get(ctx, mm, id).await?;
        scripts.push(script);
    }
    Ok(scripts)
}
