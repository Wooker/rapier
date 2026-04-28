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
    Status,
}
#[derive(Clone, Debug)]
pub enum Props {
    Part(Vec<PartProps>),
    Id(String),
    PlaylistId(String),
    Hl(String),
    MaxResults(usize),
    OnBehalfOfContentOwner(String),
    PageToken(String),
    VideoId(String),
}

pub struct YTPlaylistItemsBuilder<'yt, Tz: TimeZone> {
    api: &'yt YTApi<'yt, Tz>,
    props: Vec<Props>,
}
impl<'yt, Tz: TimeZone> Builder for YTPlaylistItemsBuilder<'yt, Tz> {
    type ParentApi = &'yt YTApi<'yt, Tz>;
    type Api = YTPlaylistItems<'yt, Tz>;
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
                Props::PlaylistId(id) => ApiPropType::Filter(format!("playlistId={}", id)),
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

pub struct YTPlaylistItems<'yt, Tz: TimeZone> {
    builder: YTPlaylistItemsBuilder<'yt, Tz>,
    props: Vec<ApiPropType>,
}
impl<'yt, Tz: TimeZone> BuildableApi for YTPlaylistItems<'yt, Tz> {
    type ApiBuilder = YTPlaylistItemsBuilder<'yt, Tz>;

    fn from_builder(builder: Self::ApiBuilder, props: Vec<ApiPropType>) -> Self {
        Self { builder, props }
    }
}
impl<'api, Tz: TimeZone> Api<'api> for YTPlaylistItems<'api, Tz> {
    const URL: &'api str = "playlistItems?";
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
