use datafusion::prelude::*;
use datafusion::error::Result;

pub async fn reader(a:String) -> Result<DataFrame> {
    
    let ctx = SessionContext::new();
    ctx.register_csv("Source", a, CsvReadOptions::new()).await.expect("Unable to read the source file");
    let df = ctx.sql("SELECT * FROM Source").await.expect("Unable to create Source dataframe"); 

    Ok(df)
}