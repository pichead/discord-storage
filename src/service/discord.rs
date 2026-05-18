use async_trait::async_trait;

pub struct DiscordService {
    pub(crate) discord_base_url: String,
}

impl DiscordService {
    pub fn new() -> Self {
        Self {
            discord_base_url: "https://cdn.discordapp.com".to_string(),
        }
    }
}

#[async_trait]
pub trait DiscordServiceTrait {
    fn get_new_file_cdn_url(&self, file_id: &str) -> String;
    fn send_message(&self, channel_id: &str, message: &str) -> String;
}

impl DiscordServiceTrait for DiscordService {
    fn get_new_file_cdn_url(&self, file_id: &str) -> String {
        format!("{0}/attachments/{file_id}?filename={file_id}", self.discord_base_url)
    }

    fn send_message(&self, channel_id: &str, message: &str) -> String {
        format!("{0}/channels/{1}/messages={2}", self.discord_base_url, channel_id,message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_file_cdn_url() {
        let service = DiscordService::new();
        let url = service.get_new_file_cdn_url("123");
        assert_eq!(url, "https://cdn.discordapp.com/attachments/123?filename=123");
    }

    #[test]
    fn test_send_message() {
        let service = DiscordService::new();
        let result = service.send_message("channel1", "hello");
        assert_eq!(result, "https://cdn.discordapp.com/channels/channel1/messages=hello");
    }
}
