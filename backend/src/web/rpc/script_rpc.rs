//! Script CRUD
//! RPC function handlers
use crate::ctx::Ctx;
use crate::model::script::{Script, ScriptBmc, ScriptForCreate, ScriptForUpdate};
use crate::model::ModelManager;
use crate::web::Result;

use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};

use rand::seq::SliceRandom;

// end of line here, fns consume ctx and mm

pub async fn create_script(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<ScriptForCreate>,
) -> Result<Script> {
    // destructure ParamsForCreate
    let ParamsForCreate { data } = params;
    let id = ScriptBmc::create(&ctx, &mm, data).await?;
    let script = ScriptBmc::get(&ctx, &mm, id).await?;
    Ok(script)
}

// TODO: (add filtering, offset, order by, etc.)
pub async fn list_scripts(ctx: Ctx, mm: ModelManager) -> Result<Vec<Script>> {
    let scripts = ScriptBmc::list(&ctx, &mm).await?;
    Ok(scripts)
}

pub async fn get_script_rand(ctx: Ctx, mm: ModelManager) -> Result<Script> {
    let _scripts = ScriptBmc::list(&ctx, &mm).await?;
    // FIXME: make this real after adding some relevant egs in db seed
    let scripts = vec![Script {
        id: 1,
        text: String::from("then there was in a little house on the road"),
    }];
    use crate::web::error::Error;
    let res = scripts
        .choose(&mut rand::thread_rng())
        .ok_or(Error::LoginFailUsernameNotFound)?;
    Ok(res.clone())
}

pub async fn update_script(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<ScriptForUpdate>,
) -> Result<Script> {
    // destructure ParamsForUpdate
    let ParamsForUpdate { id, data } = params;
    ScriptBmc::update(&ctx, &mm, id, data).await?;
    let script = ScriptBmc::get(&ctx, &mm, id).await?;
    Ok(script)
}

pub async fn delete_script(ctx: Ctx, mm: ModelManager, params: ParamsIded) -> Result<Script> {
    // destructure ParamsIded
    let ParamsIded { id } = params;
    let script = ScriptBmc::get(&ctx, &mm, id).await?;
    ScriptBmc::delete(&ctx, &mm, id).await?;
    Ok(script)
}
