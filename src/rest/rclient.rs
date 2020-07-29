use anyhow::{anyhow, format_err};
use hex::encode as hex_encode;
use reqwest::{Client, StatusCode};
use ring::hmac;
use std::collections::BTreeMap;

pub struct RestClient {
    base_url: String,
    keys: Option<(String, String)>,
}

impl RestClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url,
            keys: None,
        }
    }

    pub fn with_key(base_url: String, keys: (String, String)) -> Self {
        Self {
            base_url: base_url,
            keys: Some(keys),
        }
    }

    pub fn build_request_string(
        &self,
        end_point: &str,
        params: BTreeMap<String, String>,
        need_sign: bool,
    ) -> anyhow::Result<String> {
        if let None = self.keys {
            return Err(format_err!("{}", "KEYS not set"));
        }
        let mut params_string = String::new();
        for (k, v) in &params {
            params_string.push_str(&format!("&{}={}", k, v));
        }
        params_string.remove(0);
        if need_sign {
            let signed_key =
                hmac::Key::new(hmac::HMAC_SHA256, self.keys.as_ref().unwrap().1.as_bytes());
            let signature = hex_encode(hmac::sign(&signed_key, params_string.as_bytes().as_ref()));
            params_string.push_str(&format!("&signature={}", signature));
        } else {
        }
        Ok(format!("{}{}?{}", self.base_url, end_point, params_string))
    }

    pub async fn resp2string(&self, resp: reqwest::Response) -> anyhow::Result<String> {
        match resp.status() {
            StatusCode::OK => {
                let resp_string = resp.text().await?;
                Ok(resp_string)
            }
            StatusCode::BAD_REQUEST => Err(anyhow!("{}\n{:#?}", "BAD REQUEST", resp.text().await)),
            StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow!("{}", "INTERNAL SERVER ERROR")),
            _ => Err(format_err!("{}", "Unknown Error")),
        }
    }

    pub async fn post_sign(&self, url: String) -> anyhow::Result<String> {
        if let Some((ref ak, _)) = self.keys {
            let client = Client::new();
            let resp = client.post(&url).header("X-MBX-APIKEY", ak).send().await?;
            let re = self.resp2string(resp).await?;
            Ok(re)
        } else {
            Err(format_err!("{}", "KEYS not config"))
        }
    }

    pub async fn delete_sign(&self, url: String) -> anyhow::Result<String> {
        if let Some((ref ak, _)) = self.keys {
            let client = Client::new();
            let resp = client
                .delete(&url)
                .header("X-MBX-APIKEY", ak)
                .send()
                .await?;
            let re = self.resp2string(resp).await?;
            Ok(re)
        } else {
            Err(format_err!("{}", "KEYS not config"))
        }
    }

    pub async fn get_sign(&self, url: String) -> anyhow::Result<String> {
        if let Some((ref ak, _)) = self.keys {
            let client = Client::new();
            let resp = client.get(&url).header("X-MBX-APIKEY", ak).send().await?;
            let re = self.resp2string(resp).await?;
            Ok(re)
        } else {
            Err(format_err!("{}", "KEYS not config"))
        }
    }

    pub async fn get(&self, url: String) -> anyhow::Result<String> {
        let client = Client::new();
        let resp = client.get(&url).send().await?;
        let re = self.resp2string(resp).await?;
        Ok(re)
    }
}
