use super::raw::*;
use crate::providers::ProviderError;

const API_URL: &str = "https://h5.mygolbs.com/ApiData.do";
const REFERER: &str = "https://h5.mygolbs.com/";

pub struct GolbsClient {
    client: reqwest::Client,
    city: CityConfig,
}

impl GolbsClient {
    pub fn new(city: CityConfig) -> Result<Self, ProviderError> {
        let root_store = rustls::RootCertStore::from_iter(
            webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
        );
        let tls = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Linux; Android 12) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36")
            .use_preconfigured_tls(tls)
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;
        Ok(Self { client, city })
    }

    async fn post(&self, params: &[(&str, &str)]) -> Result<String, ProviderError> {
        let resp = self
            .client
            .post(API_URL)
            .header("Referer", REFERER)
            .header("Origin", "https://h5.mygolbs.com")
            .form(params)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(ProviderError::Server(format!(
                "服务器错误 (HTTP {})",
                status.as_u16()
            )));
        }

        resp.text()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))
    }

    fn check_status(raw: &serde_json::Value) -> Result<(), ProviderError> {
        let status = raw.get("status");
        match status {
            Some(serde_json::Value::Number(n)) if n.as_i64() == Some(1) => Ok(()),
            Some(serde_json::Value::String(s)) if s == "1" => Ok(()),
            _ => {
                let msg = raw
                    .get("msg")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown error");
                Err(ProviderError::Server(msg.to_string()))
            }
        }
    }

    fn parse<T: serde::de::DeserializeOwned>(raw: serde_json::Value) -> Result<T, ProviderError> {
        serde_json::from_value(raw).map_err(|e| ProviderError::Parse(e.to_string()))
    }

    pub async fn nearby_stations(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<NearbyStationsResponse, ProviderError> {
        let lat_s = lat.to_string();
        let lng_s = lng.to_string();
        let params = [
            ("CITYNAME", self.city.name.as_str()),
            ("LAT", &lat_s),
            ("LNG", &lng_s),
            ("CMD", "106"),
            ("CITYKEY", self.city.key.as_str()),
        ];
        let text = self.post(&params).await?;
        tracing::debug!("CMD 106 raw: {}", &text);
        let raw: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError::Parse(e.to_string()))?;
        Self::check_status(&raw)?;
        Self::parse(raw)
    }

    pub async fn station_lines(
        &self,
        station_name: &str,
        lat: &str,
        lng: &str,
        all: bool,
    ) -> Result<StationLinesResponse, ProviderError> {
        let all_s = if all { "1" } else { "0" };
        let params = [
            ("CMD", "115"),
            ("CITYNAME", self.city.name.as_str()),
            ("STATIONNAME", station_name),
            ("MYLAT", lat),
            ("MYLNG", lng),
            ("ALL", all_s),
            ("CITYKEY", self.city.key.as_str()),
        ];
        let text = self.post(&params).await?;
        tracing::debug!("CMD 115 raw: {}", &text);
        let raw: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError::Parse(e.to_string()))?;
        Self::check_status(&raw)?;
        Self::parse(raw)
    }

    pub async fn line_stations(
        &self,
        line_name: &str,
        direction: &str,
    ) -> Result<LineStationsResponse, ProviderError> {
        let params = [
            ("CMD", "103"),
            ("CITYNAME", self.city.name.as_str()),
            ("LINENAME", line_name),
            ("DIRECTION", direction),
            ("CITYKEY", self.city.key.as_str()),
        ];
        let text = self.post(&params).await?;
        tracing::debug!("CMD 103 raw: {}", &text);
        let raw: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError::Parse(e.to_string()))?;
        Self::check_status(&raw)?;
        Self::parse(raw)
    }

    pub async fn realtime(
        &self,
        line_name: &str,
        direction: &str,
        station_order: u32,
    ) -> Result<RealTimeResponse, ProviderError> {
        let order_s = station_order.to_string();
        let params = [
            ("CMD", "104"),
            ("CITYNAME", self.city.name.as_str()),
            ("LINENAME", line_name),
            ("DIRECTION", direction),
            ("STATIONORDER", &order_s),
            ("CITYKEY", self.city.key.as_str()),
        ];
        let text = self.post(&params).await?;
        tracing::debug!("CMD 104 raw: {}", &text);
        let raw: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError::Parse(e.to_string()))?;
        Self::check_status(&raw)?;
        Self::parse(raw)
    }

    pub async fn all_lines(&self) -> Result<AllLinesResponse, ProviderError> {
        let params = [
            ("CITYNAME", self.city.name.as_str()),
            ("CMD", "119"),
            ("CITYKEY", self.city.key.as_str()),
            ("KEY", ""),
        ];
        let text = self.post(&params).await?;
        tracing::debug!("CMD 119 raw: {}", &text);
        let raw: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError::Parse(e.to_string()))?;
        Self::check_status(&raw)?;
        Self::parse(raw)
    }
}
