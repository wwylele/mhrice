var language_index = 1;

var navbar_menu_active = false;

window.onload = function () {
    switchLanguage();
}

function selectLanguage(language) {
    language_index = language;
    switchLanguage();
}

function switchLanguage() {
    for (var i = 0; i < 32; ++i) {
        var c = "mh-lang-" + i;
        for (element of document.getElementsByClassName(c)) {
            if (i === language_index) {
                element.classList.remove("mh-hidden")
            } else {
                element.classList.add("mh-hidden")
            }
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

function onCheckDisplay(checkbox, class_to_display, class_to_hide) {
    for (element of document.getElementsByClassName(class_to_display)) {
        if (checkbox.checked) {
            element.classList.remove("mh-hidden")
        } else {
            element.classList.add("mh-hidden")
        }
    }

    for (element of document.getElementsByClassName(class_to_hide)) {
        if (!checkbox.checked) {
            element.classList.remove("mh-hidden")
        } else {
            element.classList.add("mh-hidden")
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
