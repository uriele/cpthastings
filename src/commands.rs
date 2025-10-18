use crate::directory_tree;

pub struct CMDInit;

impl CMDInit {
    pub fn run_init(name: &str, git: bool, readme: bool, authors: Vec<String>, force: bool, add_test: bool, add_docs: bool, template: Option<String>) {
        println!("Initializing repository: {}", name);
        // Further implementation goes here

        match template {
            Some(t) => {
                println!("Using custom template: {}", t);
                // Parse and use the custom template
            },
            None => {
                println!("Using default template");
                // Use the default template
                let root = directory_tree::default_serialize_directory_tree(name).unwrap();
                root.print_tree();
            }
        }   

    }
}