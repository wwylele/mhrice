var cookie_consent = false;
var language_index = 1;

var navbar_menu_active = false;

var classes_to_hide = new Set();

var cur_map_explain = "default";

var map_scale = 100
var map_layer = 0

var toc = null;

window.onload = function () {
    check_cookie();
    switchLanguage();
    hide_class("mh-ride-cond");
    hide_class("mh-invalid-meat");
    hide_class("mh-invalid-part");
    hide_class("mh-no-preset");

    change_sort("monster", 1);
    change_sort("item", 1);
    change_sort("armor", 1);
}

function check_cookie() {
    const cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        const s = cookie.trim().split("=");
        cookie_name = s[0];
        cookie_value = s[1];
        if (cookie_name === null || cookie_value === null) {
            continue;
        }
        if (cookie_name === "consent" && cookie_value === "yes") {
            cookie_consent = true;
        }

        if (cookie_name === "language") {
            language_index = parseInt(cookie_value)
            if (!(Number.isInteger(language_index) && language_index >= 0 && language_index < 32)) {
                language_index = 1;
            }
        }
    }

    if (cookie_consent) {
        document.getElementById("cookie-yes").checked = true;
    } else {
        document.getElementById("cookie-no").checked = true;
    }
}

function enableCookie() {
    document.cookie = "consent=yes; path=/";
    cookie_consent = true;
}

function disableCookie() {
    cookie_consent = false;
    delete_all_cookie();
}

function delete_all_cookie() {
    const cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        document.cookie = cookie.trim().split("=")[0] + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/";
    }
}

function parse_sort_tag(node) {
    var tag = node.dataset.sort;
    return tag.split(',').map(n => parseInt(n))
}

function onChangeSort(select) {
    change_sort(select.id.slice(7 /*scombo-*/), parseInt(select.value))
}

function change_sort(list_name, selecter) {
    var ul = document.getElementById("slist-" + list_name);
    if (ul) {
        var new_ul = ul.cloneNode(false);

        var l = [];
        for (const e of ul.childNodes) {
            l.push(e);
        }

        l.sort(function (a, b) {
            anode = parse_sort_tag(a);
            bnode = parse_sort_tag(b);
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
    var select = document.getElementById("scombo-" + list_name);
    if (select) {
        select.value = selecter
    }
}

function refresh_visibility(c) {
    for (const element of document.getElementsByClassName(c)) {
        matched = false;
        for (const c of classes_to_hide) {
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
    classes_to_hide.add(c);
    refresh_visibility(c);
}

function show_class(c) {
    classes_to_hide.delete(c);
    refresh_visibility(c);
}

function selectLanguage(language) {
    toc = null;
    language_index = language;
    switchLanguage();
    if (cookie_consent) {
        document.cookie = "language=" + language_index + "; path=/";
    }
}

function switchLanguage() {
    for (var i = 0; i < 32; ++i) {
        var c = "mh-lang-" + i;
        if (i === language_index) {
            show_class(c);
        } else {
            hide_class(c);
        }

        var c = "mh-lang-menu-" + i;
        for (const element of document.getElementsByClassName(c)) {
            if (i === language_index) {
                element.classList.add("has-text-weight-bold");
            } else {
                element.classList.remove("has-text-weight-bold");
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
    navbar_menu_active = !navbar_menu_active;
    if (navbar_menu_active) {
        document.getElementById("navbarBurger").classList.add("is-active");
        document.getElementById("navbarMenu").classList.add("is-active");
    } else {
        document.getElementById("navbarBurger").classList.remove("is-active");
        document.getElementById("navbarMenu").classList.remove("is-active");
    }
}

function onShowMapExplain(id) {
    if (cur_map_explain !== null) {
        document.getElementById("mh-map-explain-" + cur_map_explain).classList.add("mh-hidden");
        if (cur_map_explain !== "default") {
            document.getElementById("mh-map-icon-" + cur_map_explain).classList.remove("mh-map-select");
        }
    }
    cur_map_explain = id;
    document.getElementById("mh-map-explain-" + cur_map_explain).classList.remove("mh-hidden");
    document.getElementById("mh-map-icon-" + cur_map_explain).classList.add("mh-map-select");
}

function updateMapScale() {
    let map = document.getElementById("mh-map");
    map.style.width = map_scale + "%";
    map.style.paddingTop = map_scale + "%";
}

function scaleUpMap() {
    if (map_scale >= 500) {
        return
    }

    map_scale += 50;

    document.getElementById("button-scale-down").disabled = false;
    if (map_scale >= 500) {
        document.getElementById("button-scale-up").disabled = true;
    }

    updateMapScale()
}

function scaleDownMap() {
    if (map_scale <= 100) {
        return
    }

    map_scale -= 50

    document.getElementById("button-scale-up").disabled = false;
    if (map_scale <= 100) {
        document.getElementById("button-scale-down").disabled = true;
    }

    updateMapScale()
}

function switchMapLayer() {
    let prev = document.getElementById("mh-map-layer-" + map_layer);
    map_layer += 1;
    let cur = document.getElementById("mh-map-layer-" + map_layer);
    if (cur === null) {
        map_layer = 0;
        cur = document.getElementById("mh-map-layer-" + map_layer);
    }
    prev.classList.add("mh-hidden");
    cur.classList.remove("mh-hidden");
}

var map_filter = new Map([
    ["all", (pop) => true],
    ["item", (pop) => pop.classList.contains("mh-map-tag-item")],
    ["relic", (pop) => pop.classList.contains("mh-map-tag-relic")],
    ["camp", (pop) => pop.classList.contains("mh-map-tag-camp")],
    ["jump", (pop) => pop.classList.contains("mh-map-tag-jump")],
    ["fish", (pop) => pop.classList.contains("mh-map-tag-fish")],
]);

var cur_map_filter = "all"

function changeMapFilter(filter) {
    let filter_fun = map_filter.get(filter);
    for (const element of document.getElementsByClassName("mh-map-pop")) {
        if (filter_fun(element)) {
            element.classList.remove("mh-hidden");
        } else {
            element.classList.add("mh-hidden");
        }
    }

    const filter_button_prefix = "mh-map-filter-";
    let prev = document.getElementById(filter_button_prefix + cur_map_filter);
    if (prev !== null) {
        prev.classList.remove("is-primary")
    }

    cur_map_filter = filter;

    let cur = document.getElementById(filter_button_prefix + cur_map_filter);
    if (cur !== null) {
        cur.classList.add("is-primary")
    }
}

function doSearch() {
    let text = document.getElementById("mh-search").value.trim();
    if (text.length === 0) {
        return;
    }
    let matchers = text.split(' ').filter(m => m.length > 0);
    console.log(matchers);

    let results = [];
    for (var entry of toc) {
        let matched = 0;
        let matched_length = 0;
        for (var matcher of matchers) {
            if (entry.title.toLowerCase().includes(matcher.toLowerCase())) {
                matched += 1;
                matched_length += matcher.length;
            }
        }
        if (matched === 0) {
            continue;
        }

        let score = matched * 10 - (entry.title.length - matched_length);
        let result = { score, ...entry };
        results.push(result);
    }

    results = results.sort((a, b) => b.score - a.score);


    let ul = document.getElementById("mh-search-result");
    ul.replaceChildren();

    for (var result of results) {
        let link = document.createElement("a")
        link.setAttribute("href", result.path);
        link.appendChild(document.createTextNode(result.title));

        let tag = "";
        if (result.path.includes("monster")) {
            tag = "Monster";
        } else if (result.path.includes("armor")) {
            tag = "Armor";
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
        }

        let li = document.createElement("li");
        if (tag !== "") {
            let tagElement = document.createElement("span");
            tagElement.setAttribute("class", "tag");
            tagElement.appendChild(document.createTextNode(tag));
            li.appendChild(tagElement);
        }
        li.appendChild(link);
        ul.appendChild(li);
    }

    if (results.length === 0) {
        let li = document.createElement("li");
        li.appendChild(document.createTextNode("No result."));
        ul.appendChild(li);
    }

}

function search() {
    if (window.event.key !== 'Enter') {
        return;
    }

    if (toc === null) {
        fetch("/toc/" + language_index + ".json")
            .then(response => response.json())
            .then(json => {
                toc = json;
                doSearch();
            })
    } else {
        doSearch();
    }
}
