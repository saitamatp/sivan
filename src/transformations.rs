use datafusion::prelude::*;
use datafusion::error::Result;



pub async fn transform(df:DataFrame)-> Result<DataFrame>{
    
    let dfa=df.filter(col("S No").lt_eq(lit(100))).expect("Issue With filter trasformation");

    Ok(dfa)
}