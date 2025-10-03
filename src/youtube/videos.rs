use clap::ValueEnum;

use super::YTApi;
use crate::Api;

#[derive(Clone, Debug, ValueEnum)]
pub enum Rating {
    Like,
    Dislike,
}
impl Rating {
    fn to_string(&self) -> String {
        match self {
            Rating::Like => "like".to_string(),
            Rating::Dislike => "dislike".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum VideosFilter {
    Chart,
    Id(String),
    MyRating(Rating),
}
impl VideosFilter {
    fn to_string(&self) -> String {
        match self {
            VideosFilter::Chart => format!("chart=mostPopular"),
            VideosFilter::Id(s) => format!("id={}", s),
            VideosFilter::MyRating(r) => format!("myRating={}", r.to_string()),
        }
    }
}

pub struct YTVideosBuilder<'yt> {
    api: &'yt YTApi<'yt>,
    filter: Option<VideosFilter>,
    video_category: Option<String>,
}
pub struct YTVideos<'yt> {
    builder: YTVideosBuilder<'yt>,
}

impl<'yt> From<&'yt YTApi<'yt>> for YTVideosBuilder<'yt> {
    fn from(value: &'yt YTApi<'yt>) -> Self {
        Self {
            api: value,
            filter: None,
            video_category: None,
        }
    }
}

impl<'s> YTVideosBuilder<'s> {
    pub fn build(self) -> YTVideos<'s> {
        YTVideos { builder: self }
    }
    pub fn filter(mut self, filter: VideosFilter) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn video_category(mut self, video_category: Option<String>) -> Self {
        self.video_category = video_category;
        self
    }
}

impl<'s> YTVideos<'s> {}

impl<'api> Api<'api> for YTVideos<'api> {
    const URL: &'api str = "videos?part=snippet&";
    fn url(&self) -> String {
        let props = [
            if let Some(vc) = self.builder.video_category.clone() {
                Some(["videoCategoryId=", vc.as_str()].join(""))
            } else {
                None
            },
            Some(
                [self
                    .builder
                    .filter
                    .clone()
                    .expect("Filter is not set (mine, channelId)")
                    .to_string()
                    .as_str()]
                .join(""),
            ),
            // Some(["key=", self.builder.api.key.as_str()].join("")),
        ]
        .into_iter()
        .filter_map(|p| p)
        .collect::<Vec<String>>()
        .join("&");
        [&self.builder.api.base_url, Self::URL, props.as_str()].join("")
    }
}
