use std::collections::HashMap;

use tabled::{
    Table,
    grid::{
        Typewriter, WritableGrid,
        ansi::{ANSIBuf, ANSIFmt},
        config::{Position, SpannedConfig},
        dimension::{Estimate, PeekableGridDimension},
        records::vec_records::{Text, VecRecords},
    },
    settings::{Alignment, Color, Padding, Span, Style, themes::ColumnNames},
};

fn main() {
    let grid = get_grid();

    let terminal = ratatui::init();

    let app = App::new(grid);
    app.run(terminal).unwrap();

    ratatui::restore();
}

type Grid = WritableGrid<
    VecRecords<Text<String>>,
    SpannedConfig,
    PeekableGridDimension,
    HashMap<Position, ANSIBuf>,
>;

fn get_grid() -> Grid {
    let grid = {
        let data = [
            [
                "https://github.com/ratatui/ratatui",
                "maybe",
                "a new backend",
                "?",
            ],
            ["", "I hope", "", ""],
        ];

        let mut table = Table::nohead(data);
        table
            .with(Style::ascii())
            .with(Alignment::center_vertical())
            .with(ColumnNames::new(["tabled"]).alignment(Alignment::center()))
            .modify((1, 0), Alignment::center())
            .modify((0, 0), Padding::new(4, 4, 1, 1))
            .modify((0, 0), Span::row(0))
            .modify((1, 1), Span::column(3));

        let records = table.get_records().clone();
        let cfg = table.get_config().clone().into_inner();

        let mut colors = HashMap::new();
        colors.insert((0, 0).into(), Color::BG_BLUE.into());
        colors.insert((1, 1).into(), Color::FG_RED.into());

        let mut dims = PeekableGridDimension::default();
        dims.estimate(&records, &cfg);

        WritableGrid::new(records, cfg, dims, colors)
    };
    grid
}

struct RatatuiWriter<'a, 'b> {
    pos: Position,
    rect: Rect,
    frame: &'a mut Frame<'b>,
}

impl Typewriter for RatatuiWriter<'_, '_> {
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
        let mut area = self.rect;
        area.x += self.pos.col as u16;
        area.y += self.pos.row as u16;
        self.frame.render_widget(t, area);

        self.pos += (0, width);

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let t = RatatuiText::raw(c.to_string());
        let mut area = self.rect;
        area.x += self.pos.col as u16;
        area.y += self.pos.row as u16;
        self.frame.render_widget(t, area);

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

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Rect},
    text::Text as RatatuiText,
    widgets::TableState,
};

struct App {
    _state: TableState,
    grid: Grid,
}

impl App {
    fn new(grid: Grid) -> Self {
        Self {
            _state: TableState::default(),
            grid,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(_key) = event::read()? {
                return Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let w = RatatuiWriter {
            frame: frame,
            pos: Position::default(),
            rect: area,
        };

        self.grid.build(w).unwrap();
    }
}
