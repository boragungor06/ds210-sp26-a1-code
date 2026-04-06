extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::{Aggregation, Query};
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let mut words: Vec<&str> = Vec::new();
    for word in input.split_whitespace() {
        words.push(word);
    }
    
    let operation_category = words[words.len()-1].to_string();
    let operation_type = words[words.len()-2];
    let mut operation = Aggregation::Count(String::from("dummy"));
    match operation_type {
        "COUNT" => operation = Aggregation::Count(operation_category),
        "SUM" => operation = Aggregation::Sum(operation_category),
        "AVERAGE" => operation = Aggregation::Average(operation_category),
        _ => panic!("Error"),
    }
    let mut location_of_group = 0;

    for i in 0..words.len() {
        if words[i] == "GROUP" {
            location_of_group = i;
            break;
        }
        }
    let word_after_groupby = words[location_of_group + 2].to_string();
    // this code gets the word after group by => "GROUP by class", this gets the "class"


    }


// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}