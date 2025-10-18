use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum FolderNode {
    #[serde(rename = "file")]
    File { name: String }, // represents a file with its name
    #[serde(rename = "folder")]
    Folder { name: String, children: Vec<FolderNode> }, // represents a folder with its name and its children
}

impl FolderNode {
    pub fn name(&self) -> &String {
        match self {
            FolderNode::File { name } => name,
            FolderNode::Folder { name, .. } => name,
        }
    }

    pub fn children(&self) -> Option<&Vec<FolderNode>> {
        match self {
            FolderNode::File { .. } => None,
            FolderNode::Folder { children, .. } => Some(children),
        }
    }

    pub fn add_child(&mut self, child: FolderNode) {
        if let FolderNode::Folder { name, children } = self {   
            children.push(child);
        } else {
            panic!("Cannot add a child to a file node");
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FolderNode::File { .. })
    }

    pub fn is_folder(&self) -> bool {
        matches!(self, FolderNode::Folder { .. })
    }

    pub fn print_tree(&self) {
        self.print_tree_with_indent(0);
    }

    pub fn print_tree_with_indent(&self, indent: usize) {
        let indentation = " ".repeat(indent);
        match self {
            FolderNode::File { name } => {
                println!("{}- {}", indentation, name);
            }
            FolderNode::Folder { name, children } => {
                println!("{}+ {}", indentation, name);
                for child in children {
                    child.print_tree_with_indent(indent + 2);
                }
            }
        }
    }

    /// Find a node by name (depth-first search)
    pub fn find_by_name(&self, target_name: &str) -> Option<&FolderNode> {
        if self.name() == target_name {
            return Some(self);
        }
        
        if let Some(children) = self.children() {
            for child in children {
                if let Some(found) = child.find_by_name(target_name) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find a node by path (e.g., "folder1/subfolder/file.txt")
    pub fn find_by_path(&self, path: &str) -> Option<&FolderNode> {
        let parts: Vec<&str> = path.split('/').collect();
        self.find_by_path_parts(&parts)
    }

    fn find_by_path_parts(&self, parts: &[&str]) -> Option<&FolderNode> {
        if parts.is_empty() {
            return Some(self);
        }

        let current_part = parts[0];
        let remaining_parts = &parts[1..];

        if self.name() == current_part {
            if remaining_parts.is_empty() {
                return Some(self);
            }
            
            if let Some(children) = self.children() {
                for child in children {
                    if let Some(found) = child.find_by_path_parts(remaining_parts) {
                        return Some(found);
                    }
                }
            }
        }
        None
    }

    /// Get a child by index
    pub fn get_child(&self, index: usize) -> Option<&FolderNode> {
        if let Some(children) = self.children() {
            children.get(index)
        } else {
            None
        }
    }

    /// Get a child by name (direct children only)
    pub fn get_child_by_name(&self, name: &str) -> Option<&FolderNode> {
        if let Some(children) = self.children() {
            children.iter().find(|child| child.name() == name)
        } else {
            None
        }
    }

    /// Get all files in the tree (recursive)
    pub fn get_all_files(&self) -> Vec<&FolderNode> {
        let mut files = Vec::new();
        self.collect_files(&mut files);
        files
    }

    fn collect_files<'a>(&'a self, files: &mut Vec<&'a FolderNode>) {
        match self {
            FolderNode::File { .. } => files.push(self),
            FolderNode::Folder { children, .. }  => {
                for child in children {
                    child.collect_files(files);
                }
            }
        }
    }

    /// Get all folders in the tree (recursive)
    pub fn get_all_folders(&self) -> Vec<&FolderNode> {
        let mut folders = Vec::new();
        self.collect_folders(&mut folders);
        folders
    }

    fn collect_folders<'a>(&'a self, folders: &mut Vec<&'a FolderNode>) {
        match self {
            FolderNode::File { .. } => {},
            FolderNode::Folder { children, .. } => {
                folders.push(self);
                for child in children {
                    child.collect_folders(folders);
                }
            }
        }
    }

    /// Get the depth of a specific node in the tree
    pub fn get_depth(&self, target_name: &str) -> Option<usize> {
        self.find_depth(target_name, 0)
    }

    fn find_depth(&self, target_name: &str, current_depth: usize) -> Option<usize> {
        if self.name() == target_name {
            return Some(current_depth);
        }
        
        if let Some(children) = self.children() {
            for child in children {
                if let Some(depth) = child.find_depth(target_name, current_depth + 1) {
                    return Some(depth);
                }
            }
        }
        None
    }


}


impl Serialize for FolderNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FolderNode::File { name} => {
                let mut state = serializer.serialize_struct("File", 2)?;
                state.serialize_field("type", "file")?;
                state.serialize_field("name", name)?;
                state.end()
            }
            FolderNode::Folder { name, children }   => {
                let mut state = serializer.serialize_struct("Folder", 3)?;
                state.serialize_field("type", "folder")?;
                state.serialize_field("name", name)?;
                state.serialize_field("children", children)?;
                state.end()
            }
        }
    }
}   


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_node() {
        let mut root = FolderNode::Folder { name: "root".into(), children: vec![] };
        let file1 = FolderNode::File { name: "file1.txt".into() };
        let file2 = FolderNode::File { name: "file2.txt".into() };
        let folder1 = FolderNode::Folder { name: "folder1".into(), children: vec![] };
        let folder2 = FolderNode::Folder { name: "folder2".into(), children: vec![] };

        // Test before adding to parent
        assert_eq!(file1.is_file(), true);
        assert_eq!(folder1.is_folder(), true);

        root.add_child(file1);
        root.add_child(file2);
        root.add_child(folder1);
        root.add_child(folder2);

        assert_eq!(root.name(), "root");
        assert_eq!(root.children().unwrap().len(), 4);
        assert_eq!(root.is_folder(), true);
    }

    #[test]
    fn test_tree_access_methods() {
        let mut root = FolderNode::Folder { name: "root".into(), children: vec![] };
        let mut src = FolderNode::Folder { name: "src".into(), children: vec![] };
        let main_rs = FolderNode::File { name: "main.rs".into() };
        let lib_rs = FolderNode::File { name: "lib.rs".into() };
        let readme = FolderNode::File { name: "README.md".into() };

        src.add_child(main_rs);
        src.add_child(lib_rs);
        root.add_child(src);
        root.add_child(readme);

        // Test find by name
        assert!(root.find_by_name("main.rs").is_some());
        assert!(root.find_by_name("nonexistent").is_none());

        // Test find by path
        assert!(root.find_by_path("root/src/main.rs").is_some());
        assert!(root.find_by_path("root/nonexistent").is_none());

        // Test get child by name
        assert!(root.get_child_by_name("src").is_some());
        assert!(root.get_child_by_name("nonexistent").is_none());

        // Test get all files
        let files = root.get_all_files();
        assert_eq!(files.len(), 3); // main.rs, lib.rs, README.md

        // Test get all folders
        let folders = root.get_all_folders();
        assert_eq!(folders.len(), 2); // root, src

        // Test depth
        assert_eq!(root.get_depth("root"), Some(0));
        assert_eq!(root.get_depth("main.rs"), Some(2));
    }}