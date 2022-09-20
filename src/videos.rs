use yew::prelude::*;

#[derive( Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Clone, Properties, PartialEq)]
struct Videos {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
fn videos_list (Videos {videos}: &Videos) -> Html {
    videos.iter().map(|video| html! {
        <p>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect::<Html>()
}