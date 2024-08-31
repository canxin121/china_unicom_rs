use anyhow::Result;
use china_unicom_rs::{data::ChinaUnicomData, query::query_china_unicom_data, CLIENT};
use serde::Deserialize;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
struct Config {
    // 认证配置
    auth: AuthConfig,
    // 应用程序配置
    #[serde(default)]
    app: AppConfig,
}

// Authentication Config struct
#[derive(Debug, Deserialize)]
struct AuthConfig {
    // 中国联通 余量请求中的 Cookie
    cookie: String,
    // Bark 推送 Key
    key: String,
}

// Application Config struct
#[derive(Debug, Deserialize)]
struct AppConfig {
    // 缓存文件路径 默认值为 "./china_unicom_data.json"
    #[serde(default = "default_cache_file")]
    cache_file: String,
    // 查询间隔(s) 默认值为 30
    #[serde(default = "default_interval")]
    interval: u64,
    // 查询超时时间(s) 默认值为 30
    #[serde(default = "default_timeout")]
    timeout: i64,

    // 消息格式
    #[serde(default = "default_message_format")]
    message_format: String,
    // 首次消息格式
    #[serde(default = "default_first_message_format")]
    first_message_format: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cache_file: default_cache_file(),
            interval: default_interval(),
            timeout: default_timeout(),
            message_format: default_message_format(),
            first_message_format: default_first_message_format(),
        }
    }
}

// Default values for AppConfig fields
fn default_cache_file() -> String {
    "./china_unicom_data.json".to_string()
}

fn default_interval() -> u64 {
    30
}

fn default_timeout() -> i64 {
    30
}

// Default values for MessageFormat fields
fn default_message_format() -> String {
    "[区间时长] 跳[区间流量收费用量] 免[区间流量免费用量]%0a通用余[流量通用余量] 定向余[流量定向余量]".to_string()
}

fn default_first_message_format() -> String {
    "通用总:[流量通用总量] 定向总:[流量定向总量]%0a通用余:[流量通用余量] 定向余:[流量定向余量]"
        .to_string()
}

async fn load_config() -> Result<Config> {
    let config_content = tokio::fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

pub async fn save_data(data: &ChinaUnicomData, cache_file: &str) -> Result<()> {
    let json = serde_json::to_string(data)?;
    tokio::fs::write(cache_file, json).await?;
    Ok(())
}

pub async fn load_data(cache_file: &str) -> Option<ChinaUnicomData> {
    if let Ok(json) = tokio::fs::read_to_string(cache_file).await {
        if let Ok(data) = serde_json::from_str(&json) {
            Some(data)
        } else {
            None
        }
    } else {
        None
    }
}

pub async fn notify(bark_push_key: &str, title: &str, message: &str) -> Result<()> {
    let resp = CLIENT
        .post(format!(
            "https://api.day.app/{}/{}/{}",
            bark_push_key, title, message
        ))
        .send()
        .await?;
    let text = resp.text().await?;
    println!("Send Notification response: {}", text);
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = load_config().await.expect("Failed to load configuration");
    println!("Run with config: {:#?}", config);

    let mut max_query_retry = 3;

    let cached_data = load_data(&config.app.cache_file).await;

    let mut last_data = match query_china_unicom_data(&config.auth.cookie).await {
        Ok(data) => data,
        Err(err) => {
            let _ = notify(
                &config.auth.key,
                "联通余量",
                &format!("Failed to query initial data: {:?}", err),
            )
            .await;
            panic!("Failed to query initial data: {:?}, Abort!", err);
        }
    };

    if let Err(e) = save_data(&last_data, &config.app.cache_file).await {
        let _ = notify(
            &config.auth.key,
            "联通余量",
            &format!("Failed to save initial data: {:?}", e),
        )
        .await;
        panic!("Failed to save initial data: {:?}, Abort!", e);
    }

    let first_message_result = match &cached_data {
        Some(data) => {
            println!("Formatting first message with cached data...");
            last_data.format_with_last(&config.app.message_format, &data)
        }
        None => {
            println!("No cached data found. Formatting first message with initial data...");
            last_data.format(&config.app.first_message_format)
        }
    };

    match first_message_result {
        Ok(message) => {
            let _ = notify(&config.auth.key, &last_data.package_name, &message).await;
        }
        Err(err) => {
            let _ = notify(
                &config.auth.key,
                "联通余量",
                &format!("Failed to format first message: {:?}, Abort!", err),
            )
            .await;
            panic!("Failed to format first message: {:?}, Abort!", err);
        }
    }

    let interval = Duration::from_secs(config.app.interval);
    let timeout = chrono::Duration::seconds(config.app.timeout);

    loop {
        tokio::time::sleep(interval).await;

        match query_china_unicom_data(&config.auth.cookie).await {
            Ok(data) => {
                if let Ok(message) = data.format_with_last(&config.app.message_format, &last_data) {
                    println!("{}", message);
                    if data.non_free_flow_used - last_data.non_free_flow_used > 0.2
                        || data.free_flow_used - last_data.free_flow_used > 1.0
                        || data.time - last_data.time >= timeout
                    {
                        let _ = notify(&config.auth.key, &data.package_name, &message).await;

                        if let Err(err) = save_data(&data, &config.app.cache_file).await {
                            let _ = notify(
                                &config.auth.key,
                                "联通余量",
                                &format!("Failed to save data: {:?}", err),
                            )
                            .await;
                        }

                        last_data = data;
                    }
                }
            }
            Err(err) => {
                max_query_retry -= 1;
                println!(
                    "Failed to query data: {:?}. Retries left: {}",
                    err, max_query_retry
                );
                if max_query_retry < 0 {
                    let _ = notify(
                        &config.auth.key,
                        "联通余量",
                        &format!("Failed to query data: {:?}%0aMax Retry exceed, Abort!", err),
                    )
                    .await;
                    panic!("Failed to query data: {:?}\nMax Retry exceed, Abort!", err);
                }
                let _ = notify(
                    &config.auth.key,
                    "联通余量",
                    &format!("Failed to query data: {:?}", err),
                )
                .await;
            }
        }
    }
}
