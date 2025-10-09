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
    Id,
    Snippet,
    Status,
}
#[derive(Clone, Debug)]
pub enum Props {
    Part(Vec<PartProps>),
    Id(String),
    ChannelId(String),
    Hl(String),
    MaxResults(usize),
    OnBehalfOfContentOwner(String),
    PageToken(String),
    VideoId(String),
}

pub struct YTPlaylistsBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props>,
}
impl<'yt, Tz: TimeZone> Builder for YTPlaylistsBuilder<'yt, Tz> {
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Api = YTPlaylists<'yt, Tz>;
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
                            PartProps::Status => "status".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )),
                Props::Id(id) => ApiPropType::Filter(format!("id={}", id)),
                Props::ChannelId(id) => ApiPropType::Filter(format!("channelId={}", id)),
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

pub struct YTPlaylists<'yt, Tz: TimeZone> {
    builder: YTPlaylistsBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTPlaylists<'yt, Tz> {
    type ApiBuilder = YTPlaylistsBuilder<'yt, Tz>;

    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTPlaylists<'api, Tz> {
    const URL: &'api str = "playlists?";
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
