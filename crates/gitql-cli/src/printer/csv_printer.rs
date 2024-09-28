use csv::Writer;
use gitql_core::object::GitQLObject;

use super::base::OutputPrinter;

pub struct CSVPrinter;

impl OutputPrinter for CSVPrinter {
    fn print(&self, object: &mut GitQLObject) {
        let mut writer = Writer::from_writer(vec![]);
        let _ = writer.write_record(object.titles.clone());
        let row_len = object.titles.len();
        if let Some(group) = object.groups.first() {
            for row in &group.rows {
                let mut values_row: Vec<String> = Vec::with_capacity(row_len);
                for value in &row.values {
                    values_row.push(value.to_string());
                }
                let _ = writer.write_record(values_row);
            }
        }

        if let Ok(writer_content) = writer.into_inner() {
            println!("{:?}", String::from_utf8(writer_content));
        }
    }
}
