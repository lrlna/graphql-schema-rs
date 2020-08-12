use graphql_client::GraphQLQuery;
use prettytable::Table;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;
use structopt::StructOpt;
use textwrap;

use std::error::Error;
#[macro_use]
extern crate prettytable;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
struct ExploreQuery;

#[derive(Debug, Deserialize)]
struct Schema {
    data: HashMap<String, HashMap<String, Vec<FullType>>>,
}

#[derive(Debug, Deserialize)]
struct FullType {
    description: Option<String>,
    kind: Option<String>,
    name: Option<String>,
    // this should eventually include nested fields
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "graphql-explorer",
    about = "Explore GraphQL schema given a URL"
)]
struct Cli {
    /// GraphQL Endpoint URL to explore the schema.
    schema_url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    explore_schema(&args.schema_url)?;
    Ok(())
}

pub fn explore_schema(schema_url: &str) -> Result<(), Box<dyn Error>> {
    // use synchronous reqwest (^0.9) to avoid using async rust
    let client = reqwest::Client::builder().build()?;

    let request_body: graphql_client::QueryBody<()> = graphql_client::QueryBody {
        variables: (),
        query: explore_query::QUERY,
        operation_name: explore_query::OPERATION_NAME,
    };

    let req_builder = client.post(schema_url).headers(construct_headers());
    // when eventually accepting an auth token add it to the builder before sending a request
    // req_builder = req_builder.bearer_auth(token.as_str());
    let mut res = req_builder.json(&request_body).send()?;

    if res.status().is_success() {
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }

    let mut schema: Schema = serde_json::from_str(&res.text()?)?;
    format_schema_fields(&mut schema);
    Ok(())
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}

fn format_schema_fields(res: &mut Schema) {
    // Create the table
    let mut table = Table::new();
    table.add_row(row!["NAME", "KIND", "DESCRIPTION"]);
    if let Some(schema) = res.data.get_mut("__schema") {
        if let Some(types) = schema.get_mut("types") {
            for t in types {
                let or = &"null".to_string();
                let name = t.name.as_ref().unwrap_or(or);
                let kind = t.kind.as_ref().unwrap_or(or);
                let description = t.description.as_ref().unwrap_or(or);
                table.add_row(row![name, kind, textwrap::fill(description, 60)]);
            }
        }
    }

    table.printstd();
}
