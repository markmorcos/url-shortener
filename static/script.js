const urlForm = document.getElementById("urlForm");
const urlInput = document.getElementById("urlInput");
const shortenBtn = document.getElementById("shortenBtn");
const loading = document.getElementById("loading");
const error = document.getElementById("error");
const result = document.getElementById("result");
const shortUrl = document.getElementById("shortUrl");
const copyBtn = document.getElementById("copyBtn");
const currentYear = document.getElementById("currentYear");
const today = new Date();
currentYear.textContent = today.getFullYear();

urlForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  const url = urlInput.value.trim();

  if (!url) {
    showError("Please enter a URL");
    return;
  }

  try {
    new URL(url);
  } catch {
    showError("Please enter a valid URL (must include http:// or https://)");
    return;
  }

  hideError();
  hideResult();
  showLoading();
  disableForm();

  try {
    const response = await fetch("/api", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ url }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    shortUrl.href = data.short;
    shortUrl.textContent = data.short;

    hideLoading();
    showResult();
  } catch (err) {
    hideLoading();
    showError("Failed to shorten URL. Please try again.");
    console.error("Error:", err);
  } finally {
    enableForm();
  }
});

function showLoading() {
  loading.classList.add("show");
}

function hideLoading() {
  loading.classList.remove("show");
}

function showError(message) {
  error.textContent = message;
  error.classList.add("show");
}

function hideError() {
  error.classList.remove("show");
}

function showResult() {
  result.classList.add("show");
}

function hideResult() {
  result.classList.remove("show");
}

function disableForm() {
  urlInput.disabled = true;
  shortenBtn.disabled = true;
}

function enableForm() {
  urlInput.disabled = false;
  shortenBtn.disabled = false;
}

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(shortUrl.href);
    const originalText = copyBtn.textContent;
    copyBtn.textContent = "Copied!";
    copyBtn.style.background = "#28a745";
    setTimeout(() => {
      copyBtn.textContent = originalText;
      copyBtn.style.background = "";
    }, 2000);
  } catch {
    const textArea = document.createElement("textarea");
    textArea.value = shortUrl.href;
    document.body.appendChild(textArea);
    textArea.select();
    document.execCommand("copy");
    document.body.removeChild(textArea);
    const originalText = copyBtn.textContent;
    copyBtn.textContent = "Copied!";
    setTimeout(() => {
      copyBtn.textContent = originalText;
    }, 2000);
  }
}

urlInput.focus();
