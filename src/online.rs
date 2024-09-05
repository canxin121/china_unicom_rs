use crate::CLIENT;
use anyhow::Result;
use reqwest::{
    header::{CONTENT_TYPE, SET_COOKIE},
    Response,
};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct OnlineResp {
    pub online_token: String,
    pub cookie: String,
}

#[derive(Deserialize)]
pub struct RawOnlineResp {
    pub token_online: String,
}

pub async fn online(token_online: &str, app_id: &str) -> Result<OnlineResp> {
    let url = "https://m.client.10010.com/mobileService/onLine.htm";
    let body = format!("appId={app_id}&token_online={token_online}&version=iphone_c@9.0100");
    let resp = CLIENT
        .post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await?;

    let cookie = extract_ecs_cookies(&resp).ok_or(anyhow::anyhow!("Failed to get new cookie"))?;

    let raw_resp: RawOnlineResp = resp.json().await?;

    let resp = OnlineResp {
        online_token: raw_resp.token_online,
        cookie,
    };

    Ok(resp)
}
fn extract_ecs_cookies(response: &Response) -> Option<String> {
    // 初始化变量用于存储所需的 Cookie
    let mut ecs_token: Option<String> = None;
    let mut ecs_acc: Option<String> = None;

    // 遍历所有 Set-Cookie 头部字段
    for cookie in response.headers().get_all(SET_COOKIE).iter() {
        if let Ok(cookie_str) = cookie.to_str() {
            // 分割每个 Set-Cookie 字段的值为键值对
            for part in cookie_str.split("; ") {
                if let Some((key, value)) = part.split_once('=') {
                    // 检查是否为 ecs_token 或 ecs_acc
                    match key {
                        "ecs_token" => ecs_token = Some(value.to_string()),
                        "ecs_acc" => ecs_acc = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }

        // 如果找到了两个字段，立即返回
        if ecs_token.is_some() && ecs_acc.is_some() {
            break;
        }
    }

    // 构建并返回目标 Cookie 字符串
    match (ecs_token, ecs_acc) {
        (Some(token), Some(acc)) => Some(format!("ecs_token={}; ecs_acc={}", token, acc)),
        _ => None,
    }
}
