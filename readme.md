# Sivan

## Summary
ETL tool fully developed in Rust using DataFusion, driven by a parameter file to read, transform, and load data into the target system.

## This tools aims to 
- **ABC:AnyBody can Code**- Developing a front‑end interface that generates the parameter file consumed by the ETL tool, enabling users with minimal or no coding experience to work seamlessly with data.
- **High performance**- Written in Rust, the tool delivers high performance and efficient memory management, ensuring fast execution without stability issues.

## Paramter file example
```
src_type=csv
src_file=D:\rust\ledger_25_26.csv
tgt_typ=parquet
tgt_file=D:\rust\rustledger_25_26_filtered.parquet
transformation=[{"Transformation": "Filter","Type": "Less_equal","Name": "initial filter","Column_name": "S No","Expression": "10"}]
```