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
