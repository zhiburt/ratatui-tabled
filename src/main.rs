mod table_widget;

use tabled::{
    Table,
    settings::{Alignment, Color, Padding, Span, Style, object::Columns, themes::ColumnNames},
};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
};

use table_widget::Table as TableWidget;

fn main() {
    let table = get_table();
    let table_widget = TableWidget::new(table);

    let terminal = ratatui::init();

    let app = App::new(table_widget);
    app.run(terminal).unwrap();

    ratatui::restore();
}

fn get_table() -> Table {
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
        .modify(Columns::first(), Color::BG_BLUE | Color::FG_GREEN)
        .modify((1, 0), Alignment::center())
        .modify((0, 0), Padding::new(4, 4, 1, 1))
        .modify((0, 0), Span::row(0))
        .modify((1, 1), Span::column(3));

    table
}

struct App {
    table: TableWidget,
}

impl App {
    fn new(table: TableWidget) -> Self {
        Self { table }
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

        frame.render_widget(self.table.clone(), rects[0]);
    }
}
