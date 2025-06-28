use tabled::{
    Table as InnerTable,
    grid::{
        Typewriter, WritableGrid,
        ansi::ANSIFmt,
        config::{ColorMap, Position, SpannedConfig},
        dimension::{Estimate, PeekableGridDimension},
        records::vec_records::{Text, VecRecords},
    },
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text as RatatuiText,
    widgets::{StatefulWidget, TableState, Widget},
};

#[derive(Debug, Clone)]
pub struct Table {
    table: InnerTable,
}

impl Table {
    pub fn new(table: InnerTable) -> Self {
        Table { table }
    }
}

impl Widget for Table {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let records = self.table.get_records().clone();
        let cfg = self.table.get_config().clone().into_inner();
        let colors = self.table.get_config().get_colors().clone();

        let mut dims = PeekableGridDimension::default();
        dims.estimate(&records, &cfg);

        let grid = Grid::new(records, cfg, dims, colors);

        let writer = RatatuiWriter::new(area, buf);

        grid.build(writer).unwrap();
    }
}

impl StatefulWidget for Table {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        Widget::render(self, area, buf);
    }
}

type Grid = WritableGrid<VecRecords<Text<String>>, SpannedConfig, PeekableGridDimension, ColorMap>;

struct RatatuiWriter<'a> {
    pos: Position,
    area: Rect,
    buf: &'a mut Buffer,
}

impl<'a> RatatuiWriter<'a> {
    fn new(area: Rect, buf: &'a mut Buffer) -> Self {
        Self {
            area,
            buf,
            pos: Position::default(),
        }
    }
}

impl Typewriter for RatatuiWriter<'_> {
    fn start(&mut self) -> std::fmt::Result {
        Ok(())
    }

    fn finish(&mut self) -> std::fmt::Result {
        Ok(())
    }

    fn reset(&mut self) -> std::fmt::Result {
        self.pos.row += 1;
        self.pos.col = 0;
        Ok(())
    }

    fn write_str(&mut self, text: &str, width: usize) -> std::fmt::Result {
        let t = RatatuiText::raw(text);
        let mut area = self.area;
        area.x += self.pos.col as u16;
        area.y += self.pos.row as u16;

        t.render(area, self.buf);

        self.pos += (0, width);

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let t = RatatuiText::raw(c.to_string());
        let mut area = self.area;
        area.x += self.pos.col as u16;
        area.y += self.pos.row as u16;

        t.render(area, self.buf);

        self.pos += (0, 1);

        Ok(())
    }

    fn colorize_start<C: ANSIFmt>(&mut self, _: C) -> std::fmt::Result {
        Ok(())
    }

    fn colorize_stop<C: ANSIFmt>(&mut self, _: C) -> std::fmt::Result {
        Ok(())
    }
}
