document.addEventListener("DOMContentLoaded", function () {
  document
    .getElementById("loginForm")
    .addEventListener("submit", function (event) {
      event.preventDefault(); // Prevent the default form submission

      // Get form data
      const formData = new FormData(this);

      // Convert form data to JSON
      const formDataJSON = {};
      formData.forEach(function (value, key) {
        formDataJSON[key] = value;
      });

      // Make a POST request with the form data as JSON
      fetch("/api/v1/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formDataJSON),
      })
        .then((response) => {
          // Check if the response indicates a successful login
          if (response.ok) {
            // Redirect to the home page
            window.location.href = "/";
          } else {
            // Handle other responses if needed
            console.log("Login unsuccessful");
          }
        })
        .catch((error) => {
          // Handle errors here
          console.error("Error:", error);
        });
    });
});
