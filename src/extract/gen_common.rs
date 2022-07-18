use typed_html::{elements::*, html};

pub fn gen_slot(decorations_num_list: &[u32; 4]) -> Box<span<String>> {
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
        {(0..placeholder).map(|_| html!(<span class="mh-slot" />))}
        {slot_list.into_iter().map(|s| {
            let alt = format!("A level-{} slot", s + 1);
            let class = if s == 3 {
                "mh-slot-large"
            } else {
                "mh-slot"
            };
            html!(<img alt={alt.as_str()} src={format!("/resources/slot_{}.png", s).as_str()} class={class} />)
        })}
    </span>)
}
