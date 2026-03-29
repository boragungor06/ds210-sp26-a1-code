use std::collections::HashMap;
use std::hash::Hash;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
   
    let mut aggregated: HashMap<Value, Value> = HashMap::new();
    
    // you may notice that each for loop here consumes (takes ownership) of the object being looped over.
    // since we only need to insert aggregated data to a new HashMap, the old data can be destroyed; it is not needed

    for (value, dataset) in dataset.into_iter() {
        // for every group, aggregate it
        let result = match aggregation {
            Aggregation::Count(_col_name) => {
                // if the aggregation method is count, then use .len() to get the length of the column; that is, the number of elements within it
                Value::Integer(dataset.len() as i32)
            }

            Aggregation::Sum(col_name) => {
                // if the aggregation method is sum, then sum each entry under the column
                let col_idx = dataset.column_index(col_name);
                let mut total = 0;

                for row in dataset.into_iter() {
                    if let Value::Integer(v) = row.get_value(col_idx) {
                        total += v
                    }
                }
                Value::Integer(total)
            }

            Aggregation::Average(col_name) => {
                // if the aggregation method is average, then sum each entry under the column and divide by the count
                let col_idx = dataset.column_index(col_name);
                let mut total = 0;
                let count = dataset.len() as i32;

                for row in dataset.into_iter() {
                    if let Value::Integer(v) = row.get_value(col_idx) {
                        total += v
                    }
                }
                if count != 0 {Value::Integer(total/count)} else {Value::Integer(0)} // account for if count is 0
            }
        
        };
        aggregated.insert(value, result); // for each group, pair with it only the aggregated result
    }
    return aggregated // ez
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}