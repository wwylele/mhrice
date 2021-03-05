var language_index = 1;

var navbar_menu_active = false;

var classes_to_hide = new Set();

window.onload = function () {
    switchLanguage();
    hide_class("mh-ride-cond");
    hide_class("mh-invalid-meat");
    hide_class("mh-invalid-part");
    hide_class("mh-no-preset");
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
