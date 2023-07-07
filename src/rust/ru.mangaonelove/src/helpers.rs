use aidoku::{helpers::substring::Substring, prelude::*, Manga, MangaPageResult, MangaStatus};
use alloc::{
	string::{String, ToString},
	vec::Vec,
};

use crate::constants::{MANGA_BASE_URL, SEARCH_OFFSET_STEP};

pub fn get_manga_url(id: &str) -> String {
	format!("{MANGA_BASE_URL}/{id}")
}

pub fn get_manga_id(url: &str) -> Option<String> {
	url.trim_end_matches('/')
		.substring_after_last('/')
		.map(|s| s.to_string())
}

pub fn create_manga_page_result(mangas: Vec<Manga>, has_more: Option<bool>) -> MangaPageResult {
	let has_more = has_more.unwrap_or(mangas.len() == SEARCH_OFFSET_STEP as usize);
	MangaPageResult {
		manga: mangas,
		has_more,
	}
}

pub fn get_chapter_url(manga_id: &str, chapter_id: &str) -> String {
	// ?style=list is to preload all images
	format!("{MANGA_BASE_URL}/{manga_id}/{chapter_id}/?style=list")
}

pub fn parse_status(status_str: &str) -> MangaStatus {
	match status_str.trim() {
		"Онгоинг" => MangaStatus::Ongoing,
		"Завершен" => MangaStatus::Completed,
		"Брошено" => MangaStatus::Cancelled,
		"Заморожен" => MangaStatus::Hiatus,
		_ => MangaStatus::Unknown,
	}
}
