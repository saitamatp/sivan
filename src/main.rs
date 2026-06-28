mod prm_reader;
use prm_reader::find_value;
use prm_reader::read;
mod source_reader;
use source_reader::reader;
mod target_writer;
use target_writer::writer;
mod transformations;
use transformations::transform;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let values=read();
    let src_file=find_value(&values,"src_file".to_string());
    let tgt_file=find_value(&values,"tgt_file".to_string());
    let src_type:String=find_value(&values,"src_type".to_string());
    
    //read the source file (Extract)
    let df = reader(src_file, src_type).await.expect("Unable to create Source dataframe"); 

    //Manupulate (Transform)
    let df=transform(df).await.expect("Issue With transform trasformation");
    
    //write the target file (Load)
    writer(df,tgt_file).await.expect("Unable to write the target file");

    Ok(())
}
