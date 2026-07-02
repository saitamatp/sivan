# Sivan

## Summary
ETL tool completly written using Rust (Datafusion), all the deatils are read from paramter file and the tool reads, transforms and loads to the target.

## This tools aims to 
- **ABC:AnyBody can Code**- Working on creating a front end that will create a paramter file which can be read by this tool. This help people with minimal to no coding experience to work with data. 
- **High performance**- Since its written on rust can rust faster.

## Paramter file example
```
src_type=csv
src_file=D:\rust\ledger_25_26.csv
tgt_typ=parquet
tgt_file=D:\rust\rustledger_25_26_filtered.parquet
transformation=[{"Transformation": "Filter","Type": "Less_equal","Name": "initial filter","Column_name": "S No","Expression": "10"}]
```