use serde::Deserialize;
use structopt::clap::{_clap_count_exprs, arg_enum};
use strum_macros::EnumIter;

arg_enum! {
    #[derive(PartialEq, Deserialize, Clone, EnumIter)]
    #[serde(rename_all = "lowercase")]
    pub enum ColorScheme {
        Archlinux,
        Zenburn,
        Monokai,
        Squirrel,
    }
}

impl ColorScheme {
    /// Returns the URL-compatible name of a color scheme
    /// This must correspond to the name of the variant, in lowercase
    /// See https://github.com/svenstaro/miniserve/pull/55 for explanations
    pub fn to_slug(&self) -> String {
        match &self {
            ColorScheme::Archlinux => "archlinux",
            ColorScheme::Zenburn => "zenburn",
            ColorScheme::Monokai => "monokai",
            ColorScheme::Squirrel => "squirrel",
        }
        .to_string()
    }

    /// Returns wether a color scheme is dark
    pub fn is_dark(&self) -> bool {
        match &self {
            ColorScheme::Archlinux => true,
            ColorScheme::Zenburn => true,
            ColorScheme::Monokai => true,
            ColorScheme::Squirrel => false,
        }
    }

    /// Retrieves the color palette associated to a color scheme
    pub fn get_theme(self) -> Theme {
        match self {
            ColorScheme::Archlinux => Theme {
                background: "#383c4a".to_string(),
                text_color: "#fefefe".to_string(),
                directory_link_color: "#03a9f4".to_string(),
                file_link_color: "#ea95ff".to_string(),
                symlink_link_color: "#ff9800".to_string(),
                table_background: "#353946".to_string(),
                table_text_color: "#eeeeee".to_string(),
                table_header_background: "#5294e2".to_string(),
                table_header_text_color: "#eeeeee".to_string(),
                table_header_active_color: "#ffffff".to_string(),
                active_row_color: "#5194e259".to_string(),
                odd_row_background: "#404552".to_string(),
                even_row_background: "#4b5162".to_string(),
                root_link_color: "#abb2bb".to_string(),
                download_button_background: "#ea95ff".to_string(),
                download_button_background_hover: "#eea7ff".to_string(),
                download_button_link_color: "#ffffff".to_string(),
                download_button_link_color_hover: "#ffffff".to_string(),
                back_button_background: "#ea95ff".to_string(),
                back_button_background_hover: "#ea95ff".to_string(),
                back_button_link_color: "#ffffff".to_string(),
                back_button_link_color_hover: "#ffffff".to_string(),
                date_text_color: "#9ebbdc".to_string(),
                at_color: "#9ebbdc".to_string(),
                switch_theme_background: "#4b5162".to_string(),
                switch_theme_link_color: "#fefefe".to_string(),
                switch_theme_active: "#ea95ff".to_string(),
                switch_theme_border: "#6a728a".to_string(),
                change_theme_link_color: "#fefefe".to_string(),
                change_theme_link_color_hover: "#fefefe".to_string(),
                upload_text_color: "#fefefe".to_string(),
                upload_form_border_color: "#353946".to_string(),
                upload_form_background: "#4b5162".to_string(),
                upload_button_background: "#ea95ff".to_string(),
                upload_button_text_color: "#ffffff".to_string(),
                drag_background: "#3333338f".to_string(),
                drag_border_color: "#fefefe".to_string(),
                drag_text_color: "#fefefe".to_string(),
                size_background_color: "#5294e2".to_string(),
                size_text_color: "#fefefe".to_string(),
            },
            ColorScheme::Zenburn => Theme {
                background: "#3f3f3f".to_string(),
                text_color: "#efefef".to_string(),
                directory_link_color: "#f0dfaf".to_string(),
                file_link_color: "#87D6D5".to_string(),
                symlink_link_color: "#FFCCEE".to_string(),
                table_background: "#4a4949".to_string(),
                table_text_color: "#efefef".to_string(),
                table_header_background: "#7f9f7f".to_string(),
                table_header_text_color: "#efefef".to_string(),
                table_header_active_color: "#efef8f".to_string(),
                active_row_color: "#7e9f7f9c".to_string(),
                odd_row_background: "#777777".to_string(),
                even_row_background: "#5a5a5a".to_string(),
                root_link_color: "#dca3a3".to_string(),
                download_button_background: "#cc9393".to_string(),
                download_button_background_hover: "#dca3a3".to_string(),
                download_button_link_color: "#efefef".to_string(),
                download_button_link_color_hover: "#efefef".to_string(),
                back_button_background: "#cc9393".to_string(),
                back_button_background_hover: "#cc9393".to_string(),
                back_button_link_color: "#efefef".to_string(),
                back_button_link_color_hover: "#efefef".to_string(),
                date_text_color: "#cfbfaf".to_string(),
                at_color: "#cfbfaf".to_string(),
                switch_theme_background: "#4a4949".to_string(),
                switch_theme_link_color: "#efefef".to_string(),
                switch_theme_active: "#efef8f".to_string(),
                switch_theme_border: "#5a5a5a".to_string(),
                change_theme_link_color: "#efefef".to_string(),
                change_theme_link_color_hover: "#efefef".to_string(),
                upload_text_color: "#efefef".to_string(),
                upload_form_border_color: "#4a4949".to_string(),
                upload_form_background: "#777777".to_string(),
                upload_button_background: "#cc9393".to_string(),
                upload_button_text_color: "#efefef".to_string(),
                drag_background: "#3333338f".to_string(),
                drag_border_color: "#efefef".to_string(),
                drag_text_color: "#efefef".to_string(),
                size_background_color: "#7f9f7f".to_string(),
                size_text_color: "#efefef".to_string(),
            },
            ColorScheme::Monokai => Theme {
                background: "#272822".to_string(),
                text_color: "#F8F8F2".to_string(),
                directory_link_color: "#F92672".to_string(),
                file_link_color: "#A6E22E".to_string(),
                symlink_link_color: "#FD971F".to_string(),
                table_background: "#3B3A32".to_string(),
                table_text_color: "#F8F8F0".to_string(),
                table_header_background: "#75715E".to_string(),
                table_header_text_color: "#F8F8F2".to_string(),
                table_header_active_color: "#E6DB74".to_string(),
                active_row_color: "#ae81fe3d".to_string(),
                odd_row_background: "#3E3D32".to_string(),
                even_row_background: "#49483E".to_string(),
                root_link_color: "#66D9EF".to_string(),
                download_button_background: "#AE81FF".to_string(),
                download_button_background_hover: "#c6a6ff".to_string(),
                download_button_link_color: "#F8F8F0".to_string(),
                download_button_link_color_hover: "#F8F8F0".to_string(),
                back_button_background: "#AE81FF".to_string(),
                back_button_background_hover: "#AE81FF".to_string(),
                back_button_link_color: "#F8F8F0".to_string(),
                back_button_link_color_hover: "#F8F8F0".to_string(),
                date_text_color: "#66D9EF".to_string(),
                at_color: "#66D9EF".to_string(),
                switch_theme_background: "#3B3A32".to_string(),
                switch_theme_link_color: "#F8F8F2".to_string(),
                switch_theme_active: "#A6E22E".to_string(),
                switch_theme_border: "#49483E".to_string(),
                change_theme_link_color: "#F8F8F2".to_string(),
                change_theme_link_color_hover: "#F8F8F2".to_string(),
                upload_text_color: "#F8F8F2".to_string(),
                upload_form_border_color: "#3B3A32".to_string(),
                upload_form_background: "#49483E".to_string(),
                upload_button_background: "#AE81FF".to_string(),
                upload_button_text_color: "#F8F8F0".to_string(),
                drag_background: "#3333338f".to_string(),
                drag_border_color: "#F8F8F2".to_string(),
                drag_text_color: "#F8F8F2".to_string(),
                size_background_color: "#75715E".to_string(),
                size_text_color: "#F8F8F2".to_string(),
            },
            ColorScheme::Squirrel => Theme {
                background: "#FFFFFF".to_string(),
                text_color: "#323232".to_string(),
                directory_link_color: "#d02474".to_string(),
                file_link_color: "#0086B3".to_string(),
                symlink_link_color: "#ED6A43".to_string(),
                table_background: "#ffffff".to_string(),
                table_text_color: "#323232".to_string(),
                table_header_background: "#323232".to_string(),
                table_header_text_color: "#F5F5F5".to_string(),
                table_header_active_color: "#FFFFFF".to_string(),
                active_row_color: "#f6f8fa".to_string(),
                odd_row_background: "#fbfbfb".to_string(),
                even_row_background: "#f2f2f2".to_string(),
                root_link_color: "#323232".to_string(),
                download_button_background: "#d02474".to_string(),
                download_button_background_hover: "#f52d8a".to_string(),
                download_button_link_color: "#FFFFFF".to_string(),
                download_button_link_color_hover: "#FFFFFF".to_string(),
                back_button_background: "#d02474".to_string(),
                back_button_background_hover: "#d02474".to_string(),
                back_button_link_color: "#FFFFFF".to_string(),
                back_button_link_color_hover: "#FFFFFF".to_string(),
                date_text_color: "#797979".to_string(),
                at_color: "#797979".to_string(),
                switch_theme_background: "#323232".to_string(),
                switch_theme_link_color: "#F5F5F5".to_string(),
                switch_theme_active: "#d02474".to_string(),
                switch_theme_border: "#49483E".to_string(),
                change_theme_link_color: "#F5F5F5".to_string(),
                change_theme_link_color_hover: "#F5F5F5".to_string(),
                upload_text_color: "#323232".to_string(),
                upload_form_border_color: "#d2d2d2".to_string(),
                upload_form_background: "#f2f2f2".to_string(),
                upload_button_background: "#d02474".to_string(),
                upload_button_text_color: "#FFFFFF".to_string(),
                drag_background: "#3333338f".to_string(),
                drag_border_color: "#ffffff".to_string(),
                drag_text_color: "#ffffff".to_string(),
                size_background_color: "#323232".to_string(),
                size_text_color: "#FFFFFF".to_string(),
            },
        }
    }
}

/// Describes a theme
pub struct Theme {
    pub background: String,
    pub text_color: String,
    pub directory_link_color: String,
    pub file_link_color: String,
    pub symlink_link_color: String,
    pub table_background: String,
    pub table_text_color: String,
    pub table_header_background: String,
    pub table_header_text_color: String,
    pub table_header_active_color: String,
    pub active_row_color: String,
    pub odd_row_background: String,
    pub even_row_background: String,
    pub root_link_color: String,
    pub download_button_background: String,
    pub download_button_background_hover: String,
    pub download_button_link_color: String,
    pub download_button_link_color_hover: String,
    pub back_button_background: String,
    pub back_button_background_hover: String,
    pub back_button_link_color: String,
    pub back_button_link_color_hover: String,
    pub date_text_color: String,
    pub at_color: String,
    pub switch_theme_background: String,
    pub switch_theme_link_color: String,
    pub switch_theme_active: String,
    pub switch_theme_border: String,
    pub change_theme_link_color: String,
    pub change_theme_link_color_hover: String,
    pub upload_text_color: String,
    pub upload_form_border_color: String,
    pub upload_form_background: String,
    pub upload_button_background: String,
    pub upload_button_text_color: String,
    pub drag_background: String,
    pub drag_border_color: String,
    pub drag_text_color: String,
    pub size_background_color: String,
    pub size_text_color: String,
}
