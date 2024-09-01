use std::{sync::LazyLock, time::Duration};

use anyhow::Result;
use china_unicom_rs::{data::ChinaUnicomData, query::query_china_unicom_data};
use chrono::TimeDelta;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // 中国联通cookie
    pub cookie: String,
    // bark key
    pub key: String,
    // 请求间隔(s)
    #[serde(default = "default_interval")]
    pub interval: u64,
    // 免费用量阈值(G)
    #[serde(default)]
    pub free_threshold: Option<f64>,
    // 收费用量阈值(G)
    #[serde(default = "default_non_threshold")]
    pub non_threshold: Option<f64>,
    // 发送超时(s)(即使未到阈值，超出此时间也会发送)
    #[serde(default = "default_timeout")]
    pub timeout: Option<i64>,
}

fn default_interval() -> u64 {
    60
}

fn default_non_threshold() -> Option<f64> {
    Some(0.05)
}

fn default_timeout() -> Option<i64> {
    Some(1800)
}

const FORMAT1: &'static str = "[区间时长] 跳: [区间流量收费用量], 免: [区间流量免费用量]";

const FORMAT2: &'static str = "今跳:[区间流量收费用量], 今免: [区间流量免费用量]";

const FORMAT3: &'static str = "通用余: [流量通用余量], 定向余: [流量定向余量]";

const FORMAT4: &'static str = "通用已用: [流量通用用量], 定向已用: [流量定向用量]";

static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

fn format_cachefile_name(date: chrono::NaiveDate) -> String {
    date.format("china_unicom_%Y-%m-%d.json").to_string()
}

fn load_config() -> Result<Config> {
    let config = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

async fn load_data(date: chrono::NaiveDate) -> Result<ChinaUnicomData> {
    let file_name = format_cachefile_name(date);
    let data = tokio::fs::read_to_string(file_name).await?;
    let data: ChinaUnicomData = serde_json::from_str(&data)?;
    Ok(data)
}

async fn save_data(data: &ChinaUnicomData) -> Result<()> {
    let date = data.time.date_naive();
    let file_name = format_cachefile_name(date);
    let data = serde_json::to_string(data)?;
    tokio::fs::write(file_name, data).await?;
    Ok(())
}

async fn format_message(data: &ChinaUnicomData, lastdata: &ChinaUnicomData) -> Result<String> {
    let mut message = data.format_with_last(&FORMAT1, lastdata)?;
    message += "%0a";
    let yesterday = data.time.date_naive() - chrono::Duration::days(1);
    if let Ok(yesterday_data) = load_data(yesterday).await {
        message += &data.format_with_last(&FORMAT2, &yesterday_data)?;
        message += "%0a";
    } else {
        message += &data.format(&FORMAT4)?;
        message += "%0a";
    };
    message += &data.format(&FORMAT3)?;
    Ok(message)
}

async fn format_first_message(data: &ChinaUnicomData) -> Result<String> {
    let mut message = data.format(&FORMAT3)?;
    message += "%0a";
    message += &data.format(&FORMAT4)?;
    let yesterday = data.time.date_naive() - chrono::Duration::days(1);
    if let Ok(yesterday_data) = load_data(yesterday).await {
        message += "%0a";
        message += &data.format_with_last(&FORMAT2, &yesterday_data)?;
    };
    Ok(message)
}

async fn notify(bark_push_key: &str, title: &str, message: &str) -> Result<()> {
    println!("发送消息: [{}]-({})", title, message);
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

fn should_notify(config: &Config, data: &ChinaUnicomData, lastdata: &ChinaUnicomData) -> bool {
    if let Some(timeout) = config.timeout {
        if data.time - lastdata.time >= TimeDelta::seconds(timeout) {
            return true;
        }
    } else if let Some(free_threshold) = config.free_threshold {
        if data.free_flow_used - lastdata.free_flow_used >= free_threshold {
            return true;
        }
    } else if let Some(nonfree_threshold) = config.non_threshold {
        if data.non_free_flow_used - lastdata.non_free_flow_used >= nonfree_threshold {
            return true;
        }
    }
    return false;
}

// 首先获取一个新的数据，然后减去上次获取(第一次没有)，以及昨日获取(必有)

#[tokio::main]
async fn main() {
    let config = load_config().unwrap();
    println!("Run with config: {:#?}", config);
    let mut last_data = {
        match load_data(chrono::Local::now().naive_local().into()).await {
            Err(_) => {
                let data = query_china_unicom_data(&config.cookie).await.unwrap();
                match format_first_message(&data).await {
                    Ok(message) => {
                        let _ = notify(&config.key, &data.package_name, &message).await;
                    }
                    Err(e) => {
                        let _ =
                            notify(&config.key, "联通余量", &format!("格式化消息出错: {e}")).await;
                    }
                }
                data
            }
            Ok(data) => data,
        }
    };

    let mut max_retry = 3;
    let interval = Duration::from_secs(config.interval);

    loop {
        match query_china_unicom_data(&config.cookie).await {
            Ok(data) => match format_message(&data, &last_data).await {
                Ok(message) => {
                    println!("{}", message);
                    max_retry = 3;
                    if should_notify(&config, &data, &last_data) {
                        let _ = notify(&config.key, &data.package_name, &message).await;

                        if let Err(e) = save_data(&data).await {
                            let _ = notify(&config.key, "联通余量", &format!("缓存出错:{e}")).await;
                        };

                        last_data = data;
                    }
                }
                Err(e) => {
                    if max_retry == 0 {
                        let _ = notify(
                            &config.key,
                            "联通余量",
                            &format!("格式化消息失败: {}%0a失败次数过多, 程序退出", e),
                        )
                        .await;
                        panic!()
                    }
                    max_retry -= 1;
                    let _ =
                        notify(&config.key, "联通余量", &format!("格式化消息失败: {}", e)).await;
                }
            },
            Err(e) => {
                if max_retry == 0 {
                    let _ = notify(
                        &config.key,
                        "联通余量",
                        &format!("获取数据失败: {}%0a失败次数过多, 程序退出", e),
                    )
                    .await;
                    panic!()
                }
                max_retry -= 1;
                let _ = notify(&config.key, "联通余量", &format!("获取数据失败: {}", e)).await;
            }
        }
        sleep(interval).await;
    }
}
