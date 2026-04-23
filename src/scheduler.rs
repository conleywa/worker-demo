use worker::{ScheduleContext, ScheduledEvent};
use worker_macros::event;

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: worker::Env, _ctx: ScheduleContext) {
    match count_users(&env).await {
        Ok(count) => tracing::info!(?event, "Current user count={}", count),
        Err(e) => tracing::error!(?e, "Couldn't get user count"),
    };
}

async fn count_users(env: &worker::Env) -> worker::Result<u64> {
    let db = env.d1("demo_user_d1")?;
    Ok(db
        .prepare("SELECT count(*) FROM t_user")
        .first::<u64>(None)
        .await?
        .unwrap_or(0))
}
