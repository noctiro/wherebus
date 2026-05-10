use redb::TableDefinition;

pub const CONFIG_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("config");
pub const STATIONS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("stations");
pub const STATION_LINES_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("station_lines");
pub const LINE_DETAIL_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("line_detail");
pub const ALL_LINES_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("all_lines");
pub const TTL_TABLE: TableDefinition<&str, u64> = TableDefinition::new("ttl");
