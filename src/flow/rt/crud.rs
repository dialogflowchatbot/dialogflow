use redb::{ReadableTable, TableDefinition};

use crate::db;
use crate::result::Result;

fn get_table_name(main_flow_id: &str) -> String {
    format!("RTN{}", main_flow_id)
}

pub(crate) fn get_runtime_node(
    main_flow_id: &str,
    key: &str,
) -> Result<Option<crate::flow::rt::node::RuntimeNnodeEnum>> {
    let table_name = get_table_name(main_flow_id);
    let table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    let read_txn = db::DB.begin_read()?;
    let table = read_txn.open_table(table)?;
    let record = table.get(key)?;
    if let Some(r) = record {
        // let json = serde_json::from_str(r.value())?;
        let n = crate::flow::rt::node::deser_node(r.value())?;
        return Ok(Some(n));
    }
    Ok(None)
}

pub(crate) fn save_runtime_nodes(
    main_flow_id: &str,
    nodes: Vec<(String, rkyv::util::AlignedVec)>,
) -> Result<()> {
    let table_name = get_table_name(main_flow_id);
    let table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    let write_txn = db::DB.begin_write()?;
    // println!("save_runtime_nodes {}", main_flow_id);
    {
        let mut table = write_txn.open_table(table)?;
        for j in nodes.iter() {
            table.insert(j.0.as_str(), j.1.as_slice())?;
        }
    }
    write_txn.commit()?;
    Ok(())
}

pub(crate) fn remove_runtime_nodes(main_flow_id: &str) -> Result<()> {
    let table_name = get_table_name(main_flow_id);
    let table: TableDefinition<&str, &[u8]> = TableDefinition::new(&table_name);
    let write_txn = db::DB.begin_write()?;
    let _ = write_txn.delete_table(table)?;
    write_txn.commit()?;
    Ok(())
}
