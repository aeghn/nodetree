use async_trait::async_trait;

use crate::model::asset::Asset;

#[async_trait]
pub trait AssetMapper {
    fn generate_asset_id(&self) -> String {
        uuid::Uuid::new_v4().to_string().replace("-", "")
    }

    async fn insert_asset(
        &self,
        ori_file_name: &str,
        id: String,
        content_type: String,
        domain: Option<String>,
    ) -> anyhow::Result<Asset>;

    async fn query_asset_by_id(&self, id: &str) -> anyhow::Result<Asset>;
}
