use core::time::Duration;

use std::fs::OpenOptions;
use std::sync::OnceLock;
use std::vec::Vec;

use futures_util::StreamExt;
use sqlx::{pool::PoolOptions, Row, Sqlite};

use crate::result::{Error, Result};

type SqliteConnPool = sqlx::Pool<Sqlite>;

// static DATA_SOURCE: OnceCell<SqliteConnPool> = OnceCell::new();
static DATA_SOURCE: OnceLock<SqliteConnPool> = OnceLock::new();
// static DATA_SOURCES: OnceLock<Mutex<HashMap<String, SqliteConnPool>>> = OnceLock::new();

fn get_sqlite_path() -> std::path::PathBuf {
    let p = std::path::Path::new(".").join("data");
    if !p.exists() {
        std::fs::create_dir_all(&p).expect("Create data directory failed.");
    }
    p.join("iev.dat")
}

pub(crate) async fn init_datasource() -> Result<()> {
    // unsafe {
    //     libsqlite3_sys::sqlite3_auto_extension(Some(std::mem::transmute(
    //         sqlite_vec::sqlite3_vec_init as *const (),
    //     )));
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
    let path = get_sqlite_path();
    if path.is_dir() {
        return Err(Error::ErrorWithMessage(String::from(
            "Created database file failed, there is a directory called: e.dat",
        )));
    }
    let s = format!("sqlite://{}?mode=rw", path.display());
    let conn_str = s.replace("\\", "/");
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
        "CREATE TABLE p{} USING vec0 (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            intent_id TEXT NOT NULL,
            phrase TEXT NOT NULL
        );
        CREATE VIRTUAL TABLE pev{} USING vec0 (
            -- id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            intent_id TEXT NOT NULL,
            intent_name TEXT NOT NULL,
            vectors float[384]
        );",
        robot_id, robot_id
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
    Ok(())
}

pub(crate) async fn search(robot_id: &str, vectors: &Vec<f32>) -> Result<Vec<(String, f64)>> {
    let sql = format!(
        "SELECT intent_id, intent_name, distance FROM {} WHERE vectors MATCH ? ORDER BY distance LIMIT 1",
        robot_id
    );
    let results = sqlx::query::<Sqlite>(&sql)
        .bind(serde_json::to_string(vectors)?)
        .fetch_all(DATA_SOURCE.get().unwrap())
        .await?;
    let mut names = Vec::with_capacity(results.len());
    for r in results.iter() {
        names.push((r.try_get(1)?, r.try_get(2)?));
    }
    Ok(names)
}

pub(crate) async fn add(
    robot_id: &str,
    intent_id: &str,
    intent_name: &str,
    vectors: &Vec<f32>,
) -> Result<i64> {
    // check_datasource(robot_id, intent_id).await?;
    let sql = format!(
        "INSERT INTO {} (rowid, intent_id, intent_name, vectors)VALUES(?, ?, ?, ?)",
        robot_id
    );
    let last_insert_rowid = sqlx::query::<Sqlite>(&sql)
        .bind(intent_id)
        .bind(intent_name)
        .bind(serde_json::to_string(vectors)?)
        .execute(DATA_SOURCE.get().unwrap())
        .await?
        .last_insert_rowid();
    Ok(last_insert_rowid)
}

pub(crate) async fn batch_add(
    robot_id: &str,
    intent_id: &str,
    intent_name: &str,
    vectors: &Vec<Vec<f32>>,
) -> Result<Vec<i64>> {
    // check_datasource(robot_id, intent_id).await?;
    let sql = format!(
        "INSERT INTO {} (intent_id, intent_name, vectors)VALUES(?, ?, ?)",
        robot_id
    );
    let mut ids: Vec<i64> = Vec::with_capacity(vectors.len());
    for v in vectors.iter() {
        let last_insert_rowid = sqlx::query::<Sqlite>(&sql)
            .bind(intent_id)
            .bind(intent_name)
            .bind(serde_json::to_string(v)?)
            .execute(DATA_SOURCE.get().unwrap())
            .await?
            .last_insert_rowid();
        ids.push(last_insert_rowid);
    }
    Ok(ids)
}

pub(crate) async fn remove(robot_id: &str, id: i64) -> Result<()> {
    let sql = format!("DELETE FROM {} WHERE id = ?", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .bind(id)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}

pub(crate) async fn remove_by_intent_id(robot_id: &str, intent_id: &str) -> Result<()> {
    let sql = format!("DELETE FROM {} WHERE intent_id = ?", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .bind(intent_id)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}

pub(crate) async fn remove_table(robot_id: &str) -> Result<()> {
    let sql = format!("DROP TABLE {}", robot_id);
    sqlx::query::<Sqlite>(&sql)
        .execute(DATA_SOURCE.get().unwrap())
        .await?;
    Ok(())
}
