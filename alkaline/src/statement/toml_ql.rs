//! A mostly temporary helper language to allow dynamic statement/query construction from Repl, CLI,
//! etc without being blocked by bikeshedding on AlkQL language design.
//!
//! This module will focus on general quality of life TOML API design, but it is expected that some
//! Alkaline abstractions will be limited in capability.
use super::Statement;
use crate::statement::Show;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlkTomlQl {
    #[serde(rename = "statement", alias = "stmt")]
    statements: Vec<AlkTomlStatement>,
}
impl AlkTomlQl {
    pub fn parse(s: &str) -> Result<Self, anyhow::Error> {
        let self_: Self = toml::from_str(s)?;
        Ok(self_)
    }
    pub fn to_statements(self) -> Vec<Statement> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlkTomlStatement {
    Show(AlkTomlShow),
    Create(AlkTomlCreate),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlkTomlShow {
    Database,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlkTomlCreate {
    Database {
        name: String,
        connection: Option<String>,
    },
}

#[test]
fn shows() {
    // assert_eq!(
    //     AlkTomlQl::parse(
    //         r#"
    //         [[stmt]]
    //         show = "database"
    //     "#
    //     )
    //     .unwrap()
    //     .to_statements(),
    //     vec![Statement::Show(Show::Databases)]
    // )
    assert_eq!(
        AlkTomlQl::parse(
            r#"
            [[stmt]]
            [create.database]
            name = "foo"
        "#
        )
        .unwrap()
        .to_statements(),
        vec![]
    )
}
