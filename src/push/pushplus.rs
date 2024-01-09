use super::Push;
use serde_json::{json, Value};
pub struct PushPlus {
    token: String,
}

impl Push for PushPlus {
    async fn send_message(
        &self,
        message: String,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        let url = format!("http://www.pushplus.plus/send/{}", self.token);

        let params = json!({
            "title": title,
            "content": message,
            "channel": "wechat",
            "template": "markdown",
        });
        let response = client
            .post(url)
            .query(&params)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response
            .get("code")
            .unwrap_or(&Value::from(500))
            .as_i64()
            .unwrap_or(500)
            == 200
        {
            return Ok(true);
        }
        Ok(false)
    }
}

impl PushPlus {
    pub fn new(token: &str) -> Self {
        PushPlus {
            token: token.to_string(),
        }
    }
}

// impl Default for PushPlus {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
    #[test]
    fn test_send_message() {
        let p = PushPlus::new("");
        let r = aw!(p.send_message("测试".to_string(), "nyar 推送".to_string())).unwrap();
        println!("result :{}", r);
    }
}
