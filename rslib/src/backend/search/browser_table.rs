// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::str::FromStr;

use crate::{backend_proto as pb, browser_table, collection::Collection, i18n::I18n};

const CARD_COLUMNS: [browser_table::Column; 15] = [
    browser_table::Column::Question,
    browser_table::Column::Answer,
    browser_table::Column::CardDeck,
    browser_table::Column::CardDue,
    browser_table::Column::CardEase,
    browser_table::Column::CardLapses,
    browser_table::Column::CardInterval,
    browser_table::Column::CardMod,
    browser_table::Column::CardReps,
    browser_table::Column::CardTemplate,
    browser_table::Column::NoteCreation,
    browser_table::Column::NoteField,
    browser_table::Column::NoteMod,
    browser_table::Column::NoteTags,
    browser_table::Column::Notetype,
];

const NOTE_COLUMNS: [browser_table::Column; 11] = [
    browser_table::Column::NoteCards,
    browser_table::Column::NoteCreation,
    browser_table::Column::NoteDue,
    browser_table::Column::NoteEase,
    browser_table::Column::NoteField,
    browser_table::Column::NoteInterval,
    browser_table::Column::NoteLapses,
    browser_table::Column::NoteMod,
    browser_table::Column::NoteReps,
    browser_table::Column::NoteTags,
    browser_table::Column::Notetype,
];

impl Collection {
    pub(crate) fn all_browser_card_columns(&self) -> pb::BrowserColumns {
        self.to_pb_columns(&CARD_COLUMNS)
    }

    pub(crate) fn all_browser_note_columns(&self) -> pb::BrowserColumns {
        self.to_pb_columns(&NOTE_COLUMNS)
    }

    fn to_pb_columns(&self, columns: &[browser_table::Column]) -> pb::BrowserColumns {
        let mut columns: Vec<pb::browser_columns::Column> =
            columns.iter().map(|c| c.to_pb_column(&self.tr)).collect();
        columns.sort_by(|c1, c2| c1.label.cmp(&c2.label));
        pb::BrowserColumns { columns }
    }
}

impl browser_table::Column {
    fn to_pb_column(self, i18n: &I18n) -> pb::browser_columns::Column {
        pb::browser_columns::Column {
            key: self.to_string(),
            label: self.localized_label(i18n),
            is_sortable: self.is_sortable(),
            sorts_reversed: self == browser_table::Column::NoteField,
            uses_cell_font: self.uses_cell_font(),
            alignment: self.alignment() as i32,
        }
    }

    fn is_sortable(self) -> bool {
        !matches!(self, Self::Question | Self::Answer | Self::Custom)
    }

    fn uses_cell_font(self) -> bool {
        matches!(self, Self::Question | Self::Answer | Self::NoteField)
    }

    fn alignment(self) -> pb::browser_columns::Alignment {
        match self {
            Self::Question
            | Self::Answer
            | Self::CardTemplate
            | Self::CardDeck
            | Self::NoteField
            | Self::Notetype
            | Self::NoteTags => pb::browser_columns::Alignment::Start,
            _ => pb::browser_columns::Alignment::Center,
        }
    }

    fn localized_label(self, i18n: &I18n) -> String {
        match self {
            Self::Custom => i18n.browsing_addon(),
            Self::Question => i18n.browsing_question(),
            Self::Answer => i18n.browsing_answer(),
            Self::CardDeck => i18n.decks_deck(),
            Self::CardDue => i18n.statistics_due_date(),
            Self::CardEase => i18n.browsing_ease(),
            Self::CardInterval => i18n.browsing_interval(),
            Self::CardLapses => i18n.scheduling_lapses(),
            Self::CardMod => i18n.search_card_modified(),
            Self::CardReps => i18n.scheduling_reviews(),
            Self::CardTemplate => i18n.browsing_card(),
            Self::NoteCards => i18n.editing_cards(),
            Self::NoteCreation => i18n.browsing_created(),
            Self::NoteDue => i18n.statistics_due_date(),
            Self::NoteEase => i18n.browsing_average_ease(),
            Self::NoteField => i18n.browsing_sort_field(),
            Self::NoteInterval => i18n.browsing_average_interval(),
            Self::NoteMod => i18n.search_note_modified(),
            Self::NoteLapses => i18n.scheduling_lapses(),
            Self::NoteReps => i18n.scheduling_reviews(),
            Self::NoteTags => i18n.editing_tags(),
            Self::Notetype => i18n.browsing_note(),
        }
        .into()
    }
}

impl From<pb::StringList> for Vec<browser_table::Column> {
    fn from(input: pb::StringList) -> Self {
        input
            .vals
            .iter()
            .map(|c| browser_table::Column::from_str(c).unwrap_or_default())
            .collect()
    }
}

impl From<browser_table::Row> for pb::BrowserRow {
    fn from(row: browser_table::Row) -> Self {
        pb::BrowserRow {
            cells: row.cells.into_iter().map(Into::into).collect(),
            color: row.color.into(),
            font_name: row.font.name,
            font_size: row.font.size,
        }
    }
}

impl From<browser_table::Cell> for pb::browser_row::Cell {
    fn from(cell: browser_table::Cell) -> Self {
        pb::browser_row::Cell {
            text: cell.text,
            is_rtl: cell.is_rtl,
        }
    }
}

impl From<browser_table::Color> for i32 {
    fn from(color: browser_table::Color) -> Self {
        match color {
            browser_table::Color::Default => pb::browser_row::Color::Default as i32,
            browser_table::Color::Marked => pb::browser_row::Color::Marked as i32,
            browser_table::Color::Suspended => pb::browser_row::Color::Suspended as i32,
            browser_table::Color::FlagRed => pb::browser_row::Color::FlagRed as i32,
            browser_table::Color::FlagOrange => pb::browser_row::Color::FlagOrange as i32,
            browser_table::Color::FlagGreen => pb::browser_row::Color::FlagGreen as i32,
            browser_table::Color::FlagBlue => pb::browser_row::Color::FlagBlue as i32,
        }
    }
}
