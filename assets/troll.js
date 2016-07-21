function isMobile() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
}

function pingTrollCount() {
    var xhr = new XMLHttpRequest();
    xhr.addEventListener("load", function() {
        var hits = document.getElementById("hits");
        hits.innerText = this.responseText;
    });
    xhr.open("GET", "/count.txt");
    xhr.send();
}

function mobileTroll() {
    var trollman = document.getElementById("trollman");
    var container = document.getElementById("text-container");
    var handler;

    handler = function() {
        trollman.removeEventListener("click", handler);
        trollman.src = "troll.gif";
        container.style.display = "";
        
        var player = document.getElementById("player");
        player.play();
    }

    trollman.src = "play.png";
    trollman.addEventListener("click", handler);
    container.style.display = "none";
}

function fixTroll(size) {
    var width = size || window.innerWidth || window.clientWidth;
    var height = size || window.innerHeight || window.clientHeight;

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

    if (isMobile()) {
        mobileTroll(true);
    } else {
        var player = document.getElementById("player");
        player.play();
    }
    
    window.addEventListener("resize", function() { fixTroll(); });

    pingTrollCount();
    setInterval(pingTrollCount, 5000);

    startTime = new Date().getTime();
    setInterval(trollTime, 100);
}

document.addEventListener("DOMContentLoaded", onReady);
