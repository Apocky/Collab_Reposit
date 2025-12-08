// HALO JavaScript for the Labyrinth page
// This script provides support for monetization (supporter mode) and cooperative sessions.

document.addEventListener("DOMContentLoaded", function () {
  // Initialize supporter and co‑op features once the DOM is ready.
  setupSupport();
  setupCoop();
});

// Supporter functionality: allow users to tip via Ko‑Fi and unlock a badge.
function setupSupport() {
  const supportButton = document.getElementById("support-button");
  const supportBadge = document.getElementById("support-badge");
  if (!supportButton || !supportBadge) {
    return;
  }
  // Display supporter badge if previously activated.
  const isSupporter = localStorage.getItem("halo_supporter") === "true";
  if (isSupporter) {
    supportBadge.style.display = "inline-flex";
  }
  supportButton.addEventListener("click", () => {
    // Open Ko-Fi in a new tab.
    window.open("https://ko-fi.com/oneinfinity", "_blank", "noopener");
    // Ask the user if they actually supported. If yes, set supporter flag and show badge.
    const opted = confirm(
      "Thank you for considering support! If you just tipped on Ko‑Fi, click OK to enable supporter mode."
    );
    if (opted) {
      localStorage.setItem("halo_supporter", "true");
      supportBadge.style.display = "inline-flex";
    }
  });
}

// Co‑op functionality: generate shareable session codes and handle incoming sessions.
function setupCoop() {
  const startBtn = document.getElementById("start-coop");
  const copyBtn = document.getElementById("copy-coop");
  const linkInput = document.getElementById("coop-link");
  if (!startBtn || !copyBtn || !linkInput) {
    return;
  }
  // Create a co‑op session link with encoded payload when the user clicks the button.
  startBtn.addEventListener("click", () => {
    // Build a simple payload. In a real game, include the state (e.g., RNG seed, moves, etc.).
    const payload = {
      version: 1,
      timestamp: Date.now(),
      seed: Math.floor(Math.random() * 1e9)
    };
    const json = JSON.stringify(payload);
    const encoded = btoa(encodeURIComponent(json));
    const url = new URL(window.location.href);
    url.searchParams.set("coop", encoded);
    linkInput.value = url.toString();
    copyBtn.style.display = "inline-block";
  });
  // Copy the session link to the clipboard when requested.
  copyBtn.addEventListener("click", () => {
    if (!linkInput.value) return;
    navigator.clipboard
      .writeText(linkInput.value)
      .then(() => {
        copyBtn.textContent = "Copied!";
        setTimeout(() => {
          copyBtn.textContent = "Copy Link";
        }, 1500);
      })
      .catch(() => {
        alert("Copy failed. You can copy the link manually.");
      });
  });
  // Check if the current URL contains a co‑op payload. If so, reconstruct the session.
  const currentUrl = new URL(window.location.href);
  const param = currentUrl.searchParams.get("coop");
  if (param) {
    try {
      const json = decodeURIComponent(atob(param));
      const data = JSON.parse(json);
      // Notify the user they've joined a co‑op session. In a real game, apply this state.
      console.log("Loaded co‑op session:", data);
      alert(
        "You have joined a shared Labyrinth session! Enjoy this synchronized journey."
      );
    } catch (err) {
      console.error("Failed to parse co‑op session data", err);
    }
  }
}