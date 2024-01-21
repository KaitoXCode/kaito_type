use crate::ctx::Ctx;
use crate::model::script::{Script, ScriptBmc};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn get_script_rand(ctx: Ctx, mm: ModelManager) -> Result<Script> {
    let script = ScriptBmc::get_rand(&ctx, &mm).await?;
    Ok(script)
}
