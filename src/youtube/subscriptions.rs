use chrono::TimeZone;

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
    Id,
    Snippet,
    SubscriberSnippet,
}
#[cfg_attr(feature = "clap-derive", derive(ValueEnum))]
#[derive(Clone, Debug)]
pub enum Order {
    Alphabetical,
    Relevance,
    Unread,
}
#[derive(Clone, Debug)]
pub enum Props {
    Part(Vec<PartProps>),
    ChannelId(String),
    Id(String),
    Mine(bool),
    MyRecentSubscribers(bool),
    MySubscribers(bool),
    ForChannelId(String),
    MaxResults(usize),
    OnBehalfOfContentOwner(String),
    OnBehalfOfContentOwnerChannel(String),
    Order(Order),
    PageToken(String),
}

#[derive(Clone)]
pub struct YTSubscriptionsBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props>,
}
impl<'yt, Tz: TimeZone> Builder for YTSubscriptionsBuilder<'yt, Tz> {
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Api = YTSubscriptions<'yt, Tz>;
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
                            PartProps::Id => "id".to_string(),
                            PartProps::Snippet => "snippet".to_string(),
                            PartProps::SubscriberSnippet => "subscriberSnippet".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )),
                Props::ChannelId(id) => ApiPropType::Filter(format!("channelId={}", id)),
                Props::Id(id) => ApiPropType::Filter(format!("id={}", id)),
                Props::Mine(b) => ApiPropType::Filter(format!("mine={}", b)),
                Props::MyRecentSubscribers(b) => {
                    ApiPropType::Filter(format!("myRecentSubscribers={}", b))
                }
                Props::MySubscribers(b) => ApiPropType::Filter(format!("mySubscribers={}", b)),
                Props::ForChannelId(_) => todo!(),
                Props::MaxResults(mr) => ApiPropType::Optional(format!("maxResults={}", mr)),
                Props::OnBehalfOfContentOwner(_) => todo!(),
                Props::OnBehalfOfContentOwnerChannel(_) => todo!(),
                Props::Order(o) => match o {
                    Order::Alphabetical => ApiPropType::Optional("order=alphabetical".to_string()),
                    Order::Relevance => ApiPropType::Optional("order=relevance".to_string()),
                    Order::Unread => ApiPropType::Optional("order=unread".to_string()),
                },
                Props::PageToken(pt) => ApiPropType::Optional(format!("pageToken={}", pt)),
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

pub struct YTSubscriptions<'yt, Tz: TimeZone> {
    builder: YTSubscriptionsBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTSubscriptions<'yt, Tz> {
    type ApiBuilder = YTSubscriptionsBuilder<'yt, Tz>;

    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTSubscriptions<'api, Tz> {
    const URL: &'api str = "subscriptions?part=snippet&";
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
