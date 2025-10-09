use chrono::TimeZone;
use std::marker::PhantomData;

use crate::{
    Api,
    youtube::{
        activities::YTActivitiesBuilder,
        playlists::YTPlaylistsBuilder,
        search::YTSearchBuilder,
        subscriptions::YTSubscriptionsBuilder,
        videos::YTVideosBuilder, // channels::YTChannelsBuilder, search::YTSearchBuilder,
                                 // subscriptions::YTSubscriptionsBuilder, videos::YTVideosBuilder,
    },
};

pub mod auth;

pub mod activities;
// pub mod channels;
pub mod playlists;
pub mod search;
pub mod subscriptions;
pub mod videos;

pub mod pager;

pub struct YTApi<'yt, Tz: TimeZone> {
    base_url: &'yt str,
    #[allow(unused)]
    key: Option<String>,
    _phantom: PhantomData<Tz>,
}

pub trait Builder: Sized {
    type ParentApi;
    type Api: BuildableApi<ApiBuilder = Self>;
    type Prop: Clone;
    const MIN_FILTERS: usize;
    const MAX_FILTERS: usize;

    fn add_prop(&mut self, prop: Self::Prop);
    fn props(&self) -> Vec<ApiPropType>;
    fn from_parent(parent: Self::ParentApi) -> Self;

    fn build(self) -> Result<Self::Api, String> {
        let props = self.props();
        let required = props
            .iter()
            .filter(|p| matches!(p, ApiPropType::Required(_)))
            .count();
        let filters = props
            .iter()
            .filter(|p| matches!(p, ApiPropType::Filter(_)))
            .count();
        if required > 0 && filters >= Self::MIN_FILTERS && filters <= Self::MAX_FILTERS {
            Ok(Self::Api::from_builder(self, props))
        } else {
            Err(format!(
                "Incorrect number of properties.\nRequired: {} (must be at least one).\nFilters: {}(must be at least {} and no more than {}.",
                required,
                filters,
                Self::MIN_FILTERS,
                Self::MAX_FILTERS
            ))
        }
    }
}
pub trait BuildableApi {
    type ApiBuilder: Builder;
    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self;
}

#[derive(Clone)]
pub enum ApiPropType {
    Required(String),
    Filter(String),
    Optional(String),
}

impl<'yt, Tz: TimeZone> YTApi<'yt, Tz> {
    pub fn new(key: Option<String>) -> Self {
        Self {
            base_url: Self::URL,
            key,
            _phantom: PhantomData,
        }
    }

    pub fn search(&'yt self) -> YTSearchBuilder<'yt, Tz> {
        YTSearchBuilder::from_parent(self)
    }
    pub fn activities(&'yt self) -> YTActivitiesBuilder<'yt, Tz> {
        YTActivitiesBuilder::from_parent(self)
    }
    pub fn subscriptions(&'yt self) -> YTSubscriptionsBuilder<'yt, Tz> {
        YTSubscriptionsBuilder::from_parent(self)
    }
    pub fn videos(&'yt self) -> YTVideosBuilder<'yt, Tz> {
        YTVideosBuilder::from_parent(self)
    }
    pub fn playlists(&'yt self) -> YTPlaylistsBuilder<'yt, Tz> {
        YTPlaylistsBuilder::from_parent(self)
    }
    // pub fn channels(&'yt self) -> YTChannelsBuilder<'yt> {
    //     YTChannelsBuilder::from(self)
    // }
}

impl<'a, Tz: TimeZone> Api<'a> for YTApi<'a, Tz> {
    const URL: &'a str = "https://www.googleapis.com/youtube/v3/";
    fn url(&self) -> String {
        self.base_url.to_string()
    }
}
