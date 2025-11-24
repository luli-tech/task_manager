use crate::{db::DbPool, models::Task};
use chrono::Utc;
use sqlx::query_as;
use tokio::sync::broadcast;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

pub async fn start_notification_service(
    db: DbPool,
    notification_tx: broadcast::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    // Run every minute to check for tasks with upcoming reminders
    let job = Job::new_async("0 * * * * *", move |_uuid, _l| {
        let db = db.clone();
        let tx = notification_tx.clone();

        Box::pin(async move {
            if let Err(e) = check_and_send_notifications(db, tx).await {
                error!("Error checking notifications: {:?}", e);
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    info!("Notification service started");
    Ok(())
}

async fn check_and_send_notifications(
    db: DbPool,
    tx: broadcast::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();

    // Find tasks with reminders that are due and haven't been notified yet
    let tasks = query_as::<_, Task>(
        "SELECT * FROM tasks 
         WHERE reminder_time <= $1 
         AND notified = false 
         AND reminder_time IS NOT NULL"
    )
    .bind(now)
    .fetch_all(&db)
    .await?;

    for task in tasks {
        // Create notification in database
        let notification_message = format!(
            "Reminder: {} is due soon!",
            task.title
        );

        sqlx::query(
            "INSERT INTO notifications (user_id, task_id, message)
             VALUES ($1, $2, $3)"
        )
        .bind(task.user_id)
        .bind(task.id)
        .bind(&notification_message)
        .execute(&db)
        .await?;

        // Mark task as notified
        sqlx::query("UPDATE tasks SET notified = true WHERE id = $1")
            .bind(task.id)
            .execute(&db)
            .await?;

        // Broadcast to SSE clients
        let broadcast_message = format!(
            "{}:{}",
            task.user_id,
            notification_message
        );
        
        let _ = tx.send(broadcast_message);
        
        info!("Sent notification for task: {}", task.title);
    }

    Ok(())
}
