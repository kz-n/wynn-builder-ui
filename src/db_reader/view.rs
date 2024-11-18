use iced::Element;
use iced_table::table;
use iced_widget::{column, scrollable, text, Column, Container};

use crate::{DBReaderMessage, Message};

use super::DBReader;

impl DBReader {
    pub fn view(&self) -> Container<Message> {
        todo!();
        let header = scrollable::Id::new("db_table");
        let body = scrollable::Id::new("db_table_body");
        let columns: Vec<Column<Message>> = self.columns
            .iter()
            .map(|col| column(vec![text(col).into()]))
            .collect();
        // let on_sync = |_| Message::DBReader(DBReaderMessage::Sync);

        Container::new(
            // Element::new(table(header, body, columns.as_slice(), self.rows.as_slice(), on_sync))
            text("DB Reader")
        )
    }
}