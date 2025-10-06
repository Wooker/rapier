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
    Snippet,
}
#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum Order {
    Title,
    Relevance,
    Date,
}
#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum Type {
    Video,
    Playlist,
    Channel,
}
#[derive(Clone, Debug)]
pub enum Props {
    Part(Vec<PartProps>),
    ForContentOwner(bool),
    ForDeveloper(bool),
    ForMine(bool),
    ForChannelId(String),
    ChannelId(String),
    MaxResults(usize),
    Order(Order),
    PageToken(String),
    Query(String),
    Type(Type),
    RegionCode(CountryCode),
}

pub struct YTSearchBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props>,
}
impl<'yt, Tz: TimeZone> Builder for YTSearchBuilder<'yt, Tz> {
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Api = YTSearch<'yt, Tz>;
    type Prop = Props;
    const MIN_FILTERS: usize = 0;
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
                            PartProps::Snippet => "snippet".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )),
                Props::ChannelId(id) => ApiPropType::Filter(format!("channelId={}", id)),
                Props::ForChannelId(_) => todo!(),
                Props::MaxResults(mr) => ApiPropType::Optional(format!("maxResults={}", mr)),
                Props::PageToken(_) => todo!(),
                Props::ForContentOwner(_) => todo!(),
                Props::ForDeveloper(_) => todo!(),
                Props::ForMine(_) => todo!(),
                Props::Query(q) => ApiPropType::Optional(format!("q=\"{}\"", q)),
                Props::Order(o) => match o {
                    Order::Relevance => ApiPropType::Optional(format!("order={}", "relevance")),
                    Order::Title => ApiPropType::Optional(format!("order={}", "title")),
                    Order::Date => ApiPropType::Optional(format!("order={}", "date")),
                },
                Props::Type(t) => match t {
                    Type::Video => ApiPropType::Optional(format!("type={}", "video")),
                    Type::Playlist => ApiPropType::Optional(format!("type={}", "playlist")),
                    Type::Channel => ApiPropType::Optional(format!("type={}", "channel")),
                },
                Props::RegionCode(country_code) => {
                    ApiPropType::Optional(format!("regionCode={}", country_code))
                }
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

pub struct YTSearch<'yt, Tz: TimeZone> {
    builder: YTSearchBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTSearch<'yt, Tz> {
    type ApiBuilder = YTSearchBuilder<'yt, Tz>;

    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTSearch<'api, Tz> {
    const URL: &'api str = "search?";
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
