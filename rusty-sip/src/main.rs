use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use tokio::time::sleep;
use zbus::{zvariant::Value, proxy, Connection};

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    // Notification interval
    let interval = Duration::from_secs(1800); // 30 minutes

    // message
    let message = "Hey Buddy! 30 minutes have passed, Take a water break";

    loop {
        let _reply = proxy
            .notify(
                "RustySip",
                0,
                "dialog-information",
                "⏰ Reminder",
                // message
                message,
                &[],
                HashMap::new(),
                30000, // 30 seconds
            )
            .await?;
        
        // duration
        sleep(interval).await;

    }
}
