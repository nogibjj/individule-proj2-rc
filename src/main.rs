use proj2::{convert_csv_to_sql, query_crud};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    match command.as_str() {
        "dataload" => match convert_csv_to_sql("data.csv") {
            Ok(_) => println!("Data is successfully loaded!"),
            Err(err) => eprintln!("Error: {:?}", err),
        },
        
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query_crud(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        },

        // "dataload" => match convert_csv_to_sql("data.csv") {
        //     Ok(_) => println!("Data is successfully loaded!"),
        //     Err(err) => eprintln!("Error: {:?}", err),
        // }
        // "query" => {
        //     if let Some(q) = args.get(2) {
        //         if let Err(err) = query_crud(q) {
        //             eprintln!("Error: {:?}", err);
        //         } else {
        //             println!("Query executed successfully!");
        //         }
        //     } else {
        //         println!("Usage: {} query [SQL query]", args[0]);
        //     }
        // }
        _ => {
            println!("Invalid action.");
        }

    }
}
