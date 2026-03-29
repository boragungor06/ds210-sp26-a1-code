use std::collections::HashMap;
use std::hash::Hash;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn condition_check(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
    // we need to check if the entry satisfies the condition
    match condition {
        // since there are 4 conditions and we need to be exhaustive match is a good option
        Condition::Equal(column_name, value) => {
            let index_number = dataset.column_index(column_name);
            // which column index the data we want to check
            let value_inside = row.get_value(index_number);
            // what is the data at that index number in that row
            return value == value_inside
            //if the value inside the condition and our data in dataset is the same true
        }
        Condition::Not(cond1) => {
            // just the opposite of equal
            return !condition_check(row,cond1,dataset);
        }
        Condition::And(cond1, cond2) => {
            let result_cond1 = condition_check(row,cond1,dataset);
            let result_cond2 = condition_check(row,cond2,dataset);
            // for end I checked two conditions seperately and returned the results
            if result_cond1 == true {
                return result_cond1 == result_cond2
            } else {
                return false
            }
        }
        Condition::Or(cond1, cond2) => {
            let result_cond1 = condition_check(row,cond1,dataset);
            let result_cond2 = condition_check(row,cond2,dataset);
            // same as and but just in OR format
            if result_cond1 == true {
                return result_cond1
            } else {
                return result_cond2
            }
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    //todo!("Implement this!");
    let mut new_dataset = Dataset::new(dataset.columns().clone());
    // creating a new data set to move filtered information
    for row in dataset.iter() {
        // iterating over the rows to check if it satisfies over condition
        if condition_check(row,filter,dataset){
            new_dataset.add_row(row.clone());
            // if it satisfies the condition we add that row to our new dataset
        }
    }
    return new_dataset

}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    //todo!("Implement this!");
    let mut groups: HashMap<Value, Dataset> = HashMap::new();
    // created a hashmap to store groups
    let index = dataset.column_index(group_by_column);
    // I need the index as I cannot use the column name
    for row in dataset.iter() {
        // iterating all the rows in our datasert
        let data = row.get_value(index);
        // getting out the desired data in each row
        if groups.contains_key(data){
            // if the data is already a group in hashmap
            groups.get_mut(&data).unwrap().add_row(row.clone());
            // just add the row to it, I use unwrap as get_mut gives an option
        } else {
            // if it is not in the hashmap
            groups.insert(data.clone(), Dataset::new(dataset.columns().clone()));
            // first I create a new dataset to store the rows for that group
            groups.get_mut(&data).unwrap().add_row(row.clone());
            // and I add the row to that group's data set
        }
    }
    return groups
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