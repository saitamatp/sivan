use datafusion::prelude::*;
use datafusion::error::Result;
use sivan::transformation_types;



pub async fn transform(df:DataFrame, transformations: Vec<transformation_types>)-> Result<DataFrame>{

    //println!("{}", transformations[0].Transformation);
    let mut dfa: DataFrame=df; 
    
   for transformation in transformations{

        if transformation.Transformation == "Filter" {
            dfa=filter_transformation(dfa.clone(),transformation.Column_name,transformation.Expression,transformation.Type).await.expect("Unable to filter the dataframe");
        } else {
            panic!("Transformation {} not supported", transformation.Transformation);
        }
    }
    Ok(dfa)
}

async fn filter_transformation(df:DataFrame,clm:String,exp:String,typ:String)-> Result<DataFrame>{
    let mut fil;
    if typ=="Less_equal"{
        fil=df.filter(col(clm).lt_eq(lit(exp))).expect("Issue With Less_equal trasformation");
    } else if typ=="Greater_equal"{
        fil=df.filter(col(clm).gt_eq(lit(exp))).expect("Issue With Greater_equal trasformation");
    } else if typ=="Equal"{
        fil=df.filter(col(clm).eq(lit(exp))).expect("Issue With Equal trasformation");
    } else if typ=="Less_thn" {
       fil=df.filter(col(clm).lt(lit(exp))).expect("Issue With Less_thn trasformation"); 
    } else if typ=="Greater_thn" {
        fil=df.filter(col(clm).gt(lit(exp))).expect("Issue With Greater_thn trasformation");
    } else if typ=="Not_equal" {
        fil=df.filter(col(clm).not_eq(lit(exp))).expect("Issue With Not_equal trasformation");
    } else {
        panic!("Filter transformation error");
    }
    Ok(fil)
}