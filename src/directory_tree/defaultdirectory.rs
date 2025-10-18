use serde_json::json;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_TEMPLATE: serde_json::Value = json!([
        {
            "type": "folder",
            "name": "simulations",
            "children": [
                {
                    "type": "folder",
                    "name": "raw",
                    "children": []
                },
                {
                    "type": "folder",
                    "name": "processed",
                    "children": []
                },
                {
                    "type": "folder",
                    "name": "syntetic",
                    "children": []
                }
            ]
        },
        {
            "type": "folder",
            "name": "results",
            "children": []
        },
        {
            "type": "folder",
            "name": "scripts",
            "children": []
        }
    ]);
}