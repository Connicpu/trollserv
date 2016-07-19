function pingTrollCount() {
    var xhr = new XMLHttpRequest();
    xhr.addEventListener("load", function() {
        console.log(this.responseText);
    });
    xhr.open("GET", "/count.txt");
    xhr.send();
}

function fixTroll() {
    var width = window.innerWidth || window.clientWidth;
    var height = window.innerHeight || window.clientHeight;

    var trollman = document.getElementById("trollman");
    if (width < height) {
        trollman.style.width = width - 50;
        trollman.style.height = width - 50;
    } else {
        trollman.style.width = height - 50;
        trollman.style.height = height - 50;
    }

    var container = document.getElementById("troll-container");
    container.style.width = trollman.style.width;
}

function onReady() {
    fixTroll();
    window.addEventListener("resize", fixTroll);
    setInterval(pingTrollCount, 5000);
    pingTrollCount();
}

document.addEventListener("DOMContentLoaded", onReady);
