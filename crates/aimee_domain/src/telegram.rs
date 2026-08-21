/// Telegram Bot API `sendMessage` request. The token is only used to build
/// the URL and is never stored on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramSendRequest {
    /// Full `sendMessage` URL (contains the bot token — do not log).
    pub api_url: String,
    /// Chat or group id.
    pub chat_id: String,
    /// Message body.
    pub text: String,
}

/// Why a Telegram send request could not be built.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TelegramError {
    /// `TELEGRAM_BOT_TOKEN` missing or empty.
    #[error("TELEGRAM_BOT_TOKEN is missing")]
    MissingToken,
    /// Chat id is empty.
    #[error("Telegram chat id is empty")]
    MissingChat,
    /// Message body is empty.
    #[error("Telegram message is empty")]
    MissingText,
    /// `getUpdates` JSON could not be parsed.
    #[error("Telegram getUpdates payload is invalid")]
    InvalidUpdates,
}

impl TelegramSendRequest {
    /// Builds a send request. Token is interpolated into the URL only.
    pub fn new(
        token: &str,
        chat_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<Self, TelegramError> {
        let token = token.trim();
        if token.is_empty() {
            return Err(TelegramError::MissingToken);
        }
        let chat_id = chat_id.into();
        if chat_id.trim().is_empty() {
            return Err(TelegramError::MissingChat);
        }
        let text = text.into();
        if text.trim().is_empty() {
            return Err(TelegramError::MissingText);
        }
        Ok(Self {
            api_url: format!("https://api.telegram.org/bot{token}/sendMessage"),
            chat_id,
            text,
        })
    }

    /// Safe log label that never includes the token.
    pub fn redacted_url() -> &'static str {
        "https://api.telegram.org/bot***/sendMessage"
    }

    /// Form fields posted to the Bot API.
    pub fn form(&self) -> [(&str, &str); 2] {
        [
            ("chat_id", self.chat_id.as_str()),
            ("text", self.text.as_str()),
        ]
    }
}

/// Bot API `getUpdates` request. Token is only used to build the URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramGetUpdatesRequest {
    /// Full `getUpdates` URL (contains the bot token — do not log).
    pub api_url: String,
    /// Optional exclusive cursor (`offset`).
    pub offset: Option<i64>,
}

impl TelegramGetUpdatesRequest {
    /// Builds a poll request.
    pub fn new(token: &str, offset: Option<i64>) -> Result<Self, TelegramError> {
        let token = token.trim();
        if token.is_empty() {
            return Err(TelegramError::MissingToken);
        }
        Ok(Self {
            api_url: format!("https://api.telegram.org/bot{token}/getUpdates"),
            offset,
        })
    }

    /// Safe log label that never includes the token.
    pub fn redacted_url() -> &'static str {
        "https://api.telegram.org/bot***/getUpdates"
    }

    /// Query string pairs (`timeout=0` so the call returns immediately).
    pub fn query(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            ("timeout".into(), "0".into()),
            ("allowed_updates".into(), r#"["message"]"#.into()),
        ];
        if let Some(offset) = self.offset {
            pairs.push(("offset".into(), offset.to_string()));
        }
        pairs
    }
}

/// One inbound text message from `getUpdates`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramInbound {
    /// Telegram update id.
    pub update_id: i64,
    /// Chat or group id as a string.
    pub chat_id: String,
    /// Message body.
    pub text: String,
    /// Optional sender username.
    pub from: Option<String>,
}

impl TelegramInbound {
    /// Prompt injected when this update is handed to the agent.
    pub fn as_user_prompt(&self) -> String {
        match &self.from {
            Some(from) => format!("[telegram {} @{}]\n{}", self.chat_id, from, self.text),
            None => format!("[telegram {}]\n{}", self.chat_id, self.text),
        }
    }
}

/// Parses a `getUpdates` body. Returns inbound texts and the next offset.
pub fn parse_get_updates(body: &str) -> Result<(Vec<TelegramInbound>, Option<i64>), TelegramError> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|_| TelegramError::InvalidUpdates)?;
    if value.get("ok").and_then(|ok| ok.as_bool()) == Some(false) {
        return Err(TelegramError::InvalidUpdates);
    }
    let mut inbound = Vec::new();
    let results = value.get("result").and_then(|r| r.as_array());
    let Some(results) = results else {
        return Ok((inbound, None));
    };
    for item in results {
        let Some(update_id) = item.get("update_id").and_then(|id| id.as_i64()) else {
            continue;
        };
        let message = item.get("message").or_else(|| item.get("channel_post"));
        let Some(message) = message else {
            continue;
        };
        let Some(text) = message.get("text").and_then(|t| t.as_str()) else {
            continue;
        };
        if text.trim().is_empty() {
            continue;
        }
        let chat_id = message
            .get("chat")
            .and_then(|c| c.get("id"))
            .map(|id| match id {
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::String(s) => s.clone(),
                _ => String::new(),
            })
            .unwrap_or_default();
        if chat_id.is_empty() {
            continue;
        }
        let from = message
            .get("from")
            .and_then(|f| f.get("username").or_else(|| f.get("first_name")))
            .and_then(|n| n.as_str())
            .map(str::to_string);
        inbound.push(TelegramInbound { update_id, chat_id, text: text.to_string(), from });
    }
    let next = inbound
        .iter()
        .map(|item| item.update_id)
        .max()
        .map(|id| id + 1);
    Ok((inbound, next))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_telegram_send_request_builds_url_and_form() {
        let actual = TelegramSendRequest::new("tok:en", "-1001", "hello").unwrap();
        let expected = TelegramSendRequest {
            api_url: "https://api.telegram.org/bottok:en/sendMessage".into(),
            chat_id: "-1001".into(),
            text: "hello".into(),
        };
        assert_eq!(actual, expected);
        assert_eq!(actual.form(), [("chat_id", "-1001"), ("text", "hello")]);
        assert!(!TelegramSendRequest::redacted_url().contains("tok:en"));
    }

    #[test]
    fn test_telegram_send_request_rejects_empty() {
        let actual = TelegramSendRequest::new("", "1", "x").unwrap_err();
        assert_eq!(actual, TelegramError::MissingToken);
    }

    #[test]
    fn test_parse_get_updates_extracts_text_and_next_offset() {
        let fixture = r#"{"ok":true,"result":[{"update_id":10,"message":{"chat":{"id":-1001},"text":"hi","from":{"username":"og"}}}]}"#;
        let (actual, next) = parse_get_updates(fixture).unwrap();
        let expected = vec![TelegramInbound {
            update_id: 10,
            chat_id: "-1001".into(),
            text: "hi".into(),
            from: Some("og".into()),
        }];
        assert_eq!(actual, expected);
        assert_eq!(next, Some(11));
    }

    #[test]
    fn test_get_updates_request_redacts_token() {
        let actual = TelegramGetUpdatesRequest::new("tok:en", Some(7)).unwrap();
        assert!(actual.api_url.contains("tok:en"));
        assert!(!TelegramGetUpdatesRequest::redacted_url().contains("tok:en"));
        assert_eq!(actual.offset, Some(7));
    }
}
