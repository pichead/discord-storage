use async_trait::async_trait;

struct DiscordService {
    pub(crate) discord_base_url: String,
}

#[async_trait]
pub trait DiscordServiceTrait {
    fn get_new_file_cdn_url(&self, file_id: &str) -> String;
}

impl DiscordServiceTrait for DiscordService {
    fn get_new_file_cdn_url(&self, file_id: &str) -> String {
        format!("{self.discord_base_url}/attachments/{file_id}?filename={file_id}")
    }
}
