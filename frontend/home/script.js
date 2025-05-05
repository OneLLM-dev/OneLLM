document.getElementById("fetch-btn").addEventListener("click", async () => {
  try {
    const res = await fetch("/api");
    if (!res.ok) throw new Error("Request failed");
    const text = await res.text();
    document.getElementById("response-text").textContent = text;
  } catch (err) {
    document.getElementById("response-text").textContent =
      "Error: " + err.message;
  }
});
