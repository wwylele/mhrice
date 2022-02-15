var language_index = 1;

var navbar_menu_active = false;

var classes_to_hide = new Set();

window.onload = function () {
    switchLanguage();
    hide_class("mh-ride-cond");
    hide_class("mh-invalid-meat");
    hide_class("mh-invalid-part");
    hide_class("mh-no-preset");

    change_sort("monster", 1);
    change_sort("item", 1);
    change_sort("armor", 1);
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
        for (e of ul.childNodes) {
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

        for (e of l) {
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
    for (element of document.getElementsByClassName(c)) {
        matched = false;
        for (let c of classes_to_hide) {
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
    language_index = language;
    switchLanguage();
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
        for (element of document.getElementsByClassName(c)) {
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
