use typed_html::{elements::*, html};

pub fn gen_slot(decorations_num_list: &[u32; 3]) -> Box<span<String>> {
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
            html!(<img src={format!("/resources/slot_{}.png", s).as_str()} class="mh-slot" />)
        })}
    </span>)
}
