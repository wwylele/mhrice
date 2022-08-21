use super::gen_website::*;
use typed_html::{elements::*, html, text};

const WEBSITE_VERSIONS: &[&str] = &["10.0.2", "10.0.3", "11.0.1"];
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
            Some(html!{ <li><a class="mh-lang-menu" id={id_string.as_str()}> {
                text!("{}", language_name)
            }</a></li>})
        })
    }
    </ul>

    <p class="menu-label">
        "Use cookie to save preference"
    </p>
    <div id="cookie-consent" class="buttons has-addons">
        <button id="cookie-yes" class="button is-small">"Yes"</button>
        <button id="cookie-no" class="button is-small is-selected is-danger">"No"</button>
    </div>

    <p class="menu-label">
        "Version"
    </p>
    <ul class="menu-list">{
        WEBSITE_VERSIONS.iter().enumerate().map(|(i, &version)| {
            let latest = i == WEBSITE_VERSIONS.len() - 1;
            let href = if latest {
                "https://mhrise.mhrice.info".to_owned()
            } else {
                format!("https://mhrise-{}.mhrice.info", version.replace('.', "-"))
            };
            let text = if latest {
                format!("{version} (Latest)")
            } else {
                version.to_owned()
            };
            let mut class = "navbar-item mh-version-menu".to_owned();
            if latest {
                class += " mh-version-menu-latest";
            }
            html!(<li><a class={class.as_str()} href={href.as_str()}>
                {text!("{}", text)}
            </a></li>)
        })
    }</ul>


    <p class="menu-label">
        "Website info"
    </p>
    <ul class="menu-list">
    <li><a class="navbar-item" href="/about.html">
        "About MHRise"
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
                        src={format!("/resources/slot_{}.png", s).as_str()} class={class} />
                    { is_rampage_slot.then(||html!(<img class="mh-slot-rampage"
                        src="/resources/slot_rampage.png" />)) }
                </span>
            )
        })}
        {(0..placeholder).map(|_| html!(<span class="mh-slot-outer">
            <span class="mh-slot-0"/>
        </span>))}
    </span>)
}
