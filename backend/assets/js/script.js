// document.addEventListener("htmx:configRequest", function (event) {
//   // Add headers if needed (e.g., authentication headers)
//   event.detail.headers["Content-Type"] = "application/json";
// });

// function updateResults(response) {
//   var resultsContainer = document.getElementById("results-container");
//   resultsContainer.outerHTML = response; // Update the results container with the new content
// }

// htmx.onLoad(function () {
//   console.log("got here, can you see me?");
//   var input = document.getElementById("userInput");
//
//   // Handle sending data along with the input
//   input.addEventListener("keyup", function () {
//     var scriptContainer = document.getElementById("expectedText");
//     var script = scriptContainer.innerText.trim(); // Extract script text
//     var elapsed_time = calculateElapsedTime(); // Implement your elapsed time calculation
//
//     // Send data to the server
//     htmx.ajax("POST", "/api/v2/html/check_script", {
//       user_input: input.value,
//       script: script,
//       elapsed_time: elapsed_time,
//     });
//     // }, updateResults);
//   });
//
//   // Trigger a request to get initial text
//   // htmx.ajax("GET", "/api/v2/html/spawn_script", null, function (response) {
//   //   var textContainer = document.getElementById("text-container");
//   //   textContainer.outerHTML = response;
//   // });
// });

// old
// var inputArea = document.getElementById("userInput");
//
// inputArea.addEventListener("keyup", function (event) {
//   // Prevent the default form submission
//   event.preventDefault();
//   // get vars for request
//   var scriptContainer = document.getElementById("expectedText");
//   var script = scriptContainer.innerText.trim(); // Extract script text
//   var elapsed_time = calculateElapsedTime(); // Implement your elapsed time calculation
//
//   var payload = {
//     user_input: inputArea.value,
//     script: script,
//     elapsed_time: elapsed_time,
//   };
//   // // Send data to the server
//   // htmx.ajax("POST", "/api/v2/html/check_script", {
//   //   user_input: input.value,
//   //   script: script,
//   //   elapsed_time: elapsed_time,
//   // });
//   // Make a POST request with the form data as JSON
//   fetch("/api/v2/html/check_script", {
//     method: "POST",
//     headers: {
//       "Content-Type": "application/json",
//     },
//     body: JSON.stringify(payload),
//   })
//     .then((response) => {
//       // Check if the response indicates a success
//       if (response.ok) {
//         console.log("check successful");
//       } else {
//         // Handle other responses if needed
//         console.log("check unsuccessful");
//         console.log(response);
//       }
//     })
//     .catch((error) => {
//       // Handle errors here
//       console.error("Error:", error);
//     });
// });
//
// function calculateElapsedTime() {
//   if (!calculateElapsedTime.startTime) {
//     // Set the start time if it's not set
//     calculateElapsedTime.startTime = performance.now();
//   }
//
//   // Calculate elapsed time in seconds
//   var elapsedSeconds = (performance.now() - calculateElapsedTime.startTime) /
//     1000;
//
//   return elapsedSeconds.toFixed(2); // Return elapsed time rounded to two decimal places
// }

// new
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
