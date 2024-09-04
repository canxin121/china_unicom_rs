use crate::{format_duration, DATETIME_FORMAT, DEFAULT_FORMAT, DEFAULT_FORMAT_WITH_LAST};
use aho_corasick::AhoCorasick;
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChinaUnicomData {
    // 套餐名称
    pub package_name: String,
    // 查pub
    pub time: DateTime<Local>,

    // 已用流量
    pub sum_flow_used: f64,
    // 已用定向流量
    pub limit_flow_used: f64,
    // 已用通用流量
    pub non_limit_flow_used: f64,
    // 已用免费流量
    pub free_flow_used: f64,
    // 已用非免费流量
    pub non_free_flow_used: f64,

    // 总
    pub sum_flow: f64,
    // 总定向流量
    pub limit_flow: f64,
    // 总通用流量
    pub non_limit_flow: f64,

    // 已用通话
    pub sum_voice_used: i64,
    // 已用定向通话
    pub limit_voice_used: i64,
    // 已用通用通话
    pub non_limit_voice_used: i64,

    // 总通话
    pub sum_voice: i64,
    // 总定向通话
    pub limit_voice: i64,
    // 总通用通话
    pub non_limit_voice: i64,
}

// (总量，通用总量，定向总量，总余量，通用余量，定向余量)
pub(crate) fn parse_infinate_voice(
    limit_total: i64,
    non_limit_total: i64,
    limit_used: i64,
    non_limited_used: i64,
) -> (String, String, String, String, String, String) {
    if limit_total == 0 && limit_used > 0 {
        if non_limit_total == 0 && non_limited_used > 0 {
            return (
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
            );
        } else {
            return (
                format!("{:.2}分钟", non_limit_total),
                format!("{:.2}分钟", non_limit_total),
                "无限".to_string(),
                format!("{:.2}分钟", non_limit_total - non_limited_used),
                format!("{:.2}分钟", non_limit_total - non_limited_used),
                "无限".to_string(),
            );
        }
    } else {
        if non_limit_total == 0 && non_limited_used > 0 {
            return (
                "无限".to_string(),
                "无限".to_string(),
                format!("{:.2}分钟", limit_total),
                "无限".to_string(),
                "无限".to_string(),
                format!("{:.2}分钟", limit_total - limit_used),
            );
        } else {
            return (
                format!("{:.2}分钟", limit_total + non_limit_total),
                format!("{:.2}分钟", non_limit_total),
                format!("{:.2}分钟", limit_total),
                format!(
                    "{:.2}分钟",
                    limit_total + non_limit_total - non_limited_used - limit_used
                ),
                format!("{:.2}分钟", non_limit_total - non_limited_used),
                format!("{:.2}分钟", limit_total - limit_used),
            );
        }
    }
}

// (总量，通用总量，定向总量，总余量，通用余量，定向余量)
pub fn parse_infinate_flow(
    limit_total: f64,
    non_limit_total: f64,
    limit_used: f64,
    non_limited_used: f64,
) -> (String, String, String, String, String, String) {
    if limit_total == 0.0 && limit_used > 0.0 {
        if non_limit_total == 0.0 && non_limited_used > 0.0 {
            return (
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
                "无限".to_string(),
            );
        } else {
            return (
                format!("{:.2}G", non_limit_total),
                format!("{:.2}G", non_limit_total),
                "无限".to_string(),
                format!("{:.2}G", non_limit_total - non_limited_used),
                format!("{:.2}G", non_limit_total - non_limited_used),
                "无限".to_string(),
            );
        }
    } else {
        if non_limit_total == 0.0 && non_limited_used > 0.0 {
            return (
                "无限".to_string(),
                "无限".to_string(),
                format!("{:.2}G", limit_total),
                "无限".to_string(),
                "无限".to_string(),
                format!("{:.2}G", limit_total - limit_used),
            );
        } else {
            return (
                format!("{:.2}G", limit_total + non_limit_total),
                format!("{:.2}G", non_limit_total),
                format!("{:.2}G", limit_total),
                format!(
                    "{:.2}G",
                    limit_total + non_limit_total - non_limited_used - limit_used
                ),
                format!("{:.2}G", non_limit_total - non_limited_used),
                format!("{:.2}G", limit_total - limit_used),
            );
        }
    }
}

impl ChinaUnicomData {
    pub fn format(&self, fmt: &str) -> Result<String> {
        let patterns = &[
            "[时间]",
            "[流量总量]",
            "[流量定向总量]",
            "[流量通用总量]",
            "[流量总用量]",
            "[流量免费用量]",
            "[流量收费用量]",
            "[流量定向用量]",
            "[流量通用用量]",
            "[通话总用量]",
            "[通话定向用量]",
            "[通话通用用量]",
            "[通话总量]",
            "[通话定向总量]",
            "[通话通用总量]",
            "[流量总余量]",
            "[流量定向余量]",
            "[流量通用余量]",
            "[通话总余量]",
            "[通话定向余量]",
            "[通话通用余量]",
            "[套餐名称]",
        ];

        let (
            sum_voice,
            limit_voice,
            non_limit_voice,
            sum_voice_left,
            limit_voice_left,
            non_limit_voice_left,
        ) = parse_infinate_voice(
            self.limit_voice,
            self.non_limit_voice,
            self.limit_voice_used,
            self.non_limit_voice_used,
        );
        let (
            sum_flow,
            non_limit_flow,
            limit_flow,
            sum_flow_left,
            non_limit_flow_left,
            limit_flow_left,
        ) = parse_infinate_flow(
            self.limit_flow,
            self.non_limit_flow,
            self.limit_flow_used,
            self.non_limit_flow_used,
        );
        let replace_with = &[
            self.time.format(DATETIME_FORMAT).to_string(),
            sum_flow,
            limit_flow,
            non_limit_flow,
            format!("{:.2}G", self.sum_flow_used),
            format!("{:.2}G", self.free_flow_used),
            format!("{:.2}G", self.non_free_flow_used),
            format!("{:.2}G", self.limit_flow_used),
            format!("{:.2}G", self.non_limit_flow_used),
            format!("{:.2}分钟", self.sum_voice_used),
            format!("{:.2}分钟", self.limit_voice_used),
            format!("{:.2}分钟", self.non_limit_voice_used),
            sum_voice,
            limit_voice,
            non_limit_voice,
            sum_flow_left,
            limit_flow_left,
            non_limit_flow_left,
            sum_voice_left,
            limit_voice_left,
            non_limit_voice_left,
            self.package_name.clone(),
        ];

        let ac = AhoCorasick::new(patterns).unwrap();
        let mut result = vec![];
        ac.try_stream_replace_all(fmt.as_bytes(), &mut result, replace_with)?;
        Ok(String::from_utf8(result).unwrap())
    }

    pub fn format_with_last(&self, fmt: &str, last: &Self) -> Result<String> {
        let duration = self.time - last.time;
        let duration_str = format_duration(duration);
        let sum_flow_used = (self.sum_flow_used - last.sum_flow_used).max(0.0);
        let free_flow_used = (self.free_flow_used - last.free_flow_used).max(0.0);
        let non_free_flow_used = (self.non_free_flow_used - last.non_free_flow_used).max(0.0);
        let sum_voice_used = (self.sum_voice_used - last.sum_voice_used).max(0);
        let limit_voice_used = (self.limit_voice_used - last.limit_voice_used).max(0);
        let non_limit_voice_used = (self.non_limit_voice_used - last.non_limit_voice_used).max(0);

        let patterns = &[
            "[区间时长]",
            "[区间流量总用量]",
            "[区间流量免费用量]",
            "[区间流量收费用量]",
            "[区间通话总用量]",
            "[区间通话定向用量]",
            "[区间通话通用用量]",
            "[时间]",
            "[流量总量]",
            "[流量定向总量]",
            "[流量通用总量]",
            "[流量总用量]",
            "[流量免费用量]",
            "[流量收费用量]",
            "[流量定向用量]",
            "[流量通用用量]",
            "[通话总用量]",
            "[通话定向用量]",
            "[通话通用用量]",
            "[通话总量]",
            "[通话定向总量]",
            "[通话通用总量]",
            "[流量总余量]",
            "[流量定向余量]",
            "[流量通用余量]",
            "[通话总余量]",
            "[通话定向余量]",
            "[通话通用余量]",
            "[套餐名称]",
        ];
        let (
            sum_voice,
            limit_voice,
            non_limit_voice,
            sum_voice_left,
            limit_voice_left,
            non_limit_voice_left,
        ) = parse_infinate_voice(
            self.limit_voice,
            self.non_limit_voice,
            self.limit_voice_used,
            self.non_limit_voice_used,
        );
        let (
            sum_flow,
            non_limit_flow,
            limit_flow,
            sum_flow_left,
            non_limit_flow_left,
            limit_flow_left,
        ) = parse_infinate_flow(
            self.limit_flow,
            self.non_limit_flow,
            self.limit_flow_used,
            self.non_limit_flow_used,
        );
        let replace_with = &[
            duration_str,
            format!("{:.2}G", sum_flow_used),
            format!("{:.2}G", free_flow_used),
            format!("{:.2}G", non_free_flow_used),
            format!("{:.2}分钟", sum_voice_used),
            format!("{:.2}分钟", limit_voice_used),
            format!("{:.2}分钟", non_limit_voice_used),
            self.time.format(DATETIME_FORMAT).to_string(),
            sum_flow,
            limit_flow,
            non_limit_flow,
            format!("{:.2}G", self.sum_flow_used),
            format!("{:.2}G", self.free_flow_used),
            format!("{:.2}G", self.non_free_flow_used),
            format!("{:.2}G", self.limit_flow_used),
            format!("{:.2}G", self.non_limit_flow_used),
            format!("{:.2}分钟", self.sum_voice_used),
            format!("{:.2}分钟", self.limit_voice_used),
            format!("{:.2}分钟", self.non_limit_voice_used),
            sum_voice,
            limit_voice,
            non_limit_voice,
            sum_flow_left,
            limit_flow_left,
            non_limit_flow_left,
            sum_voice_left,
            limit_voice_left,
            non_limit_voice_left,
            self.package_name.clone(),
        ];

        let ac = AhoCorasick::new(patterns).unwrap();
        let mut result = vec![];
        ac.try_stream_replace_all(fmt.as_bytes(), &mut result, replace_with)?;
        Ok(String::from_utf8(result).unwrap())
    }

    pub fn format_default(&self) -> Result<String> {
        self.format(DEFAULT_FORMAT)
    }

    pub fn format_default_with_last(&self, last: &Self) -> Result<String> {
        self.format_with_last(DEFAULT_FORMAT_WITH_LAST, last)
    }
}
