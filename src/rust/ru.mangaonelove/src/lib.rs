#![no_std]
#![feature(pattern)]
#![feature(iter_intersperse)]

mod constants;
mod helpers;
mod parser;
mod wrappers;

use aidoku::{
	error::Result,
	prelude::*,
	std::{String, Vec},
	Chapter, DeepLink, Filter, Listing, Manga, MangaPageResult, Page,
};
use constants::BASE_URL;
use wrappers::WNode;

extern crate alloc;

#[get_manga_list]
pub fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let search_url = parser::get_filter_url(&filters, page)?;
	let html = wrappers::get_html(&search_url)?;
	let mangas = parser::parse_search_results(&html)?;
	Ok(helpers::create_manga_page_result(mangas, None))
}

#[get_manga_listing]
pub fn get_manga_listing(listing: Listing, _page: i32) -> Result<MangaPageResult> {
	let html = wrappers::get_html(BASE_URL)?;
	let mangas = parser::parse_lising(&html, listing)?;
	Ok(helpers::create_manga_page_result(mangas, Some(false)))
}

#[get_manga_details]
pub fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = helpers::get_manga_url(&manga_id);
	let html = wrappers::get_html(&url)?;
	parser::parse_manga(&html, manga_id).ok_or(WNode::PARSING_ERROR)
}

#[get_chapter_list]
pub fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = helpers::get_manga_url_readmanga(&manga_id);
	let html = wrappers::get_html(&url)?;
	parser::parse_chapters_readmanga(&html, &manga_id)
}

#[get_page_list]
pub fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = helpers::get_chapter_url_readmanga(&manga_id, &chapter_id);
	let html = wrappers::get_html(&url)?;
	parser::get_page_list_readmanga(&html)
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	parser::parse_incoming_url_readmanga(&url)
}
