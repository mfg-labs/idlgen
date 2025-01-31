use crate::{generators::common::{indent, make_defined_types_fields}, types::Accounts, IDL};
use convert_case::{Casing, Case};

pub fn make_accounts(idl: &IDL) -> String {
    format!("pub mod accounts {{
    #![allow(unused)]
    use super::*;

{}  
}}",
    idl.accounts.iter().map(|account| {
        let account_name_pascal = account.name.to_case(Case::Pascal);
        let fields = match account.kind.fields.is_empty() {
            true => if let Some(matched_type) = idl.types.iter().find(|t| t.name == account_name_pascal) {
                        indent(make_defined_types_fields(matched_type.clone()))
                    } else {
                        make_account_props(account)
                    },
            false => make_account_props(account)
        };

        format!("    #[account]
    pub struct {} {{
{}
    }}", account_name_pascal, fields)

        }).collect::<Vec<String>>().join("\n\n")
    )
}

pub fn make_account_props(account: &Accounts) -> String {
    account.kind.fields.iter().map(|t| format!("        pub {}: {}", t.name.to_case(Case::Snake), t.kind.to_string())).collect::<Vec<String>>().join(",\n")
}