use crate::data::{RawData, RenderableData};
use crate::formatting::{format_date_with_offset, generate_axis};
use crate::process::generate_renderable_data;
use Constraint::{Fill, Length, Min};
use crossterm::event::{self, KeyEventKind};
use ratatui;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Cell, List, ListState, Row, Sparkline, Table};

struct App {
    exit: bool,
    raw_data: RawData,
    list_state: ListState,
    numbins: usize,
    range: (u32, u32),
    renderable_data: Option<RenderableData>,
}

impl App {
    fn init_with(data: RawData, range: (u32, u32)) -> Self {
        let mut list_state = ListState::default();
        *list_state.offset_mut() = 0;
        list_state.select(Some(0));

        App {
            exit: false,
            raw_data: data,
            list_state,
            numbins: 0,
            range,
            renderable_data: None,
        }
    }

    fn draw_list(&mut self, frame: &mut Frame, area: Rect) {
        let author_items = self.raw_data.authors.iter().map(|c| Text::raw(&c.name));
        let list = List::new(author_items)
            .block(Block::bordered().title("Authors"))
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn draw_details(&mut self, frame: &mut Frame, area: Rect) {
        let mut data: Vec<(String, String)> = vec![];

        if let Some(renderable_data) = &self.renderable_data
            && let Some(selected_index) = &self.list_state.selected()
        {
            let author = &renderable_data.authors[*selected_index];

            data.push((String::from("Author name"), author.name.clone()));

            data.push((
                String::from("Total commits"),
                author.total_commits.to_string(),
            ));

            let start_date = format_date_with_offset(author.first_commit, None);
            data.push((String::from("First commit"), start_date));

            let end_date = format_date_with_offset(author.last_commit, None);
            data.push((String::from("Last commit"), end_date));
        };

        let max_width_first_col = data
            .iter()
            .fold(0, |a, v| if v.0.len() > a { v.0.len() } else { a });

        let rows: Vec<Row> = data
            .iter()
            .map(|(a, b)| Row::new([Cell::from(a.clone()), Cell::from(b.clone())]))
            .collect();

        let widths = [
            Constraint::Length(max_width_first_col as u16),
            Constraint::Fill(1),
        ];

        frame.render_widget(
            Table::new(rows, widths)
                .column_spacing(2)
                .block(Block::bordered().title("Details")),
            area,
        );
    }

    fn draw_histogram(&mut self, frame: &mut Frame, plot_area: Rect) {
        if let Some(renderable_data) = &self.renderable_data
            && let Some(selected_index) = self.list_state.selected()
        {
            let selected_author = renderable_data.authors[selected_index].name.to_string();
            if let Some(author) = renderable_data
                .authors
                .iter()
                .find(|a| a.name == selected_author)
            {
                let data = author.bins.iter().map(|c| *c as u64);
                let histogram = Sparkline::default()
                    .data(data)
                    .max(renderable_data.max_count as u64);

                frame.render_widget(histogram, plot_area);
            }
        }
    }

    fn draw_axis(&mut self, frame: &mut Frame, axis_area: Rect) {
        let axis = Text::raw(generate_axis(axis_area.width as u64, self.range, None));
        frame.render_widget(axis, axis_area);
    }

    fn refresh_processed_data(&mut self) {
        self.renderable_data = Some(generate_renderable_data(
            &self.raw_data,
            self.range,
            self.numbins,
        ));
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([Length(1), Min(6), Fill(2), Length(1)]);
        let [header_area, top_area, bottom_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Min(12), Fill(2)]);
        let [left_area, right_area] = horizontal.areas(top_area);
        let vertical_bottom = Layout::vertical([Fill(1), Length(1)]).margin(1);
        let [plot_area, axis_area] = vertical_bottom.areas(bottom_area);

        // Header line
        let header_line =
            Block::new().title(format!("git-contributors ({})", self.raw_data.repo_name));
        frame.render_widget(header_line, header_area);

        // Top sections
        self.draw_list(frame, left_area);

        // Bottom section
        let histogram_width = plot_area.width as usize;
        if histogram_width != self.numbins {
            // recompute all histograms
            self.numbins = histogram_width;
            self.refresh_processed_data();
        }
        frame.render_widget(Block::bordered().title("Commits"), bottom_area);
        self.draw_histogram(frame, plot_area);
        self.draw_axis(frame, axis_area);

        // Details
        self.draw_details(frame, right_area);

        // Status line
        frame.render_widget(Block::new().title("Press 'q' to quit"), status_area);
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        ratatui::run(|terminal| {
            while !self.exit {
                terminal.draw(|frame| self.draw(frame)).unwrap();
                self.handle_events();
            }
        });

        Ok(())
    }

    fn handle_events(&mut self) {
        let e = event::read().unwrap();
        match e {
            // Key press
            event::Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                // Quit
                event::KeyCode::Char('q') => {
                    self.exit = true;
                }
                event::KeyCode::Down | event::KeyCode::Char('j') => {
                    self.list_state.select_next();
                }
                event::KeyCode::Up | event::KeyCode::Char('k') => {
                    self.list_state.select_previous();
                }
                _ => {}
            },
            // Everything else
            _ => {}
        }
    }
}

pub fn run_interactive(data: RawData, range: (u32, u32)) -> std::io::Result<()> {
    App::init_with(data, range).run()
}
