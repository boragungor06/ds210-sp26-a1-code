use analytics_lib::dataset::Value;
extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let mut words: Vec<&str> = Vec::new();
    for word in input.split_whitespace() {
        words.push(word);
    }
    
    // The Query object is comprised of three components: 
    // (1) the condition, (2) the group-by, and (3) the aggregation method

    // CONDITION:

    // first, notice that the FILTER condition lies strictly between "FILTER" and "GROUP BY" in the query. so, first we extract
    // this component of the string

    let mut start_idx = 0;
    let mut end_idx = 0;
    
    for i in 0..words.len() {
        if words[i] == "FILTER" {
            start_idx = i + 1
        }
        else if words[i] == "GROUP" {
            end_idx = i;
            break;
        }
    }

    // the vec containing only the condition tokens
    let condition_as_vec = words.clone()[start_idx..end_idx].to_vec();
    
    let final_filter: Condition; // this will hold the filter condition

    // now, we want to divide this substring into smaller instructions, divided by AND or OR
    // so that each instruction within each OR/AND clause is seperate
    // crucially but limiting, I will assume that there can only be ONE and/or; no recursion

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut operator = "";

    for i in  0..condition_as_vec.len() {
        if condition_as_vec[i] == "AND" {
            left = condition_as_vec[0..i].to_vec();
            right = condition_as_vec[i+1..condition_as_vec.len()].to_vec();
            operator = "AND";
            break;
        }
        else if condition_as_vec[i] == "OR"  {
            left = condition_as_vec[0..i].to_vec();
            right = condition_as_vec[i+1..condition_as_vec.len()].to_vec();
            operator = "OR";
            break;
        }; 
    }
    // now, we can start parsing the condition. this code is long.
    // but, it simply repeats the same logic 3 times: for left and for right, or for only the original argument
    // in this segment, we accomplish: (1) cleaning off punctuation from the query, 
    // (2) feeding the resulting strings directly into a Condition object
    // (3) handling the Condition for there was an AND, an OR, or neither

    // for LEFT and RIGHT
    if operator == "AND" || operator == "OR" {
        // LEFT
        let mut left_first = left[0];
        let mut left_last = left[left.len() - 1];
        let mut left_is_not = false;
    
        // remove punctuation from the front of the first word
        while left_first.starts_with('(') || left_first.starts_with('!') || left_first.starts_with('"') {
            if left_first.starts_with('!') { 
                left_is_not = true; 
            }
            left_first = &left_first[1..]; // move up one char to remove the punctuation
        }

        // remove punctuation from the end of the last word
        while left_last.ends_with(')') || left_last.ends_with('"') {
                left_last = &left_last[..left_last.len()-1];
        }

        // remove punctuation from the front of the last word
        while left_last.starts_with('"') {
            left_last = &left_last[1..];
        }

        // creating the Condition
        let mut left_condition = Condition::Equal(
            left_first.to_string(), 
            Value::String(left_last.to_string())
        );

        if left_is_not {
            left_condition = Condition::Not(Box::new(left_condition));
        }

        // RIGHT
        let mut right_first = right[0];
        let mut right_last = right[right.len() - 1];
        let mut right_is_not = false;
    
        // remove punctuation from the front of the first word
        while right_first.starts_with('(') || right_first.starts_with('!') || right_first.starts_with('"') {
            if right_first.starts_with('!') { 
                right_is_not = true; 
            }
            right_first = &right_first[1..];
        }

        // remove punctuation from the end of the last word
        while right_last.ends_with(')') || right_last.ends_with('"') {
            if !right_last.is_empty() {
                right_last = &right_last[..right_last.len() - 1];
            } else { break; }
        }

        // remove punctuation from the front of the last word
        while right_last.starts_with('"') {
            if !right_last.is_empty() {
                right_last = &right_last[1..];
            } else { break; }
        }

        // creating the Condition
        let mut right_condition = Condition::Equal(
            right_first.to_string(), 
            Value::String(right_last.to_string())
        );

        if right_is_not {
            right_condition = Condition::Not(Box::new(right_condition));
        }

        if operator == "AND" {
            final_filter = Condition::And(Box::new(left_condition), Box::new(right_condition));
        } else {
            final_filter = Condition::Or(Box::new(left_condition), Box::new(right_condition));
        }

    }
    else {
        // FOR THE ORIGINAL ARGUMENT
        let mut first = condition_as_vec[0];
        let mut last = condition_as_vec[condition_as_vec.len() - 1];
        let mut is_not = false;
    
        // remove punctuation from the front of the first word
        while first.starts_with('(') || first.starts_with('!') || first.starts_with('"') {
            if first.is_empty() { break; }
            if first.starts_with('!') { is_not = true; }
            first = &first[1..];
        }

        // remove punctuation from the end of the last word
        while last.ends_with(')') || last.ends_with('"') {
            if last.is_empty() { break; }
            last = &last[..last.len() - 1];
        }

        // remove punctuation from the front of the last word
        while last.starts_with('"') {
            if last.is_empty() { break; }
            last = &last[1..];
        }

        // creating the Condition
        let mut base = Condition::Equal(
            first.to_string(), 
            Value::String(last.to_string())
        );

        if is_not {
            base = Condition::Not(Box::new(base));
        }

        final_filter = base;
    }

    // AGGREGATION:
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

    // GROUP-BY:
    for i in 0..words.len() {
        if words[i] == "GROUP" {
            location_of_group = i;
            break;
        }
        }
    let word_after_groupby = words[location_of_group + 2].to_string();
    // this code gets the word after group by => "GROUP by class", this gets the "class"
    
    // now that we have the condition, the group_by, and the aggregation method, simply combine them into final Query
    let x = Query::new(final_filter, word_after_groupby, operation);
    return x
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