use chrono::{DateTime, TimeZone};
use codes_iso_3166::part_1::CountryCode;

use crate::{
    Api,
    youtube::{ApiPropType, BuildableApi, Builder, YTApi},
};

#[cfg(feature = "clap-derive")]
use clap::ValueEnum;

#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Debug, Clone)]
pub enum PartProps {
    ContentDetails,
    Id,
    Snippet,
}

#[derive(Debug, Clone)]
pub enum Props<Tz: TimeZone> {
    Part(Vec<PartProps>),
    ChannelId(String),
    #[deprecated]
    Home(bool),
    Mine(bool),
    MaxResults(usize),
    PageToken(String),
    PublishedAfter(DateTime<Tz>),
    PublishedBefore(DateTime<Tz>),
    RegionCode(CountryCode),
}

pub struct YTActivitiesBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props<Tz>>,
}
impl<'yt, Tz: TimeZone> Builder for YTActivitiesBuilder<'yt, Tz> {
    type Api = YTActivities<'yt, Tz>;
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Prop = Props<Tz>;
    const MIN_FILTERS: usize = 1;
    const MAX_FILTERS: usize = 1;
    fn props(&self) -> Vec<ApiPropType> {
        self.props
            .clone()
            .into_iter()
            .map(|p| match p {
                Props::Part(p) => ApiPropType::Required(format!(
                    "part={}",
                    p.iter()
                        .map(|part| match part {
                            PartProps::ContentDetails => "contentDetails".to_string(),
                            PartProps::Id => "id".to_string(),
                            PartProps::Snippet => "snippet".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )),
                Props::ChannelId(id) => ApiPropType::Filter(format!("channelId={}", id)),
                // Props::Home(b) => ApiPropType::Filter(format!("home={}", b)),
                Props::Mine(b) => ApiPropType::Filter(format!("mine={}", b)),
                Props::MaxResults(mr) => {
                    ApiPropType::Optional(format!("maxResults={}", mr).to_string())
                }
                Props::PageToken(v) => ApiPropType::Optional(format!("pageToken={}", v)),
                Props::PublishedAfter(v) => ApiPropType::Optional(format!(
                    "publishedAfter={}",
                    v.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                )),
                Props::PublishedBefore(v) => ApiPropType::Optional(format!(
                    "publishedBefore={}",
                    v.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                )),
                Props::RegionCode(v) => ApiPropType::Optional(format!("regionCode={}", v)),
                _ => panic!("Deprecated"),
            })
            .collect::<Vec<ApiPropType>>()
    }

    fn from_parent(parent: Self::ParentApi) -> Self {
        Self {
            api: parent,
            props: vec![],
        }
    }

    fn add_prop(&mut self, prop: Self::Prop) {
        self.props.push(prop);
    }
}

pub struct YTActivities<'yt, Tz: TimeZone> {
    builder: YTActivitiesBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTActivities<'yt, Tz> {
    type ApiBuilder = YTActivitiesBuilder<'yt, Tz>;
    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTActivities<'api, Tz> {
    const URL: &'api str = "activities?";
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
