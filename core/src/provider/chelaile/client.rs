use super::raw::*;
use crate::providers::ProviderError;
use aes::Aes256;
use base64::Engine;
use ecb::cipher::{BlockModeDecrypt, KeyInit};
use md5::{Digest, Md5};

const BASE_URL: &str = "https://web.chelaile.net.cn/api";
const SIGN_SALT: &str = "qwihrnbtmj";
const AES_KEY: &[u8; 32] = b"422556651C7F7B2B5C266EED06068230";

type Aes256EcbDec = ecb::Decryptor<Aes256>;

pub struct ChelaileClient {
    client: reqwest::Client,
    city_id: String,
}

impl ChelaileClient {
    pub fn new(city_id: &str) -> Result<Self, ProviderError> {
        use reqwest::header::{HeaderMap, HeaderValue, REFERER};
        let mut headers = HeaderMap::new();
        headers.insert(
            REFERER,
            HeaderValue::from_static("https://web.chelaile.net.cn/"),
        );

        let root_store =
            rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let tls = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36")
            .default_headers(headers)
            .use_preconfigured_tls(tls)
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;
        Ok(Self {
            client,
            city_id: city_id.to_string(),
        })
    }

    fn crypto_sign(params: &[(&str, &str)]) -> String {
        let concat: String = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let input = format!("{concat}{SIGN_SALT}");
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        let digest = hasher.finalize();
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn decrypt_aes(encrypted: &str) -> Result<serde_json::Value, ProviderError> {
        let ciphertext = base64::engine::general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| ProviderError::Parse(format!("base64 decode: {e}")))?;

        let mut buf = ciphertext;
        let decryptor = Aes256EcbDec::new(AES_KEY.into());
        let plaintext = decryptor
            .decrypt_padded::<ecb::cipher::block_padding::Pkcs7>(&mut buf)
            .map_err(|e| ProviderError::Parse(format!("AES decrypt: {e}")))?;

        let text = std::str::from_utf8(plaintext)
            .map_err(|e| ProviderError::Parse(format!("UTF-8: {e}")))?;

        serde_json::from_str(text).map_err(|e| ProviderError::Parse(format!("JSON: {e}")))
    }

    fn strip_markers(text: &str) -> &str {
        let s = text.strip_prefix("**YGKJ").unwrap_or(text);
        s.strip_suffix("YGKJ##").unwrap_or(s)
    }

    fn common_params(&self) -> Vec<(&str, String)> {
        vec![
            ("s", "h5".into()),
            ("v", "9.1.2".into()),
            ("vc", "1".into()),
            ("src", "webapp_default".into()),
            ("userId", "browser_wherebus".into()),
            ("h5Id", "browser_wherebus".into()),
            ("sign", "1".into()),
            ("cityId", self.city_id.clone()),
        ]
    }

    async fn get(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, ProviderError> {
        let url = format!("{BASE_URL}/{path}");
        let resp = self
            .client
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let text = resp
            .text()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let clean = Self::strip_markers(&text);
        tracing::debug!("{path} raw: {clean}");

        let json: serde_json::Value = serde_json::from_str(clean)
            .map_err(|e| ProviderError::Parse(format!("JSON parse: {e}")))?;

        let status = json
            .pointer("/jsonr/status")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if status != "00" {
            let msg = json
                .pointer("/jsonr/errmsg")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(ProviderError::Server(format!("{msg} (status={status})")));
        }

        Ok(json)
    }

    fn extract_encrypted(json: &serde_json::Value) -> Result<serde_json::Value, ProviderError> {
        let encrypted = json
            .pointer("/jsonr/data/encryptResult")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ProviderError::Parse("missing encryptResult".into()))?;
        Self::decrypt_aes(encrypted)
    }

    fn extract_data(json: &serde_json::Value) -> Result<serde_json::Value, ProviderError> {
        json.pointer("/jsonr/data")
            .cloned()
            .ok_or_else(|| ProviderError::Parse("missing jsonr.data".into()))
    }

    pub async fn nearby_lines(&self, lat: f64, lng: f64) -> Result<NearbyResponse, ProviderError> {
        let lat_s = lat.to_string();
        let lng_s = lng.to_string();

        let sign_params = [
            ("cityId", self.city_id.as_str()),
            ("lat", &lat_s),
            ("lng", &lng_s),
            ("gpstype", "gcj"),
        ];
        let sign = Self::crypto_sign(&sign_params);

        let common = self.common_params();
        let mut params: Vec<(&str, &str)> = vec![
            ("cryptoSign", &sign),
            ("lat", &lat_s),
            ("lng", &lng_s),
            ("geo_lat", &lat_s),
            ("geo_lng", &lng_s),
            ("gpstype", "gcj"),
        ];
        for (k, v) in &common {
            params.push((k, v.as_str()));
        }

        let json = self
            .get("bus/stop!encryptedNearlines.action", &params)
            .await?;
        let decrypted = Self::extract_encrypted(&json)?;
        serde_json::from_value(decrypted)
            .map_err(|e| ProviderError::Parse(format!("nearlines: {e}")))
    }

    pub async fn line_detail(
        &self,
        line_id: &str,
        line_name: &str,
        direction: &str,
        station_name: &str,
        next_station_name: &str,
        line_no: &str,
        target_order: u32,
    ) -> Result<LineDetailResponse, ProviderError> {
        let order_s = target_order.to_string();

        let sign_params = [
            ("lineId", line_id),
            ("lineName", line_name),
            ("direction", direction),
            ("stationName", station_name),
            ("nextStationName", next_station_name),
            ("lineNo", line_no),
            ("targetOrder", order_s.as_str()),
        ];
        let sign = Self::crypto_sign(&sign_params);

        let common = self.common_params();
        let mut params: Vec<(&str, &str)> = vec![
            ("lineId", line_id),
            ("lineName", line_name),
            ("direction", direction),
            ("stationName", station_name),
            ("nextStationName", next_station_name),
            ("lineNo", line_no),
            ("targetOrder", &order_s),
            ("cryptoSign", &sign),
        ];
        for (k, v) in &common {
            params.push((k, v.as_str()));
        }

        let json = self
            .get("bus/line!encryptedLineDetail.action", &params)
            .await?;
        let decrypted = Self::extract_encrypted(&json)?;
        serde_json::from_value(decrypted)
            .map_err(|e| ProviderError::Parse(format!("lineDetail: {e}")))
    }

    pub async fn line_route(&self, line_id: &str) -> Result<LineRouteResponse, ProviderError> {
        let common = self.common_params();
        let mut params: Vec<(&str, &str)> = vec![("lineId", line_id)];
        for (k, v) in &common {
            params.push((k, v.as_str()));
        }

        let json = self.get("bus/line!lineRoute.action", &params).await?;
        let data = Self::extract_data(&json)?;
        serde_json::from_value(data).map_err(|e| ProviderError::Parse(format!("lineRoute: {e}")))
    }

    pub async fn search_lines(&self, line_name: &str) -> Result<SearchResponse, ProviderError> {
        let common = self.common_params();
        let mut params: Vec<(&str, &str)> = vec![("lineName", line_name)];
        for (k, v) in &common {
            params.push((k, v.as_str()));
        }

        let json = self.get("bus/cityLineList", &params).await?;
        let data = Self::extract_data(&json)?;
        serde_json::from_value(data).map_err(|e| ProviderError::Parse(format!("cityLineList: {e}")))
    }
}
