use core::time::Duration;

use std::fs::OpenOptions;
use std::path::Path;
use std::sync::OnceLock;
use std::vec::Vec;

use futures_util::StreamExt;
use oasysdb::prelude::*;
use sqlx::{pool::PoolOptions, Row, Sqlite};

use crate::result::{Error, Result};

type SqliteConnPool = sqlx::Pool<Sqlite>;

#[macro_export]
macro_rules! sql_query_one (
    ($sql: expr, $($bind: expr),*) => ({
        let pool = match db::mysql::get_pool() {
            Some(p) => p,
            None => return Err("mysql get pool failed".into()),
        };

        match sqlx::query_as(&$sql)$(.bind($bind))*.fetch_one(pool).await {
            Ok(u) => Ok(Some(u)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into())
            },
        }
    });
    ($sql: expr) => (query_one!($sql,));
);

// static DATA_SOURCE: OnceCell<SqliteConnPool> = OnceCell::new();
static DATA_SOURCE: OnceLock<SqliteConnPool> = OnceLock::new();
// static DATA_SOURCES: OnceLock<Mutex<HashMap<String, SqliteConnPool>>> = OnceLock::new();

fn get_idx_db() -> Result<Database> {
    let mut p = get_sqlite_path();
    p.pop();
    // let dir = std::env::temp_dir();
    let db = Database::open(p, Some(get_sqlite_url()?))?;
    Ok(db)
}

async fn create_idx_db(robot_id: &str) -> Result<()> {
    let config = SourceConfig::new(robot_id, "id", "vectors").with_metadata(vec!["intent_id"]);
    // let params = ParamsIVFPQ::default();
    // let algorithm = IndexAlgorithm::IVFPQ(params);
    let mut params = ParamsFlat::default();
    params.metric = oasysdb::types::distance::DistanceMetric::Cosine;
    let algorithm = IndexAlgorithm::Flat(params);
    get_idx_db()?
        .async_create_index(robot_id, algorithm, config)
        .await?;
    Ok(())
}

pub(crate) fn search_idx_db(robot_id: &str, search_vector: Vector) -> Result<Vec<SearchResult>> {
    let r = get_idx_db()?.search_index(robot_id, search_vector, 1, "")?;
    Ok(r)
}

pub(crate) fn get_sqlite_path() -> std::path::PathBuf {
    let p = Path::new(".").join("data").join("intentev");
    if !p.exists() {
        std::fs::create_dir_all(&p).expect("Create data directory failed.");
    }
    p.join("e.dat")
}

pub(crate) fn get_sqlite_url() -> Result<String> {
    let path = get_sqlite_path();
    if path.is_dir() {
        return Err(Error::ErrorWithMessage(String::from(
            "Created database file failed, there is a directory called: e.dat",
        )));
    }
    let s = format!("sqlite://{}?mode=rw", path.display());
    let s = s.replace("\\", "/");
    Ok(s)
}

pub(crate) async fn init_datasource() -> Result<()> {
    // if path.exists() {
    //     return Ok(());
    // }
    match OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(get_sqlite_path().as_path())
    {
        Ok(_f) => {}
        // Err(e: ErrorKind::NotFound) => None,
        Err(e) => {
            return Err(Error::ErrorWithMessage(format!(
                "Created database file failed, err: {:?}",
                &e
            )))
        }
    };
    let pool_ops = PoolOptions::<Sqlite>::new()
        .min_connections(1)
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(5))
        .test_before_acquire(true);
    let conn_str = get_sqlite_url()?;
    // log::info!("Embedding database path: {}", &conn_str);
    let pool = pool_ops.connect(conn_str.as_str()).await?;
    DATA_SOURCE
        .set(pool)
        .map_err(|_| Error::ErrorWithMessage(String::from("Datasource has been set.")))
    /*
    下面这个不会打印，解决：
    1、把map换成for_each
    2、由于map是lazy的，所以需要在map后面加.collect()
     */
    /*
    match sqlite_get_list::<Tag>("SELECT id, name FROM blog_tag ORDER BY id DESC", None).await {
        Ok(tags) => tags.iter().map(|tag| {
            println!("{}", &tag.name);
            tag::put_id_name(tag.id, &tag.name);
        }),
        Err(e) => panic!("{:?}", e),
    };
    */
}

pub async fn shutdown() {
    // let mut r = match DATA_SOURCES.lock() {
    //     Ok(l) => l,
    //     Err(e) => e.into_inner(),
    // };
    // let all_keys: Vec<String> = r.keys().map(|k| String::from(k)).collect();
    // let mut pools: Vec<SqliteConnPool> = Vec::with_capacity(all_keys.len());
    // for key in all_keys {
    //     let v = r.remove(&key).unwrap();
    //     pools.push(v);
    // }
    // tokio::task::spawn_blocking(|| async move {
    //     for p in pools.iter() {
    //         p.close().await;
    //     }
    // });
    DATA_SOURCE.get().unwrap().close().await;
}

pub(crate) async fn create_table(robot_id: &str) -> Result<()> {
    // println!("Init database");
    // let ddl = include_str!("./embedding_ddl.sql");
    let sql = format!(
        "CREATE TABLE {} (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            intent_id TEXT NOT NULL,
            vectors JSON NOT NULL
            );",
        robot_id
    );
    // log::info!("sql = {}", &sql);
    let mut stream = sqlx::raw_sql(&sql).execute_many(DATA_SOURCE.get().unwrap());
    while let Some(res) = stream.next().await {
        match res {
            Ok(_r) => log::info!("Initialized intent table"),
            Err(e) => log::error!("Create table failed, err: {:?}", e),
        }
    }
    // let dml = include_str!("../resource/sql/dml.sql");
    // if let Err(e) = sqlx::query(dml).execute(&pool).await {
    //     panic!("{:?}", e);
    // }
    create_idx_db(robot_id).await
}

pub(crate) async fn add(robot_id: &str, intent_id: &str, vector: &Vec<f32>) -> Result<i64> {
    // check_datasource(robot_id, intent_id).await?;
    let sql = format!("INSERT INTO {} (intent_id,vectors)VALUES(?,?)", robot_id);
    let last_insert_rowid = sqlx::query::<Sqlite>(&sql)
        .bind(intent_id)
        .bind(serde_json::to_string(vector)?)
        .execute(DATA_SOURCE.get().unwrap())
        .await?
        .last_insert_rowid();
    get_idx_db()?.async_refresh_index(robot_id).await?;
    Ok(last_insert_rowid)
    // Ok(0i64)
}

pub(crate) async fn batch_add(
    robot_id: &str,
    intent_id: &str,
    vectors: &Vec<Vec<f32>>,
) -> Result<Vec<i64>> {
    // check_datasource(robot_id, intent_id).await?;
    let sql = format!("INSERT INTO {} (intent_id,vectors)VALUES(?,?)", robot_id);
    let mut ids: Vec<i64> = Vec::with_capacity(vectors.len());
    for v in vectors.iter() {
        let last_insert_rowid = sqlx::query::<Sqlite>(&sql)
            .bind(intent_id)
            .bind(serde_json::to_string(v)?)
            .execute(DATA_SOURCE.get().unwrap())
            .await?
            .last_insert_rowid();
        ids.push(last_insert_rowid);
    }
    get_idx_db()?.async_refresh_index(robot_id).await?;
    Ok(ids)
    // Ok(0i64)
}

pub(crate) async fn remove(robot_id: &str, id: i64) -> Result<()> {
    get_idx_db()?.delete_from_index(robot_id, vec![RecordID(id as u32)])?;
    let sql = format!("DELETE FROM {} WHERE id=?", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .bind(id)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}

pub(crate) async fn remove_by_intent_id(robot_id: &str, intent_id: &str) -> Result<()> {
    let sql = format!("SELECT id FROM {} WHERE intent_id=?", robot_id);
    let results = sqlx::query::<Sqlite>(&sql)
        .bind(intent_id)
        .fetch_all(DATA_SOURCE.get().unwrap())
        .await?;
    let mut ids: Vec<RecordID> = Vec::with_capacity(results.len());
    for r in results.iter() {
        ids.push(RecordID(r.try_get(0)?));
    }
    get_idx_db()?.delete_from_index(robot_id, ids)?;

    let sql = format!("DELETE FROM {} WHERE intent_id=?", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .bind(intent_id)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}

pub(crate) async fn remove_table(robot_id: &str) -> Result<()> {
    get_idx_db()?.delete_index(robot_id)?;
    let sql = format!("DROP TABLE {}", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}
