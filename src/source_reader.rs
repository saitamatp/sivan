use datafusion::prelude::*;
use datafusion::error::Result;

pub async fn reader(a:String,b:String) -> Result<DataFrame> {
    
    let ctx = SessionContext::new();
    
    if b=="csv"{
        ctx.register_csv("Source", a, CsvReadOptions::new()).await.expect("Unable to read CSV source file");
    }
    else if b=="parquet"{
        ctx.register_parquet("Source", a, ParquetReadOptions::default()).await.expect("Unable to read parquet source file");
    } 
    else if b=="json"{
        ctx.register_json("Source", a, JsonReadOptions::default()).await.expect("Unable to read json source file");
    }
    else{
        panic!("Invalid Source Type");
    };
    let df = ctx.sql("SELECT * FROM Source").await.expect("Unable to create Source dataframe"); 

    Ok(df)
}