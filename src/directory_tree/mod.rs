pub mod foldernodes;
pub mod defaultdirectory;

pub use foldernodes::*; 
pub use defaultdirectory::*;


pub fn create_directory_tree_from_json(project_name: &str,json_data: &serde_json::Value) -> Result<FolderNode, serde_json::Error> {
    let children: Vec<FolderNode> = serde_json::from_value(json_data.clone())?;
    Ok(FolderNode::Folder {
        name: project_name.to_string(),
        children,
    })
}

pub fn default_serialize_directory_tree(project_name: &str) -> Result<FolderNode, serde_json::Error> {
    create_directory_tree_from_json(project_name, &defaultdirectory::DEFAULT_TEMPLATE)
}