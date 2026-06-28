use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::prelude::DataFrame;


pub async fn writer(df:DataFrame, tgt_file: String)-> datafusion::error::Result<()> {
    df.write_parquet(
    &tgt_file,
    DataFrameWriteOptions::new(),
    None, // Optional: Pass Some(TableParquetOptions) here for tuning
    )
    .await.expect("Unable to write the target file");
    Ok(())
}