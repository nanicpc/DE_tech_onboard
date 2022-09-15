use datafusion::prelude::*;


// #[tokio::main]
// async fn main() -> datafusion::error::Result<()> {
//   // create the dataframe
//   let ctx = SessionContext::new();
//   let df = ctx.read_csv("tests/example.csv", CsvReadOptions::new()).await?;

//   let df = df.filter(col("Latitude").gt_eq(col("Longitude")))?
//            .aggregate(vec![col("State")], vec![min(col("Longitude"))])?;

//   // execute and print results
//   df.show_limit(100).await?;
//   Ok(())
// }

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // register the table
  let ctx = SessionContext::new();
  ctx.register_csv("example", "tests/example.csv", CsvReadOptions::new()).await?;

  // create a plan to run a SQL query
  let df = ctx.sql("SELECT \"State\", MIN(\"Latitude\"), MIN(\"Longitude\") FROM example GROUP BY \"State\" LIMIT 100").await?;

  // execute and print results
  df.show().await?;
  Ok(())
}