use super::model::{Request, SupportExchangeApi};
use failure::Fallible;
// use reqwest::{Client, Response};
use super::error::{EdpError, ExchagneErrorMessage, ExchangeErrorResponse};
use chrono::{Duration, Utc};
use hex;
use log::{error, trace};
use reqwest::blocking::{Client, Response};
use reqwest::Method;
use ring::hmac;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string, to_value};
use url::Url;

// pub trait Exchange {
//     fn ticker();
//     fn book();
//     fn kline();
//     fn limit_buy();
//     fn limit_sell();
//     fn market_buy();
//     fn market_sell();
// }

const EXPIRE_DURATION: i64 = 5;

#[derive(Debug, Clone)]
pub struct Exchange {
    client: Client,
    credential: Option<(String, String)>,
    host_url: String,
}

impl Exchange {
    pub fn new(host_url: &str) -> Self {
        Exchange {
            client: Client::new(),
            credential: None,
            host_url: host_url.into(),
        }
    }

    pub fn with_credential(api_key: &str, sec_key: &str, host_url: &str) -> Self {
        Exchange {
            client: Client::new(),
            credential: Some((api_key.into(), sec_key.into())),
            host_url: host_url.into(),
        }
    }

    pub fn request<R>(&self, req: R) -> Fallible<R::ResponseData>
    where
        R: Request,
        R::ResponseData: DeserializeOwned,
    {
        let url_str = format!("{}{}", self.host_url, R::ENDPOINT);
        let url = Url::parse(&url_str)?;
        let body = match R::METHOD {
            Method::PUT | Method::POST => to_string(&req)?,
            _ => "".to_string(),
        };

        let mut builder = self.client.request(R::METHOD, url.clone());

        match R::SIGNED {
            Some(api) => match api {
                SupportExchangeApi::Binance => {
                    let url_new = Url::parse_with_params(&url_str, req.to_url_query())?;
                    let (api_key, sec_key) = self.check_key()?;
                    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, sec_key.as_bytes());
                    let signature = hex::encode(hmac::sign(
                        &signed_key,
                        url_new.query().unwrap().to_string().as_bytes(),
                    ));
                    builder = builder
                        .query(&req.to_url_query())
                        .query(&[("signature", signature)])
                        .header("X-MBX-APIKEY", api_key);
                }
                SupportExchangeApi::Bitmex => {
                    let expires = (Utc::now() + Duration::seconds(EXPIRE_DURATION)).timestamp();
                    let (key, signature) = self.signature(R::METHOD, expires, &url, &body)?;

                    builder = builder
                        .header("api-expires", expires)
                        .header("api-key", key)
                        .header("api-signature", signature)
                        .header("content-type", "application/json")
                        .body(body.clone());
                }
            },
            None => {
                match R::METHOD {
                    Method::GET | Method::DELETE => {
                        if R::HAS_PAYLOAD {
                            builder = builder
                            .query(&req.to_url_query());
                        } else {
                            
                        }
                    }
                    _ => {}
                };
            }
        }
        let resp = builder.send()?;
        // println!("{}", resp.url());
        Ok(self.handle_response(resp)?)
    }

    pub fn check_key(&self) -> Fallible<(&str, &str)> {
        match self.credential.as_ref() {
            Some((a, s)) => Ok((a, s)),
            None => Err(EdpError::NoApiKeySet.into()),
        }
    }

    pub fn signature(
        &self,
        method: Method,
        expires: i64,
        url: &Url,
        body: &str,
    ) -> Fallible<(&str, String)> {
        let (api_key, sec_key) = self.check_key()?;
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, sec_key.as_bytes());
        let sign_message = match url.query() {
            Some(query) => format!(
                "{}{}?{}{}{}",
                method.as_str(),
                url.path(),
                query,
                expires,
                body
            ),
            None => format!("{}{}{}{}", method.as_str(), url.path(), expires, body),
        };
        trace!("Sign message {}", sign_message);
        let signature = hex::encode(hmac::sign(&signed_key, sign_message.as_bytes()));
        Ok((api_key, signature))
    }

    pub fn handle_response<T>(&self, res: Response) -> Fallible<T>
    where
        T: DeserializeOwned,
    {
        if res.status().is_success() {
            let res_text = res.text()?;
            match from_str::<T>(&res_text) {
                Ok(res_data) => Ok(res_data),
                Err(e) => {
                    error!("cannot deserialize '{}'", res_text);
                    Err(e.into())
                }
            }
        } else {
            let res: ExchagneErrorMessage = res.json()?;
            Err(EdpError::from(res).into())
        }
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query_string(&self) -> String {
        let vec = self.to_url_query();
        vec.into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_string(value).unwrap()))
            }
        }
        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}
