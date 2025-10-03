use super::YTApi;
use crate::Api;

#[derive(Clone)]
pub enum ChannelsFilter {
    Mine,
    Id(String),
}
impl ChannelsFilter {
    fn to_string(&self) -> String {
        match self {
            ChannelsFilter::Mine => "mine=true".to_string(),
            ChannelsFilter::Id(id) => format!("id={}", id),
        }
    }
}

pub struct YTChannelsBuilder<'yt> {
    api: &'yt YTApi<'yt>,
    filter: Option<ChannelsFilter>,
    video_category: Option<String>,
}
pub struct YTChannels<'yt> {
    builder: YTChannelsBuilder<'yt>,
}

impl<'yt> From<&'yt YTApi<'yt>> for YTChannelsBuilder<'yt> {
    fn from(value: &'yt YTApi<'yt>) -> Self {
        Self {
            api: value,
            filter: None,
            video_category: None,
        }
    }
}

impl<'s> YTChannelsBuilder<'s> {
    pub fn build(self) -> YTChannels<'s> {
        YTChannels { builder: self }
    }
    pub fn filter(mut self, filter: ChannelsFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl<'s> YTChannels<'s> {}

impl<'api> Api<'api> for YTChannels<'api> {
    const URL: &'api str = "channels?part=snippet,contentDetails&";
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
                    .expect("Filter is not set (mine)")
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
