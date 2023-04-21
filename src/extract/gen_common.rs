use super::gen_quest::*;
use super::gen_website::*;
use super::pedia::*;
use crate::msg::MsgEntry;
use typed_html::{elements::*, html, text};

const WEBSITE_VERSIONS: &[&str] = &[
    "10.0.2", "10.0.3", "11.0.1", "11.0.2", "12.0.0", "12.0.1", "13.0.0", "14.0.0", "15.0.0",
];

pub fn open_graph(
    title: Option<&MsgEntry>,
    title_plan: &str,
    description: Option<&MsgEntry>,
    description_plan: &str,
    image: Option<&str>,
    path: &str,
    config: &WebsiteConfig,
) -> Vec<Box<dyn MetadataContent<String>>> {
    let Some(origin) = &config.origin else {return vec![]};
    let mut title = if let Some(title) = title {
        translate_msg_plain(&title.content[1])
    } else {
        title_plan.to_owned()
    };
    if title.is_empty() {
        title = "MHRice".to_owned()
    }
    let mut description = if let Some(description) = description {
        translate_msg_plain(&description.content[1]).replace("\r\n", " ")
    } else {
        description_plan.to_owned()
    };
    if description.is_empty() {
        description = " ".to_owned(); // avoid empty meta attribute
    }
    let image = image.unwrap_or("/favicon.png");
    let image = origin.clone() + image;
    let url = origin.clone() + path;
    vec![
        html!(<meta property="og:type" content="website" />),
        html!(<meta property="og:title" content={title} />),
        html!(<meta property="og:description" content={description.as_str()} />),
        html!(<meta property="og:image" content={image} />),
        html!(<meta property="og:url" content={url} />),
        html!(<meta property="og:site_name" content="MHRice" />),
        html!(<meta name="description" content={description.as_str()} />),
    ]
}

pub struct Section {
    pub title: String,
    pub content: Box<section<String>>,
}

pub fn gen_menu(sections: &[Section]) -> Box<aside<String>> {
    html!(<aside id="left-aside">
    <div class="aside-button" id="left-aside-button"/>
    <div class="side-menu">
    <p class="menu-label">
        "On this page"
    </p>
    <ul class="menu-list">
        {sections.iter().map(|s| {
            let href = format!("#{}", s.content.attrs.id.as_ref().unwrap());
            html!(<li><a href={href.as_str()} class="left-aside-item">
                {text!("{}", s.title)}
            </a></li>)
        })}
    </ul>
    </div>
    </aside>)
}

pub fn right_aside() -> Box<aside<String>> {
    type VersionRow<'a> = Vec<(
        &'a str, /*minor*/
        String,  /*url*/
        bool,    /*latest*/
    )>;
    let mut version_tree: Vec<(&str /*major*/, VersionRow)> = vec![];

    for (i, version) in WEBSITE_VERSIONS.iter().enumerate() {
        let latest = i == WEBSITE_VERSIONS.len() - 1;
        let dot = version.find('.').unwrap();
        let (major, minor) = version.split_at(dot);
        let url = if latest {
            "".to_owned()
        } else {
            "-".to_owned() + &version.replace('.', "-")
        };
        if version_tree.last().map(|v| v.0) == Some(major) {
            version_tree
                .last_mut()
                .unwrap()
                .1
                .push((minor, url, latest))
        } else {
            version_tree.push((major, vec![(minor, url, latest)]))
        }
    }

    html!(<aside id="right-aside">
    <div class="aside-button" id="right-aside-button"/>
    <div class="side-menu">
    <p class="menu-label">
        "Language"
    </p>
    <ul class="menu-list">
    {
        (0..32).filter_map(|i| {
            let (language_name, language_code) = LANGUAGE_MAP[i]?;
            let id_string = format!("mh-lang-menu-{language_code}");
            Some(html!{<li><button type="button" class="mh-lang-menu" id={id_string.as_str()}> {
                text!("{}", language_name)
            }</button></li>})
        })
    }
    </ul>

    <p class="menu-label">
        "Version"
    </p>
    <ul class="menu-list">{
        version_tree.into_iter().map(|(major, minors)| {
            html!(<li class="mh-version-block">
                <span class="mh-major">{text!("{}", major)}</span>
                {minors.into_iter().map(|(minor, url, latest)| {
                    let href = format!("https://mhrise{url}.mhrice.info");
                    let mut class = "mh-version-menu".to_owned();
                    if latest {
                        class += " mh-version-menu-latest";
                    }
                    html!(<a class={class.as_str()} href={href.as_str()}>
                        {text!("{}", minor)}
                    </a>)
                })}
            </li>)
        })
    }</ul>


    <p class="menu-label">
        "Website info"
    </p>
    <ul class="menu-list">
    <li><a class="navbar-item" href="/about.html">
        "About MHRice"
    </a></li>
    </ul>

    </div>
    </aside>)
}

pub fn gen_slot(decorations_num_list: &[u32], is_rampage_slot: bool) -> Box<span<String>> {
    let mut slot_list = vec![];

    for (i, num) in decorations_num_list.iter().enumerate().rev() {
        for _ in 0..*num {
            slot_list.push(i);
        }
    }

    let placeholder = if slot_list.len() < 3 {
        3 - slot_list.len()
    } else {
        0
    };

    html!(<span>
        {slot_list.into_iter().map(|s| {
            let alt = format!("A level-{} slot", s + 1);
            let class = if s == 3 {
                "mh-slot-large"
            } else {
                "mh-slot"
            };
            html!(
                <span class="mh-slot-outer">
                    <img alt={alt.as_str()}
                        src={format!("/resources/slot_{s}.png").as_str()} class={class} />
                    { is_rampage_slot.then(||html!(<img alt="Rampage slot" class="mh-slot-rampage"
                        src="/resources/slot_rampage.png" />)) }
                </span>
            )
        })}
        {(0..placeholder).map(|_| html!(<span class="mh-slot-outer">
            <span class="mh-slot-0"/>
        </span>))}
    </span>)
}

pub fn gen_progress(progress_flag: i32, pedia_ex: &PediaEx) -> Box<div<String>> {
    if progress_flag == 0 {
        return html!(<div>"None"</div>);
    }
    let progress = if let Some(&progress) = pedia_ex.progress.get(&progress_flag) {
        progress
    } else {
        return html!(<div>{text!("Unknown progress {}", progress_flag)}</div>);
    };
    let mut flags = vec![];
    if let Some(village) = progress.village.display() {
        flags.push(html!(<div>{text!("{}", village)}</div>));
    }
    if let Some(hall) = progress.hall.display() {
        flags.push(html!(<div>{text!("{}", hall)}</div>));
    }
    if let Some(hall) = progress.talk_flag_hall.display() {
        flags.push(html!(<div>{text!("NPC:{}", hall)}</div>));
    }
    if let Some(mr) = progress.mr.display() {
        flags.push(html!(<div>{text!("{}", mr)}</div>));
    }
    if progress.quest_no != -1 {
        if let Some(quest) = pedia_ex.quests.get(&progress.quest_no) {
            flags.push(html!(<div>"Quest:"{gen_quest_tag(quest, false, false, None, None)}</div>));
        } else {
            flags.push(html!(<div>{text!("Quest:{}", progress.quest_no)}</div>));
        }
    }
    if progress.talk_flag != -1 {
        // TODO: fast search
        if let Some(mission) = pedia_ex
            .npc_missions
            .values()
            .find(|m| m.param.end_flag == progress.talk_flag)
        {
            flags.push(gen_npc_mission_tag(mission))
        } else {
            flags.push(html!(<div>{text!("NPC:{}", progress.talk_flag)}</div>));
        }
    }
    if progress.enable_progress_hr_check {
        flags.push(html!(<div>{text!("Check:{:?}", progress.progress_hr)}</div>));
    }

    html!(<div>{flags}</div>)
}
