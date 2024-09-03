use anyhow::Result;
use chrono::Local;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};

use crate::data::ChinaUnicomData;
use crate::CLIENT;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChinaUnicomResponse {
    // pub online_customer_service_switch: bool,
    // pub new_tw_flag: String,
    // pub charging_capacity_link: String,
    // pub buy_voice_link: String,
    // pub can_use_sms_all: String,
    // #[serde(rename = "RzbResources")]
    // pub rzb_resources: Vec<RzbResource>,
    // pub use_daily_percent: String,
    // #[serde(rename = "Topblock")]
    // pub topblock: String,
    // #[serde(rename = "XsbResources")]
    // pub xsb_resources: Vec<XsbResource>,
    // pub rzb_all_use: String,
    // pub cs_img: String,
    // pub languageflag: String,
    // pub sum_percent: String,
    // #[serde(rename = "MlResources")]
    // pub ml_resources: Vec<MlResource>,
    // pub canuse_voice_all_unit: String,
    // pub crowdfunding_flag: bool,
    // pub all_user_flow: String,
    // pub sms_head_used: i64,
    // #[serde(rename = "TwResources")]
    // pub tw_resources: Vec<Value>,
    // #[serde(rename = "accountBAR")]
    // pub account_bar: Vec<AccountBar>,
    // pub subscribe_to_text_messages_link: String,
    // pub voice_exceed: i64,
    // pub code: String,
    // pub canuse_flow_all_unit: String,
    pub sumresource: f64,
    // pub city_code: String,
    pub sum: String,
    // pub used_voice_noun_explain: String,
    // pub cs_entrance_url: String,
    // pub flow_exceed: f64,
    // pub balancesumqry: String,
    // #[serde(rename = "wangTTcUrl")]
    // pub wang_ttc_url: String,
    // pub use_percent: Vec<UsePercent>,
    // pub voice_sumresource: i64,
    pub package_name: String,
    // pub can_use_value_all: String,
    pub summary: Summary,
    // #[serde(rename = "s1HistoryFlowDetails")]
    // pub s1history_flow_details: Vec<Value>,
    // pub reminder: String,
    // pub province_code: String,
    // pub package_id: String,
    // pub usefree_percent: String,
    // pub mobile: String,
    pub resources: Vec<Resource>,
    // pub traffic_prompts_are_exempted: String,
    // pub left_month: i64,
    // pub canuse_sms_all_unit: String,
    // #[serde(rename = "s2HistoryFlowDetails")]
    // pub s2history_flow_details: Vec<Value>,
    // pub voice_head_used: i64,
    // pub time: String,
    // pub business_type: String,
    // #[serde(rename = "MlProgrammeFlag")]
    // pub ml_programme_flag: String,
    // pub sms_sumresource: i64,
    // pub balancedetailqry: String,
    // pub sms_exceed: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RzbResource {
//     pub details: Vec<Value>,
//     pub rzb_all_use: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct XsbResource {
//     pub details: Vec<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MlResource {
//     pub details: Vec<Detail>,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub user_resource: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Detail {
//     pub elem_type: String,
//     pub end_date: String,
//     pub end_xsb_date: String,
//     pub fee_policy_id: String,
//     pub fee_policy_name: String,
//     pub hide_carry_forward_label: bool,
//     pub limited: String,
//     #[serde(rename = "use")]
//     pub use_field: String,
//     pub used_percent: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AccountBar {
//     pub business_code: String,
//     pub button_name: String,
//     pub create_time: String,
//     pub creator: String,
//     pub creator_province_code: String,
//     pub id: String,
//     pub link_url: String,
//     pub nettype: String,
//     pub order_number: String,
//     pub payment_type: String,
//     pub province: String,
//     pub status: String,
//     pub user_id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UsePercent {
//     #[serde(rename = "Value")]
//     pub value: String,
//     #[serde(rename = "value")]
//     pub value2: String,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    // pub domestic_daily_flow: String,
    // pub fengdingstate: String,
    pub free_flow: String,
    // pub free_percent: String,
    // pub percent: Vec<Percent>,
    // pub remain_feng_ding: String,
    // pub remain_percent: String,
    // pub sum: String,
    // pub use_percent: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Percent {
//     #[serde(rename = "Value")]
//     pub value: String,
//     #[serde(rename = "value")]
//     pub value2: String,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub details: Vec<ResourceDetail>,
    // pub remain_resource: String,
    #[serde(rename = "type")]
    pub type_field: String,
    // pub url: String,
    // pub user_resource: String,
    // pub w_turl: String,
    // pub wang_turl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDetail {
    // pub add_up_item_name: String,
    // pub addup_item_code: String,
    // pub elem_type: String,
    // pub end_date: String,
    // pub end_xsb_date: String,
    // pub fee_policy_id: String,
    // pub fee_policy_name: String,
    // pub hide_carry_forward_label: bool,
    pub limited: String,
    // pub rb_flag: String,
    // pub realresourcetype: String,
    // pub remain: String,
    // pub resource_source: String,
    // pub resource_type: String,
    pub total: String,
    // pub typemark: String,
    #[serde(rename = "use")]
    pub use_field: String,
    // pub used_percent: String,
    // pub xexceedvalue: String,
    // pub before_remain: Option<String>,
    // pub before_total: Option<String>,
    // pub before_use: Option<String>,
    // pub f_kused_percent: Option<String>,
    // pub merge_flag: Option<String>,
    // pub z_kused_percent: Option<String>,
}

impl ChinaUnicomResponse {
    pub fn to_unicom_data(&self) -> ChinaUnicomData {
        let (sum_flow_used, free_flow_used, non_free_flow_used) =
            self.get_flow_usage().unwrap_or((0.0, 0.0, 0.0));
        let (sum_flow, limit_flow, non_limit_flow, limit_flow_used, non_limit_flow_used) =
            self.get_flow_details().unwrap_or((0.0, 0.0, 0.0, 0.0, 0.0));
        let (
            sum_voice_used,
            sum_voice,
            limit_voice_used,
            limit_voice,
            non_limit_voice_used,
            non_limit_voice,
        ) = self.get_voice_details().unwrap_or((0, 0, 0, 0, 0, 0));

        ChinaUnicomData {
            package_name: self.package_name.clone(),
            time: Local::now(),
            sum_flow_used,
            free_flow_used,
            non_free_flow_used,
            sum_flow,
            limit_flow,
            non_limit_flow,
            limit_flow_used,
            non_limit_flow_used,
            sum_voice_used,
            sum_voice,
            limit_voice_used,
            limit_voice,
            non_limit_voice_used,
            non_limit_voice,
        }
    }
}

impl ChinaUnicomResponse {
    fn parse_flow(flow: &str) -> Result<f64, String> {
        flow.parse::<f64>()
            .map(|f| f / 1024.0)
            .map_err(|e| e.to_string())
    }

    pub fn get_flow_usage(&self) -> Result<(f64, f64, f64), String> {
        let sum_flow = self.sumresource / 1024.0;
        let free_flow = Self::parse_flow(&self.summary.free_flow)?;
        let non_free_flow = sum_flow - free_flow;

        Ok((sum_flow, free_flow, non_free_flow))
    }

    pub fn get_flow_details(&self) -> Result<(f64, f64, f64, f64, f64), String> {
        let mut sum_flow = 0.0;
        let mut limit_flow = 0.0;
        let mut non_limit_flow = 0.0;
        let mut limit_flow_used = 0.0;
        let mut non_limit_flow_used = 0.0;

        let details: &Resource = self
            .resources
            .iter()
            .find(|r| r.type_field == "flow")
            .ok_or("无流量包信息")?;

        for detail in &details.details {
            let total = detail.total.parse::<f64>().map_err(|e| e.to_string())? / 1024.0;
            let used = detail.use_field.parse::<f64>().map_err(|e| e.to_string())? / 1024.0;
            sum_flow += total;

            if detail.limited == "1" {
                limit_flow += total;
                limit_flow_used += used;
            } else {
                non_limit_flow += total;
                non_limit_flow_used += used;
            }
        }

        Ok((
            sum_flow,
            limit_flow,
            non_limit_flow,
            limit_flow_used,
            non_limit_flow_used,
        ))
    }

    pub fn get_voice_details(&self) -> Result<(i64, i64, i64, i64, i64, i64), String> {
        let mut sum_voice_used = 0;
        let mut sum_voice = 0;
        let mut limit_voice_used = 0;
        let mut non_limit_voice_used = 0;
        let mut limit_voice = 0;
        let mut non_limit_voice = 0;

        let details: &Resource = self
            .resources
            .iter()
            .find(|r| r.type_field == "Voice")
            .ok_or("无通话包信息")?;

        for detail in &details.details {
            let used = detail.use_field.parse::<i64>().map_err(|e| e.to_string())?;
            let total = detail.total.parse::<i64>().map_err(|e| e.to_string())?;
            sum_voice_used += used;
            sum_voice += total;

            if detail.limited == "1" {
                limit_voice_used += used;
                limit_voice += total;
            } else {
                non_limit_voice_used += used;
                non_limit_voice += total;
            }
        }

        Ok((
            sum_voice_used,
            sum_voice,
            limit_voice_used,
            limit_voice,
            non_limit_voice_used,
            non_limit_voice,
        ))
    }
}

pub async fn query_china_unicom_data(cookie: &str) -> Result<ChinaUnicomData> {
    let url = "https://m.client.10010.com/servicequerybusiness/operationservice/queryOcsPackageFlowLeftContentRevisedInJune";

    let response:ChinaUnicomResponse = CLIENT.post(url)
        .header(ACCEPT, "application/json, text/plain, */*")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(COOKIE, cookie)
        .header(USER_AGENT, "Mozilla/5.0 (iPhone; CPU iPhone OS 16_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 unicom{version:iphone_c@11.0700}")
        .header(REFERER, "https://img.client.10010.com/")
        .header(ACCEPT_LANGUAGE, "zh-SG,zh-CN;q=0.9,zh-Hans;q=0.8")
        .body("duanlianjieabc=&channelCode=&serviceType=&saleChannel=&externalSources=&contactCode=&ticket=&ticketPhone=&ticketChannel=&language=chinese")
        .send()
        .await?.json().await?;

    Ok(response.to_unicom_data())
}
