use tracing::error;

use super::{
    table::{Assets, Nodes, NodesHistory, Table},
    BackupContent, RowSum,
};

impl TryFrom<RowSum> for NodesHistory {
    type Error = anyhow::Error;

    fn try_from(value: RowSum) -> Result<Self, Self::Error> {
        match value {
            RowSum::Pg(row) => Ok(NodesHistory {
                id: row.try_get("id")?,
                delete_time: row.try_get("delete_time")?,
                name: row.try_get("name")?,
                content: row.try_get("content")?,
                node_type: crate::model::node::NodeType::TiptapV1,
                domain: row.try_get("domain")?,
                version_time: row.try_get("version_time")?,
                initial_time: row.try_get("initial_time")?,
            }),
        }
    }
}

impl TryFrom<RowSum> for Nodes {
    type Error = anyhow::Error;

    fn try_from(value: RowSum) -> Result<Nodes, Self::Error> {
        match value {
            RowSum::Pg(row) => Ok(Nodes {
                id: row.try_get("id")?,
                delete_time: row.try_get("delete_time")?,
                name: row.try_get("name")?,
                content: row.try_get("content")?,
                node_type: crate::model::node::NodeType::TiptapV1,
                domain: row.try_get("domain")?,
                version_time: row.try_get("version_time")?,
                initial_time: row.try_get("initial_time")?,
                parent_id: row.try_get("parent_id")?,
                prev_sliding_id: row.try_get("prev_sliding_id")?,
            }),
        }
    }
}

impl TryFrom<RowSum> for Assets {
    type Error = anyhow::Error;

    fn try_from(value: RowSum) -> Result<Assets, Self::Error> {
        match value {
            RowSum::Pg(row) => Ok(Assets {
                id: row.try_get("id")?,
                ori_file_name: row.try_get("ori_file_name")?,
                domain: row.try_get("domain")?,
                create_time: row.try_get("create_time")?,
                content_type: row.try_get("content_type")?,
            }),
        }
    }
}

pub fn row_to_table(row_type: &BackupContent, row: RowSum) -> anyhow::Result<Table> {
    match row_type {
        BackupContent::Nodes => Ok(Table::Nodes(row.try_into()?)),
        BackupContent::NodesHistory => Ok(Table::NodesHistory(row.try_into()?)),
        BackupContent::Assets => Ok(Table::Assets(row.try_into()?)),
        _ => {
            anyhow::bail!("unable map thie row to table")
        }
    }
}
