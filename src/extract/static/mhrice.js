"use strict";

let g_supported_mh_lang = [];
let g_language_code = "en";

let g_navbar_menu_active = false;

let g_classes_to_hide = new Set();

let g_cur_map_explain = "default";

let g_map_scale = 100;
let g_map_layer = 0;

let g_filter = "all";

let g_toc = null;

let g_map_pos = { top: 0, left: 0, x: 0, y: 0, container: null };

let g_diagram_current = new Map();
let g_diagram_template = new Map();

let g_weapon_masonry = null;

document.addEventListener('DOMContentLoaded', function () {
    delete_all_cookie();
    addEventListensers();

    for (const element of document.getElementsByClassName("mh-color-diagram-img")) {
        imgOnLoad(element, (img) => {
            const canvas = new OffscreenCanvas(img.naturalWidth, img.naturalHeight);
            const context = canvas.getContext('2d');
            context.drawImage(img, 0, 0, canvas.width, canvas.height);
            const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
            g_diagram_template.set(img.id, imageData);
        })
    }

    for (const element of document.getElementsByClassName("mh-lang-menu")) {
        g_supported_mh_lang.push(removePrefix(element.id, "mh-lang-menu-"));
    }

    loadPreference();
    switchLanguage();
    adjustVersionMenu();
    hide_class("mh-ride-cond");
    hide_class("mh-invalid-meat");
    hide_class("mh-invalid-part");
    hide_class("mh-no-preset");
    hide_class("mh-non-target");
    hide_class("mh-quest-detail");
    hide_class("mh-part-internal");
    hide_class("mh-hitzone-internal");

    change_sort("monster", 1);
    change_sort("item", 1);
    change_sort("armor", 1);
    //change_sort("npcmission", 1);

    initFilter("map");
    initFilter("item");
    initFilter("armor");
    initFilter("skill");
    initFilter("main")

    autoSearch();

    initWeaponTreeMasonry();
});

function imgOnLoad(img, callback) {
    const callbackWrapper = () => { callback(img) }
    img.addEventListener("load", callbackWrapper, false);
    if (img.complete) {
        callbackWrapper();
        img.removeEventListener("load", callbackWrapper, false);
    }
}

function initWeaponTreeMasonry() {
    if (document.getElementById("mh-weapon-tree") !== null) {
        g_weapon_masonry = new Masonry(".mh-weapon-tree-list>ul");
    }
}

function addEventListensers() {
    addEventListenerToClass("mh-lang-menu", "click", selectLanguage);
    addEventListenerToId("navbarBurger", "click", onToggleNavbarMenu);

    addEventListenerToId("left-aside-button", "click", onToggleLeftAside);
    addEventListenerToId("right-aside-button", "click", onToggleRightAside);
    // doesn't work on all platform
    // addEventListenerToClass("left-aside-item", "click", onLeftAsideItem);

    addEventListenerToId("mh-search", "keydown", search);
    addEventListenerToId("nav-search", "keydown", goSearch);

    addEventListenerToClass("mh-item-filter-button", "click", changeItemFilter);
    addEventListenerToClass("mh-armor-filter-button", "click", changeArmorFilter);
    addEventListenerToClass("mh-skill-filter-button", "click", changeSkillFilter);
    addEventListenerToClass("mh-main-filter-button", "click", changeMainFilter);
    addEventListenerToClass("mh-scombo", "change", onChangeSort);

    addEventListenerToId("combo-weapon-tree", "change", onChangeWeaponTree)

    addEventListenerToClass("mh-map-filter-item", "click", onShowMapExplain);
    addEventListenerToClass("mh-map-filter-button", "click", changeMapFilter);
    addEventListenerToId("button-scale-down", "click", scaleDownMap);
    addEventListenerToId("button-scale-up", "click", scaleUpMap);
    addEventListenerToId("button-map-layer", "click", switchMapLayer);
    addEventListenerToId("mh-map-container", "mousedown", startDragMap);
    addEventListenerToClass("undraggable", "dragstart", undraggable);

    addEventListenerToId("mh-invalid-part-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-invalid-part', null));
    addEventListenerToId("mh-invalid-meat-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-invalid-meat', null));
    addEventListenerToId("mh-ride-cond-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-ride-cond', 'mh-default-cond'));
    addEventListenerToId("mh-preset-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-no-preset', 'mh-preset'));
    addEventListenerToId("mh-non-target-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-non-target', null));
    addEventListenerToId("mh-quest-detail-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-quest-detail', null));
    addEventListenerToId("mh-part-internal-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-part-internal', null));
    addEventListenerToId("mh-hitzone-internal-check", "click",
        e => onCheckDisplay(e.currentTarget, 'mh-hitzone-internal', null));

    addEventListenerToClass("mh-color-diagram-switch", "click", onChangeDiagramColor);
}

function addEventListenerToClass(class_name, event_name, f) {
    for (const element of document.getElementsByClassName(class_name)) {
        element.addEventListener(event_name, f);
    }
}

function addEventListenerToId(id, event_name, f) {
    const element = document.getElementById(id);
    if (element) {
        element.addEventListener(event_name, f);
    }
}

function undraggable(e) {
    e.preventDefault();
    return false;
}

function onToggleLeftAside() {
    const aside = document.getElementById("left-aside");
    const the_other = document.getElementById("right-aside");
    if (aside) {
        aside.classList.toggle("is-active");
        if (the_other && aside.classList.contains("is-active")) {
            the_other.classList.remove("is-active");
        }
    }
}

function onToggleRightAside() {
    const aside = document.getElementById("right-aside");
    const the_other = document.getElementById("left-aside");
    if (aside) {
        aside.classList.toggle("is-active");
        if (the_other && aside.classList.contains("is-active")) {
            the_other.classList.remove("is-active");
        }
    }
}

//function onLeftAsideItem() {
//    const left_aside = document.getElementById("left-aside");
//    if (left_aside) {
//        left_aside.classList.remove("is-active");
//    }
//}

function loadPreference() {
    const language = localStorage.getItem("mh-language");
    if (language !== null && g_supported_mh_lang.includes(language)) {
        g_language_code = language;
    }
}

function delete_all_cookie() {
    const cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        const name = cookie.trim().split("=")[0];
        document.cookie = `${name}=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/`;
    }
}


function adjustVersionMenu() {
    for (const item of document.getElementsByClassName("mh-version-menu")) {
        let href = item.getAttribute("href");
        let hostname = window.location.hostname;
        let current = `https://${hostname}`;
        if (href === current) {
            item.classList.add("has-text-weight-bold");
        }
        item.setAttribute("href", href + window.location.pathname
            + window.location.search
            + window.location.hash);
    }
}

function parse_sort_tag(node) {
    const tag = node.dataset.sort;
    return tag.split(',').map(n => parseInt(n))
}

function onChangeSort(e) {
    const select = e.currentTarget;
    change_sort(removePrefix(select.id, "scombo-"), parseInt(select.value))
}

function onChangeWeaponTree(e) {
    const select = e.currentTarget;
    const root = document.getElementById("mh-weapon-tree");
    if (root === null) {
        return;
    }
    if (select.value === "grid") {
        g_weapon_masonry.destroy();
        root.classList.remove("mh-weapon-tree-list");
        root.classList.add("mh-weapon-tree-grid");
    } else {
        root.classList.remove("mh-weapon-tree-grid");
        root.classList.add("mh-weapon-tree-list");
        initWeaponTreeMasonry();
    }
}

function change_sort(list_name, selecter) {
    const ul = document.getElementById(`slist-${list_name}`);
    if (ul) {
        const new_ul = ul.cloneNode(false);

        const l = [];
        for (const e of ul.childNodes) {
            l.push(e);
        }

        l.sort(function (a, b) {
            const anode = parse_sort_tag(a);
            const bnode = parse_sort_tag(b);
            if (anode[selecter] === bnode[selecter]) {
                return anode[0] - bnode[0];
            } else {
                return anode[selecter] - bnode[selecter];
            }
        });

        for (const e of l) {
            new_ul.appendChild(e);
        }

        ul.parentNode.replaceChild(new_ul, ul);
    }
    const select = document.getElementById(`scombo-${list_name}`);
    if (select) {
        select.value = selecter
    }
}

function refresh_visibility(c) {
    for (const element of document.getElementsByClassName(c)) {
        let matched = false;
        for (const c of g_classes_to_hide) {
            if (element.classList.contains(c)) {
                matched = true;
                break;
            }
        }

        if (matched) {
            element.classList.add("mh-hidden");
        } else {
            element.classList.remove("mh-hidden");
        }
    }
}

function hide_class(c) {
    g_classes_to_hide.add(c);
    refresh_visibility(c);
}

function show_class(c) {
    g_classes_to_hide.delete(c);
    refresh_visibility(c);
}


function selectLanguage(e) {
    const language = removePrefix(e.currentTarget.id, "mh-lang-menu-");
    g_toc = null;
    g_language_code = language;
    switchLanguage();
    localStorage.setItem("mh-language", language)
}

function switchLanguage() {
    document.getElementById("mh-lang-style").innerHTML =
        `.mh-lang:not([lang="${g_language_code}"]) { display:none; }`;

    const title_meta = document.head.querySelector(`meta[itemprop="title-${g_language_code}"]`);
    if (title_meta) {
        document.title = title_meta.content + " - MHRice";
    }

    for (const l of g_supported_mh_lang) {
        const menu_option = document.getElementById(`mh-lang-menu-${l}`);
        if (menu_option) {
            if (l === g_language_code) {
                menu_option.classList.add("has-text-weight-bold");
            } else {
                menu_option.classList.remove("has-text-weight-bold");
            }
        }
    }
}

function onCheckDisplay(checkbox, class_to_show, class_to_hide) {
    if (checkbox.checked) {
        show_class(class_to_show)
        if (class_to_hide != null) {
            hide_class(class_to_hide)
        }
    } else {
        hide_class(class_to_show)
        if (class_to_hide != null) {
            show_class(class_to_hide)
        }
    }
}

function onToggleNavbarMenu() {
    g_navbar_menu_active = !g_navbar_menu_active;
    if (g_navbar_menu_active) {
        document.getElementById("navbarBurger").classList.add("is-active");
        document.getElementById("navbarMenu").classList.add("is-active");

        const left = document.getElementById("right-aside");
        const right = document.getElementById("left-aside");
        if (left) {
            left.classList.remove("is-active");
        }
        if (right) {
            right.classList.remove("is-active");
        }
    } else {
        document.getElementById("navbarBurger").classList.remove("is-active");
        document.getElementById("navbarMenu").classList.remove("is-active");
    }
}

function onShowMapExplain(e) {
    const id = removePrefix(e.currentTarget.id, "mh-map-icon-");
    if (g_cur_map_explain !== null) {
        document.getElementById(`mh-map-explain-${g_cur_map_explain}`).classList.add("mh-hidden");
        if (g_cur_map_explain !== "default") {
            document.getElementById(`mh-map-icon-${g_cur_map_explain}`).classList.remove("mh-map-select");
        }
    }
    g_cur_map_explain = id;
    document.getElementById(`mh-map-explain-${g_cur_map_explain}`).classList.remove("mh-hidden");
    document.getElementById(`mh-map-icon-${g_cur_map_explain}`).classList.add("mh-map-select");
}

function updateMapScale() {
    const map = document.getElementById("mh-map");
    map.style.width = `${g_map_scale}%`;
    map.style.paddingTop = `${g_map_scale}%`;
}

function scaleUpMap() {
    if (g_map_scale >= 500) {
        return
    }

    g_map_scale += 50;

    document.getElementById("button-scale-down").disabled = false;
    if (g_map_scale >= 500) {
        document.getElementById("button-scale-up").disabled = true;
    }

    updateMapScale()
}

function scaleDownMap() {
    if (g_map_scale <= 100) {
        return
    }

    g_map_scale -= 50

    document.getElementById("button-scale-up").disabled = false;
    if (g_map_scale <= 100) {
        document.getElementById("button-scale-down").disabled = true;
    }

    updateMapScale()
}

function switchMapLayer() {
    const prev = document.getElementById(`mh-map-layer-${g_map_layer}`);
    g_map_layer += 1;
    let cur = document.getElementById(`mh-map-layer-${g_map_layer}`);
    if (cur === null) {
        g_map_layer = 0;
        cur = document.getElementById(`mh-map-layer-${g_map_layer}`);
    }
    prev.classList.add("mh-hidden");
    cur.classList.remove("mh-hidden");
}

function changeMapFilter(e) {
    changeFilter(e, 'map');
}

function changeItemFilter(e) {
    changeFilter(e, 'item');
}

function changeArmorFilter(e) {
    changeFilter(e, 'armor');
}

function changeSkillFilter(e) {
    changeFilter(e, 'skill');
}

function changeMainFilter(e) {
    changeFilter(e, 'main');
}

function filterStyle(category, filter) {
    return `.mh-${category}-filter-item:not([data-filter*="${filter}"]) { display:none !important; }`;
}

function initFilter(category) {
    if (location.hash === null || !location.hash.startsWith("#")) {
        return;
    }
    const style = document.getElementById(`mh-${category}-list-style`);
    const filter = removePrefix(location.hash, "#");
    if (style && filter !== "") {
        style.innerHTML = filterStyle(category, filter);
        const filter_button_prefix = `mh-${category}-filter-button-`;
        const prev = document.getElementById(filter_button_prefix + g_filter);
        if (prev !== null) {
            prev.classList.remove("is-active")
        }

        g_filter = filter;

        const cur = document.getElementById(filter_button_prefix + g_filter);
        if (cur !== null) {
            cur.classList.add("is-active")
        }
    }
}

function changeFilter(e, category) {
    let filter = removePrefix(e.currentTarget.id, `mh-${category}-filter-button-`);
    const style = document.getElementById(`mh-${category}-list-style`);
    var hash = "#";
    if (style) {
        if (filter == "all") {
            style.innerHTML = "";
        } else {
            style.innerHTML = filterStyle(category, filter);
            hash = `#${filter}`;
        }
    }

    history.replaceState(null, null, hash);

    const filter_button_prefix = `mh-${category}-filter-button-`;
    const prev = document.getElementById(filter_button_prefix + g_filter);
    if (prev !== null) {
        prev.classList.remove("is-active")
    }

    g_filter = filter;

    const cur = document.getElementById(filter_button_prefix + g_filter);
    if (cur !== null) {
        cur.classList.add("is-active")
    }
}

function doSearch() {
    const text = document.getElementById("mh-search").value.trim();
    if (text.length === 0) {
        return;
    }
    const matchers = text.split(' ').filter(m => m.length > 0);

    let results = [];
    for (const entry of g_toc) {
        let score = Number.MIN_VALUE;
        let best_title = null;
        for (const title of entry.title) {
            let matched = 0;
            let matched_length = 0;
            for (const matcher of matchers) {
                if (title.toLowerCase().includes(matcher.toLowerCase())) {
                    matched += 1;
                    matched_length += matcher.length;
                }
            }
            if (matched === 0) {
                continue;
            }

            const new_score = matched * 10 - (title.length - matched_length);
            if (new_score > score) {
                score = new_score;
                best_title = title;
            }
        }

        if (best_title === null) {
            continue
        }

        const result = { score, title: best_title, path: entry.path };
        results.push(result);
    }

    results.sort((a, b) => b.score - a.score);
    results = results.slice(0, 100);

    const ul = document.getElementById("mh-search-result");
    ul.replaceChildren();

    for (const result of results) {
        const link = document.createElement("a")
        link.setAttribute("href", result.path);
        link.appendChild(document.createTextNode(result.title));

        let tag = "";
        if (result.path.includes("monster")) {
            tag = "Monster";
        } else if (result.path.includes("armor")) {
            tag = "Armor";
        } else if (result.path.includes("hyakuryu_skill")) {
            tag = "Rampage skill";
        } else if (result.path.includes("skill")) {
            tag = "Skill";
        } else if (result.path.includes("item")) {
            tag = "Item";
        } else if (result.path.includes("map")) {
            tag = "Map";
        } else if (result.path.includes("quest")) {
            tag = "Quest";
        } else if (result.path.includes("weapon")) {
            tag = "Weapon";
        } else if (result.path.includes("otomo")) {
            tag = "Buddy";
        } else if (result.path.includes("dlc")) {
            tag = "DLC";
        }

        const li = document.createElement("li");
        if (tag !== "") {
            const tagElement = document.createElement("span");
            tagElement.setAttribute("class", "tag");
            tagElement.appendChild(document.createTextNode(tag));
            li.appendChild(tagElement);
        }
        li.appendChild(link);
        ul.appendChild(li);
    }

    if (results.length === 0) {
        const li = document.createElement("li");
        li.appendChild(document.createTextNode("No result."));
        ul.appendChild(li);
    }

}

function search(e) {
    if (e.key !== 'Enter') {
        return;
    }

    loadTocAndDoSearch();
}

function loadTocAndDoSearch() {
    if (g_toc === null) {
        fetch(`/tocv2/${g_language_code}.json`)
            .then(response => response.json())
            .then(json => {
                g_toc = json;
                doSearch();
            })
    } else {
        doSearch();
    }
}

function goSearch(e) {
    if (e.key !== 'Enter') {
        return;
    }

    const text = document.getElementById("nav-search").value.trim();

    window.location.assign(`/index.html?search=${encodeURIComponent(text)}`)
}


function autoSearch() {
    const param = new URLSearchParams(window.location.search);
    const text = param.get("search");
    if (text === null || text === "") {
        return;
    }

    const searchBox = document.getElementById("mh-search");
    if (searchBox === null) {
        return;
    }
    searchBox.value = text;
    loadTocAndDoSearch();
}

function startDragMap(e) {
    const container = e.currentTarget;
    g_map_pos = {
        // The current scroll
        left: container.scrollLeft,
        top: container.scrollTop,
        // Get the current mouse position
        x: e.clientX,
        y: e.clientY,
        container
    };

    document.addEventListener('mousemove', dragMap);
    document.addEventListener('mouseup', stopDragMap);
}

function dragMap(e) {
    // How far the mouse has been moved
    const dx = e.clientX - g_map_pos.x;
    const dy = e.clientY - g_map_pos.y;

    // Scroll the element
    g_map_pos.container.scrollTop = g_map_pos.top - dy;
    g_map_pos.container.scrollLeft = g_map_pos.left - dx;
}

function stopDragMap() {
    document.removeEventListener('mousemove', dragMap);
    document.removeEventListener('mouseup', stopDragMap);
}

function removePrefix(s, prefix) {
    if (!s.startsWith(prefix)) {
        console.error(`String "${s}" doesn't have prefix "${prefix}"`);
        return null;
    }
    return s.slice(prefix.length);
}

function parseColor(color) {
    const r = parseInt(color.slice(1, 3), 16);
    const g = parseInt(color.slice(3, 5), 16);
    const b = parseInt(color.slice(5, 7), 16);
    return { r, g, b };
}

function onChangeDiagramColor(e) {
    const colorButton = e.currentTarget;
    const id = colorButton.id;
    const diagram_name = colorButton.getAttribute("data-diagram");
    const img = document.getElementById(diagram_name + "-img");
    const canvas = document.getElementById(diagram_name + "-canvas");
    canvas.width = img.clientWidth;
    canvas.height = img.clientHeight;
    const context = canvas.getContext('2d');

    const previous_id = g_diagram_current.get(diagram_name);
    if (previous_id === id) {
        g_diagram_current.delete(diagram_name);
        context.clearRect(0, 0, canvas.width, canvas.height);
        colorButton.classList.remove("mh-active");
        return;
    }

    if (previous_id != null) {
        const previousButton = document.getElementById(previous_id);
        previousButton.classList.remove("mh-active");
    }
    colorButton.classList.add("mh-active");
    g_diagram_current.set(diagram_name, id);

    const imageDataTemp = g_diagram_template.get(img.id);
    const imageData = new ImageData(
        new Uint8ClampedArray(imageDataTemp.data),
        imageDataTemp.width,
        imageDataTemp.height
    )
    const data = imageData.data;

    if (id === "mh-part-dt-extract") {
        const colors = [];
        for (const colorButton of document.getElementsByClassName("mh-extractive-color")) {
            const color = colorButton.getAttribute("data-color");
            const { r, g, b } = parseColor(color);
            const extract = colorButton.getAttribute("data-extractcolor");
            colors.push({ r, g, b, extract });
        }
        for (let i = 0; i < data.length; i += 4) {
            if (data[i + 0] === 0 && data[i + 1] === 0 && data[i + 2] === 0) {
                continue;
            }
            if (data[i + 0] === 255 && data[i + 1] === 255 && data[i + 2] === 255) {
                data[i + 0] = 0;
                data[i + 1] = 0;
                data[i + 2] = 0;
                continue;
            }
            for (const color of colors) {
                const { r, g, b, extract } = color;
                const diff = Math.abs(data[i] - r) + Math.abs(data[i + 1] - g) + Math.abs(data[i + 2] - b);
                if (diff <= 5) {
                    if (extract === "red") {
                        data[i + 0] = 255;
                        data[i + 1] = 0;
                        data[i + 2] = 0;
                    } else if (extract === "white") {
                        data[i + 0] = data[i + 1] = data[i + 2] = 255;
                    } else if (extract === "orange") {
                        data[i + 0] = 255;
                        data[i + 1] = 165;
                        data[i + 2] = 0;
                    } else {
                        data[i + 0] = data[i + 1] = data[i + 2] = 20;
                    }
                    break;
                }
            }
        }
    } else {
        const color = colorButton.getAttribute("data-color");
        const { r, g, b } = parseColor(color)
        for (let i = 0; i < data.length; i += 4) {
            if (data[i + 0] === 0 && data[i + 1] === 0 && data[i + 2] === 0) {
                continue;
            }
            if (data[i + 0] === 255 && data[i + 1] === 255 && data[i + 2] === 255) {
                continue;
            }
            const diff = Math.abs(data[i] - r) + Math.abs(data[i + 1] - g) + Math.abs(data[i + 2] - b);
            if (diff > 5) {
                data[i + 0] = data[i + 1] = data[i + 2] = 230;
            }
        }
    }



    createImageBitmap(imageData).then((bitmap) => {
        context.drawImage(bitmap, 0, 0, imageData.width, imageData.height, 0, 0, canvas.width, canvas.height)
    });
}
