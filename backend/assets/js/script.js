function startCountUp() {
  //The initial starting point of the counter is 0
  var secs = 0;
  var mins = 0;
  var hrs = 0;

  document.getElementById("timer").value = "0:0:0";

  var interval = setInterval(function () {
    secs++;
    document.getElementById("timer").value = hrs + ":" + mins + ":" + secs;
    if (secs > 59) {
      mins++;
      secs = 0;
      document.getElementById("timer").innerHTML = hrs + ":" + mins + ":" +
        secs;
    }
    if (mins > 59) {
      mins = 0;
      secs = 0;
      hrs++;
      document.getElementById("timer").innerHTML = hrs + ":" + mins + ":" +
        secs;
    }
    if (hrs > 59) {
      clearInterval(interval);
    }
  }, 1000);
}
