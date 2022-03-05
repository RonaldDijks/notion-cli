use std::error::Error;
use std::str::from_utf8;
use tokio::process::Command;

pub async fn get_title() -> Result<Option<String>, Box<dyn Error>> {
    let output = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("✅ Add todo")
        .output()
        .await?;

    if output.status.success() {
        let title = from_utf8(&output.stdout)?.to_string();
        Ok(Some(title))
    } else {
        Ok(None)
    }
}
