use std::sync::Arc;
use std::vec::Vec;

use arrow_array::array::ArrayRef;
use arrow_array::types::Float32Type;
use arrow_array::{FixedSizeListArray, Int32Array, RecordBatch, RecordBatchIterator};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;

use lancedb::arrow::IntoArrow;
use lancedb::connection::Connection;
use lancedb::index::Index;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{connect, Error, Result, Table as LanceDbTable};

const DATA_ROOT_PATH: &str = "./data/intentev";

async fn add(robot_id: &str, v: Vec<i32>) -> Result<()> {
    let db = connect(DATA_ROOT_PATH).execute().await?;

    // let field_a = Field::new("id", DataType::UInt64, false);
    // let field_b = Field::new(
    //     "vector",
    //     DataType::FixedSizeList(
    //         Arc::new(Field::new("item", DataType::Float32, false)),
    //         v.len() as i32,
    //     ),
    //     false,
    // );
    let v: ArrayRef = Arc::new(Int32Array::from(v));

    let record_batch = RecordBatch::try_from_iter(vec![("a", v)]).unwrap();

    let batches: Vec<RecordBatch> = vec![record_batch.clone(), record_batch.clone()];

    let batches = RecordBatchIterator::new(batches.into_iter().map(Ok), record_batch.schema());

    let table = match db.open_table(robot_id).execute().await {
        Ok(t) => t,
        Err(Error::TableNotFound { name }) => db.create_table(name, batches).execute().await?,
        Err(err) => return Err(err),
    };
    Ok(())
}
