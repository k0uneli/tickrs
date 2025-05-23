use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::symbols::bar;
use ratatui::widgets::{BarChart, Block, Borders, StatefulWidget, Widget};

use crate::common::{Price, TimeFrame};
use crate::theme::style;
use crate::widget::StockState;
use crate::THEME;

pub struct VolumeBarChart<'a> {
    pub data: &'a [Price],
    pub loaded: bool,
    pub show_x_labels: bool,
}

impl StatefulWidget for VolumeBarChart<'_> {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut volume_chunks = area;
        volume_chunks.height += 1;

        let x_offset = if !self.loaded {
            8
        } else if self.show_x_labels {
            match state.time_frame {
                TimeFrame::Day1 => 9,
                TimeFrame::Week1 => 12,
                _ => 11,
            }
        } else {
            9
        };
        volume_chunks.x += x_offset;
        volume_chunks.width = volume_chunks.width.saturating_sub(x_offset + 1);

        let width = volume_chunks.width;
        let num_bars = width as usize;

        let volumes = state.volumes(self.data);
        let vol_count = volumes.len();

        if vol_count > 0 {
            let volumes = self
                .data
                .iter()
                .flat_map(|p| [p.volume].repeat(num_bars))
                .chunks(vol_count)
                .into_iter()
                .map(|c| ("", c.sum::<u64>() / vol_count as u64))
                .collect::<Vec<_>>();

            volume_chunks.x = volume_chunks.x.saturating_sub(1);

            Block::default()
                .borders(Borders::LEFT)
                .border_style(style().fg(THEME.border_axis()))
                .render(volume_chunks, buf);

            volume_chunks.x += 1;

            BarChart::default()
                .bar_gap(0)
                .bar_set(bar::NINE_LEVELS)
                .style(style().fg(THEME.gray()))
                .data(&volumes)
                .render(volume_chunks, buf);
        }
    }
}
