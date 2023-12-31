use crate::model::script::{Script, ScriptBmc};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn get_script_rand(mm: ModelManager) -> Result<Script> {
    let script = ScriptBmc::get_rand(&mm).await?;
    Ok(script)
}
