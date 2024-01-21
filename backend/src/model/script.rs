use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

// region:      --- Script Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Script {
    pub id: i64,
    pub text: String,
}

#[derive(Fields, Deserialize)]
pub struct ScriptForCreate {
    pub text: String,
}

#[derive(Fields, Deserialize)]
pub struct ScriptForUpdate {
    pub text: Option<String>,
}
// endregion:   --- Script Types

// region:      --- ScriptBmc
pub struct ScriptBmc;

/// Use shared implementation of the base BMCs
impl DbBmc for ScriptBmc {
    const TABLE: &'static str = "script";
}

/// Script specific implementation of the BMC
/// for now 1:1 with base but this may change
impl ScriptBmc {
    // region:      --- /api/v1
    pub async fn create(ctx: &Ctx, mm: &ModelManager, script_c: ScriptForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, script_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Script> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Script>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        script_u: ScriptForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, script_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
    // endregion:   --- /api/v1

    // region:      --- /api/v2
    pub async fn get_rand(_ctx: &Ctx, mm: &ModelManager) -> Result<Script> {
        let db = mm.db();
        let script: Script = sqlx::query_as("SELECT * FROM script ORDER BY RANDOM() LIMIT 1")
            .fetch_optional(db)
            .await?
            .ok_or(Error::NoEtitiesFound { entity: "script" })?;
        Ok(script)
    }

    // endregion:   --- /api/v2
}
// endregion:   --- ScriptBmc

// region:      --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use serial_test::serial;
    use tracing::debug;

    #[tokio::test]
    #[serial]
    async fn test_create_ok() -> Result<()> {
        // Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fixture_text = "test_create_ok text";
        // exec
        let script_c = ScriptForCreate {
            text: fixture_text.to_string(),
        };
        let id = ScriptBmc::create(&ctx, &mm, script_c).await?;
        // check
        let script_g = ScriptBmc::get(&ctx, &mm, id).await?;
        assert_eq!(script_g.text, fixture_text);
        // clean up
        ScriptBmc::delete(&ctx, &mm, id);
        // let count = sqlx::query("DELETE FROM script WHERE id = $1")
        //     .bind(id)
        //     .execute(mm.db())
        //     .await?
        //     .rows_affected();
        // assert_eq!(count, 1, "created script was not deleted!?");
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fixture_id = 100;
        // exec
        let res = ScriptBmc::get(&ctx, &mm, fixture_id).await;
        // check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "script",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fixture_texts = &["test_list_ok-script 01", "test_list_ok-script 02"];
        _dev_utils::seed_scripts(&ctx, &mm, fixture_texts).await?;
        // exec
        let scripts = ScriptBmc::list(&ctx, &mm).await?;
        // check
        let scripts: Vec<Script> = scripts
            .into_iter()
            .filter(|s| s.text.starts_with("test_list_ok-script"))
            .collect();
        assert_eq!(scripts.len(), 2, "number of seeded scripts does not match");
        // clean up
        for script in scripts.iter() {
            ScriptBmc::delete(&ctx, &mm, script.id).await?;
        }
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fixture_text = "test_update_ok - script 01";
        let fixture_text_new = "test_update_ok - script 01 - new";
        let fixture_script = _dev_utils::seed_scripts(&ctx, &mm, &[fixture_text])
            .await?
            .remove(0);
        // exec
        ScriptBmc::update(
            &ctx,
            &mm,
            fixture_script.id,
            ScriptForUpdate {
                text: Some(fixture_text_new.to_string()),
            },
        )
        .await?;
        // check
        let script = ScriptBmc::get(&ctx, &mm, fixture_script.id).await?;
        assert_eq!(script.text, fixture_text_new);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fixture_id = 100;
        // exec
        let res = ScriptBmc::delete(&ctx, &mm, fixture_id).await;
        // check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "script",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );
        Ok(())
    }
}
// endregion:   --- Tests
