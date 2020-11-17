function play_countdown(aid, sid) {
    var audio = document.getElementById(aid);
    if(audio != null) {
        var audiosource = document.getElementById(sid);
        if(audiosource != null) {
            audio.load()
            audio.play();
        }
    }
}