use chrono::TimeZone;
use codes_iso_3166::part_1::CountryCode;

use super::YTApi;
use crate::{
    Api,
    youtube::{ApiPropType, BuildableApi, Builder},
};

#[cfg(feature = "clap-derive")]
use clap::ValueEnum;

#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum PartProps {
    ContentDetails,
    FileDetails,
    Id,
    LiveStreamingDetails,
    Localizations,
    PaidProductPlacementDetails,
    Player,
    ProcessingDetails,
    RecordingDetails,
    Snippet,
    Statistics,
    Status,
    Suggestions,
    TopicDetails,
}
#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum Chart {
    MostPopular,
}
#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum Rating {
    Like,
    Dislike,
}
#[derive(Clone, Debug)]
pub enum Props {
    Part(Vec<PartProps>),
    Chart(Chart),
    Id(String),
    MyRating(Rating),
    Hl(String),
    MaxHeight(usize),
    MaxWidth(usize),
    MaxResults(usize),
    OnBehalfOfContentOwner(String),
    PageToken(String),
    RegionCode(CountryCode),
    VideoCategoryId(String),
}

pub struct YTVideosBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props>,
}
impl<'yt, Tz: TimeZone> Builder for YTVideosBuilder<'yt, Tz> {
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Api = YTVideos<'yt, Tz>;
    type Prop = Props;
    const MIN_FILTERS: usize = 1;
    const MAX_FILTERS: usize = 1;

    fn add_prop(&mut self, prop: Self::Prop) {
        self.props.push(prop)
    }

    fn props(&self) -> Vec<super::ApiPropType> {
        self.props
            .clone()
            .into_iter()
            .map(|p| match p {
                Props::Part(part_props) => ApiPropType::Required(format!(
                    "part={}",
                    part_props
                        .iter()
                        .map(|part| match part {
                            PartProps::ContentDetails => "contentDetails".to_string(),
                            PartProps::FileDetails => "fileDetails".to_string(),
                            PartProps::Id => "id".to_string(),
                            PartProps::LiveStreamingDetails => "liveStreamingDetails".to_string(),
                            PartProps::Localizations => "localizations".to_string(),
                            PartProps::PaidProductPlacementDetails =>
                                "paidProductPlacementDetails".to_string(),
                            PartProps::Player => "player".to_string(),
                            PartProps::ProcessingDetails => "processingDetails".to_string(),
                            PartProps::RecordingDetails => "recordingDetails".to_string(),
                            PartProps::Snippet => "snippet".to_string(),
                            PartProps::Statistics => "statistics".to_string(),
                            PartProps::Status => "status".to_string(),
                            PartProps::Suggestions => "suggestions".to_string(),
                            PartProps::TopicDetails => "topicDetails".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )),
                Props::Chart(ch) => ApiPropType::Filter(format!(
                    "chart={}",
                    match ch {
                        Chart::MostPopular => "mostPopular".to_string(),
                    }
                )),
                Props::MyRating(r) => ApiPropType::Filter(format!(
                    "myRating={}",
                    match r {
                        Rating::Like => "like".to_string(),
                        Rating::Dislike => "dislike".to_string(),
                    }
                )),
                Props::Id(id) => ApiPropType::Filter(format!("id={}", id)),
                Props::MaxResults(mr) => ApiPropType::Optional(format!("maxResults={}", mr)),
                Props::OnBehalfOfContentOwner(_) => todo!(),
                Props::PageToken(_) => todo!(),
                _ => todo!(),
            })
            .collect::<Vec<ApiPropType>>()
    }

    fn from_parent(parent: Self::ParentApi) -> Self {
        Self {
            api: parent,
            props: vec![],
        }
    }
}

pub struct YTVideos<'yt, Tz: TimeZone> {
    builder: YTVideosBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTVideos<'yt, Tz> {
    type ApiBuilder = YTVideosBuilder<'yt, Tz>;

    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTVideos<'api, Tz> {
    const URL: &'api str = "videos?";
    fn url(&self) -> String {
        let props = self
            .props
            .iter()
            .map(|p| match p {
                ApiPropType::Required(s) => s.clone(),
                ApiPropType::Filter(s) => s.clone(),
                ApiPropType::Optional(s) => s.clone(),
            })
            .collect::<Vec<String>>()
            .join("&");
        [&self.builder.api.base_url, Self::URL, props.as_str()].join("")
    }
}
