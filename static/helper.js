function play_countdown() {
    var audio = document.getElementById('countdown');
    if(audio != null) {
        var audiosource = document.getElementById('playerSource');
        if(audiosource != null) {
            audio.play();
        }
    }
}