function pingTrollCount() {
    var xhr = new XMLHttpRequest();
    xhr.addEventListener("load", function() {
        var hits = document.getElementById("hits");
        hits.innerText = this.responseText;
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

var startTime = null;

function trollTime() {
    var now = new Date().getTime();
    var ms = now - startTime;
    var trolltime = document.getElementById("trolltime");
    trolltime.innerText = (Math.round(ms / 100) / 10).toString();
}

function onReady() {
    fixTroll();
    window.addEventListener("resize", fixTroll);

    pingTrollCount();
    setInterval(pingTrollCount, 5000);

    startTime = new Date().getTime();
    setInterval(trollTime, 100);
}

document.addEventListener("DOMContentLoaded", onReady);
