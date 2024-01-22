use crate::ctx::Ctx;
use crate::model::script::{Script, ScriptBmc};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn get_script_rand(ctx: Ctx, mm: ModelManager) -> Result<Script> {
    let script = ScriptBmc::get_rand(&ctx, &mm).await?;
    Ok(script)
}

pub async fn calculate_accuracy(user_input: &str, script: &str) -> f64 {
    let correct_chars: usize = user_input
        .chars()
        .zip(script.chars())
        .filter(|(a, b)| a == b)
        .count();
    let total_chars: usize = script.chars().count();
    (correct_chars as f64 / total_chars as f64) * 100.0
}

pub async fn calculate_speed(user_input: &str, elapsed_time: &i64) -> f64 {
    // Assume an average word length for simplicity
    // const AVERAGE_WORD_LENGTH: usize = 5;
    let word_count = user_input.split_whitespace().count();
    let wpm = if *elapsed_time > 0 {
        // avoid div by 0
        let minutes = (*elapsed_time as f64) / 60.0;
        ((word_count as f64) / minutes) as f64
    } else {
        0.0
    };
    wpm.round()
}
