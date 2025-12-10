// HALO Meta‑Oracle + Co‑op + Supporter JS
// This script defines the core Labyrinth oracle logic and plugs in
// optional supporter (Ko‑Fi) and cooperative session features.

// === Meta‑Oracle Definitions ===
// We define a handful of symbolic categories for the oracle.  These lists
// are deliberately compact but evocative; they can be expanded or
// modified to suit your mythology.  Each entry contains a name and a
// brief tagline.
const AXES = [
  { name: "Mind & Narrative", tagline: "Rewrite your story" },
  { name: "Domain & Magic", tagline: "Shape your reality" },
  { name: "Body & Elemental", tagline: "Honor the vessel" },
  { name: "Spirit & Communion", tagline: "Call your allies" },
  { name: "Fate & Unknown", tagline: "Embrace mystery" }
];

const VECTORS = [
  { name: "Observe", tagline: "Watch and wait" },
  { name: "Release", tagline: "Let go and clear" },
  { name: "Transmute", tagline: "Change and evolve" },
  { name: "Illuminate", tagline: "Reveal and understand" },
  { name: "Manifest", tagline: "Bring it into being" }
];

const TIMELINES = [
  { name: "Now–1 year", tagline: "Immediate/short term" },
  { name: "1–3 years", tagline: "Short term" },
  { name: "3–7 years", tagline: "Medium term" },
  { name: "7–20 years", tagline: "Long term" }
];

const ARCHETYPES = [
  { name: "The Weaver (18)", tagline: "Fates and patterns" },
  { name: "The Gatekeeper (4)", tagline: "Thresholds and choices" },
  { name: "The Fool (0)", tagline: "Beginner's mind" },
  { name: "The Magician (1)", tagline: "Will and manifestation" },
  { name: "The Empress (3)", tagline: "Fertility and nurture" },
  { name: "The Hermit (9)", tagline: "Inner search" },
  { name: "The Tower (16)", tagline: "Sudden change" },
  { name: "The Star (17)", tagline: "Hope and renewal" },
  { name: "The Sun (19)", tagline: "Clarity and vitality" }
];

// Storage keys for localStorage.  These can be namespaced to avoid
// colliding with other apps on the same domain.
const PROFILE_KEY = "halo_profile";
const HISTORY_KEY = "halo_history";
const GAME_KEY = "halo_game_state";
const SUPPORTER_KEY = "halo_supporter";
const ENTITLEMENTS_KEY = "halo_entitlements";

// Game-mode data
const ROOM_TYPES = [
  { name: "Encounter", reward: [3, 7], note: "Face an archetype; negotiate your cost." },
  { name: "Boon", reward: [4, 8], note: "A gift arrives when you name your need." },
  { name: "Trial", reward: [3, 6], note: "Solve a puzzle; gain clarity coins." },
  { name: "Mirror", reward: [2, 6], note: "Reflect and re-align; slower, but safer." },
  { name: "Threshold", reward: [5, 9], note: "Geometry shifts; bank extra if you commit." }
];

const OMEN_BONUSES = [
  { label: "4:44", effect: "Path confirmation", bonus: 3 },
  { label: "11:11", effect: "Doorframe fork", bonus: 5 },
  { label: "906", effect: "Jackpot convergence", bonus: 9 }
];

// Utility: Roll a die with a given number of sides.
function roll(sides) {
  return Math.floor(Math.random() * sides);
}

// Create a new reading based off the defined lists.  A reading bundles
// four random selections along with the user's question and a timestamp.
function createReading(question) {
  const axis = AXES[roll(AXES.length)];
  const vector = VECTORS[roll(VECTORS.length)];
  const timeline = TIMELINES[roll(TIMELINES.length)];
  const archetype = ARCHETYPES[roll(ARCHETYPES.length)];
  return {
    axis,
    vector,
    timeline,
    archetype,
    question: question || "",
    timestamp: new Date().toISOString()
  };
}

// Load the saved profile from localStorage and populate the form fields.
function loadProfile() {
  try {
    const raw = localStorage.getItem(PROFILE_KEY);
    if (!raw) return;
    const profile = JSON.parse(raw);
    const nameInput = document.getElementById("profile-name");
    const sunInput = document.getElementById("profile-sun");
    const moonInput = document.getElementById("profile-moon");
    const risingInput = document.getElementById("profile-rising");
    if (nameInput) nameInput.value = profile.name || "";
    if (sunInput) sunInput.value = profile.sun || "";
    if (moonInput) moonInput.value = profile.moon || "";
    if (risingInput) risingInput.value = profile.rising || "";
  } catch (err) {
    console.warn("Failed to load profile", err);
  }
}

// Save the profile to localStorage.  This persists only on the client.
function saveProfile() {
  const name = document.getElementById("profile-name").value.trim();
  const sun = document.getElementById("profile-sun").value.trim();
  const moon = document.getElementById("profile-moon").value.trim();
  const rising = document.getElementById("profile-rising").value.trim();
  const profile = { name, sun, moon, rising };
  localStorage.setItem(PROFILE_KEY, JSON.stringify(profile));
}

// Load reading history from storage.  Returns an array (possibly empty).
function loadHistory() {
  try {
    const raw = localStorage.getItem(HISTORY_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch (err) {
    console.warn("Failed to load history", err);
    return [];
  }
}

// Save reading history back to storage.
function saveHistory(history) {
  localStorage.setItem(HISTORY_KEY, JSON.stringify(history));
}

// === Game State Helpers ===
function defaultGameState() {
  return {
    coins: 0,
    runs: 0,
    bestRooms: 0,
    dailyTarget: null,
    dailyStreak: 0,
    lastDaily: null
  };
}

function defaultEntitlements() {
  return {
    booster: false,
    adBonusDate: null
  };
}

function loadGameState() {
  try {
    const raw = localStorage.getItem(GAME_KEY);
    if (!raw) return defaultGameState();
    const parsed = JSON.parse(raw);
    return { ...defaultGameState(), ...parsed };
  } catch (err) {
    console.warn("Failed to load game state", err);
    return defaultGameState();
  }
}

function saveGameState(state) {
  localStorage.setItem(GAME_KEY, JSON.stringify(state));
}

function loadEntitlements() {
  try {
    const raw = localStorage.getItem(ENTITLEMENTS_KEY);
    if (!raw) return defaultEntitlements();
    const parsed = JSON.parse(raw);
    return { ...defaultEntitlements(), ...parsed };
  } catch (err) {
    console.warn("Failed to load entitlements", err);
    return defaultEntitlements();
  }
}

function saveEntitlements(state) {
  localStorage.setItem(ENTITLEMENTS_KEY, JSON.stringify(state));
}

// Render the current reading into the DOM.  If no reading is provided,
// clears the current display.
function renderCurrent(reading) {
  const container = document.getElementById("current-reading");
  if (!container) return;
  if (!reading) {
    container.innerHTML = "";
    return;
  }
  // Format the timestamp into a human‑readable string.
  const ts = new Date(reading.timestamp);
  const tsStr = ts.toLocaleString();
  container.innerHTML = `
    <div class="reading-result">
      <p><strong>When:</strong> <span class="timestamp">${tsStr}</span></p>
      <p><strong>Axis:</strong> ${reading.axis.name}</p>
      <p><strong>Vector:</strong> ${reading.vector.name}</p>
      <p><strong>Timeline:</strong> ${reading.timeline.name}</p>
      <p><strong>Archetype:</strong> ${reading.archetype.name}</p>
      ${reading.question ? `<p><strong>Q:</strong> ${reading.question}</p>` : ""}
    </div>
  `;
}

// Render the history table.  Each entry shows the timestamp and top‑level
// categories.  You could expand this to include question or notes.
function renderHistory(history) {
  const container = document.getElementById("history");
  if (!container) return;
  if (!history || history.length === 0) {
    container.innerHTML = "<p class='muted'>No past readings yet.</p>";
    return;
  }
  // Build a simple HTML table.
  let html = "<table><thead><tr><th>When</th><th>Axis</th><th>Vector</th><th>Timeline</th><th>Archetype</th></tr></thead><tbody>";
  history.forEach((r) => {
    const ts = new Date(r.timestamp).toLocaleString();
    html += `<tr><td>${ts}</td><td>${r.axis.name}</td><td>${r.vector.name}</td><td>${r.timeline.name}</td><td>${r.archetype.name}</td></tr>`;
  });
  html += "</tbody></table>";
  container.innerHTML = html;
}

// === Labyrinth Run Rendering ===
let gameState = defaultGameState();
let entitlements = defaultEntitlements();
let currentRun = null;

function renderGameState() {
  const coinsEl = document.getElementById("banked-coins");
  const runsEl = document.getElementById("runs-cleared");
  const bestEl = document.getElementById("best-run");
  const dailyEl = document.getElementById("daily-target");
  const streakEl = document.getElementById("daily-streak");
  if (coinsEl) coinsEl.textContent = `${gameState.coins} ◎`;
  if (runsEl) runsEl.textContent = gameState.runs;
  if (bestEl) bestEl.textContent = `${gameState.bestRooms} rooms`;
  if (dailyEl)
    dailyEl.textContent = gameState.dailyTarget ? `${gameState.dailyTarget} rooms` : "–";
  if (streakEl) streakEl.textContent = `${gameState.dailyStreak} days`;
}

function renderRunStatus(message) {
  const status = document.getElementById("run-status");
  if (status) {
    status.innerHTML = message || "No active run.";
  }
}

function appendRunLog(line) {
  const log = document.getElementById("run-log");
  if (!log) return;
  const now = new Date().toLocaleTimeString();
  const existing = log.innerHTML || "";
  log.innerHTML = `<div>[${now}] ${line}</div>` + existing;
}

function renderMonetization() {
  const status = document.getElementById("monetization-status");
  if (!status) return;
  const today = new Date().toISOString().slice(0, 10);
  const adReady = entitlements.adBonusDate !== today;
  const supporterStatus = supporterActive()
    ? "Supporter Mode: active (Ko‑Fi badge)"
    : "Supporter Mode: locked (tip to unlock)";
  const boosterStatus = entitlements.booster
    ? "Sigil Booster: active (+1 ◎ per room)"
    : "Sigil Booster: not purchased";
  const adStatus = adReady ? "Daily ad bonus available" : "Ad bonus claimed today";
  status.innerHTML = `
    <div>${supporterStatus}</div>
    <div>${boosterStatus}</div>
    <div>${adStatus}</div>
  `;
}

function refreshDailyTarget() {
  const today = new Date().toISOString().slice(0, 10);
  if (gameState.lastDaily === today && gameState.dailyTarget) return;
  const base = new Date().getDate();
  gameState.dailyTarget = Math.max(3, (base % 8) + 3); // 3–10 rooms
  gameState.lastDaily = today;
  saveGameState(gameState);
}

function supporterActive() {
  return localStorage.getItem(SUPPORTER_KEY) === "true";
}

function claimAdBonus() {
  const today = new Date().toISOString().slice(0, 10);
  if (entitlements.adBonusDate === today) {
    appendRunLog("Ad bonus already claimed today.");
    return;
  }
  const bonus = 10;
  entitlements.adBonusDate = today;
  gameState.coins += bonus;
  saveEntitlements(entitlements);
  saveGameState(gameState);
  renderGameState();
  renderMonetization();
  appendRunLog(`Ad reward claimed: +${bonus} ◎ added to bank.`);
}

function purchaseBooster(pack) {
  if (entitlements.booster) {
    appendRunLog("Sigil Booster already active.");
    return;
  }
  const confirmed = confirm(
    "Mock purchase: unlock the Sigil Booster by tipping via Supporter mode? (Adds +15 ◎ now and +1 ◎ per room.)"
  );
  if (!confirmed) return;
  entitlements.booster = true;
  saveEntitlements(entitlements);
  gameState.coins += 15;
  saveGameState(gameState);
  renderMonetization();
  renderGameState();
  appendRunLog("Sigil Booster unlocked: banked +15 ◎ and future rooms gain +1 ◎.");
}

function startRun() {
  refreshDailyTarget();
  if (currentRun) {
    appendRunLog("Run already active—clear the current room first.");
    return;
  }
  currentRun = {
    rooms: 0,
    coins: 0,
    omen: null
  };
  document.getElementById("next-room")?.style.setProperty("display", "inline-block");
  document.getElementById("end-run")?.style.setProperty("display", "inline-block");
  appendRunLog("Run opened. Choose your rooms wisely.");
  renderRunStatus("Run active: step into the first room.");
}

function rollRoomReward(range) {
  const [min, max] = range;
  return min + Math.floor(Math.random() * (max - min + 1));
}

function maybeOmenBonus() {
  // ~35% chance to trigger an omen event per room
  if (Math.random() > 0.35) return null;
  return OMEN_BONUSES[roll(OMEN_BONUSES.length)];
}

function endRun(reason) {
  if (!currentRun) return;
  gameState.coins += currentRun.coins;
  gameState.runs += 1;
  gameState.bestRooms = Math.max(gameState.bestRooms, currentRun.rooms);
  // Daily streak bonus
  if (currentRun.rooms >= gameState.dailyTarget) {
    gameState.dailyStreak += 1;
    const streakBonus = 5 + gameState.dailyStreak;
    gameState.coins += streakBonus;
    appendRunLog(`Daily streak advanced (+${streakBonus} ◎).`);
  }
  saveGameState(gameState);
  renderGameState();

  appendRunLog(
    `Run closed after ${currentRun.rooms} room(s); banked ${currentRun.coins} ◎. ${reason || ""}`
  );
  renderRunStatus("No active run.");
  document.getElementById("next-room")?.style.setProperty("display", "none");
  document.getElementById("end-run")?.style.setProperty("display", "none");
  currentRun = null;
}

function stepRoom() {
  if (!currentRun) {
    appendRunLog("Start a run first.");
    return;
  }
  const room = ROOM_TYPES[roll(ROOM_TYPES.length)];
  const baseReward = rollRoomReward(room.reward);
  const supporterBoost = supporterActive() ? 2 : 0;
  const boosterBoost = entitlements.booster ? 1 : 0;
  const omen = maybeOmenBonus();
  let totalReward = baseReward + supporterBoost + boosterBoost;
  let closure = false;

  if (omen) {
    totalReward += omen.bonus;
    // 906 closes the loop early as a jackpot
    if (omen.label === "906" && Math.random() < 0.5) {
      closure = true;
    }
  }

  currentRun.rooms += 1;
  currentRun.coins += totalReward;

  const omenText = omen
    ? ` | Omen ${omen.label} (${omen.effect}) +${omen.bonus} ◎`
    : "";
  const supporterText = supporterBoost ? " | Supporter bonus +2 ◎" : "";
  const boosterText = boosterBoost ? " | Booster +1 ◎" : "";

  renderRunStatus(
    `Room ${currentRun.rooms}: ${room.name} → ${room.note}<br/>Reward: ${totalReward} ◎${omenText}${supporterText}${boosterText}`
  );
  appendRunLog(
    `${room.name} cleared for ${totalReward} ◎.${omenText}${supporterText}${boosterText}`
  );

  // Small chance the labyrinth decides you've had enough
  if (closure || Math.random() < 0.15) {
    endRun("The labyrinth seals behind you—bank what you earned.");
  }
}

// Main initialization: wire up event handlers and load any saved data.
function init() {
  // Load profile and history on page load.
  loadProfile();
  let history = loadHistory();
  renderHistory(history);

  // Profile save button.
  const saveBtn = document.getElementById("save-profile");
  if (saveBtn) {
    saveBtn.addEventListener("click", () => {
      saveProfile();
      alert("Profile saved locally");
    });
  }
  // Roll button: generate a reading and update the UI and history.
  const rollBtn = document.getElementById("roll-btn");
  if (rollBtn) {
    rollBtn.addEventListener("click", () => {
      const question = document.getElementById("question").value.trim();
      const reading = createReading(question);
      // Unshift reading (add to beginning) to show latest first.
      history.unshift(reading);
      saveHistory(history);
      renderCurrent(reading);
      renderHistory(history);
    });
  }
}

// === Supporter and Co‑op Features ===

// Supporter functionality: allow users to tip via Ko‑Fi and unlock a badge.
function setupSupport() {
  const supportButton = document.getElementById("support-button");
  const supportBadge = document.getElementById("support-badge");
  if (!supportButton || !supportBadge) {
    return;
  }
  // Display supporter badge if previously activated.
  const isSupporter = supporterActive();
  if (isSupporter) {
    supportBadge.style.display = "inline-flex";
  }
  supportButton.addEventListener("click", () => {
    // Open Ko‑Fi in a new tab.
    window.open("https://ko-fi.com/oneinfinity", "_blank", "noopener");
    // Ask the user if they actually supported. If yes, set supporter flag and show badge.
    const opted = confirm(
      "Thank you for considering support! If you just tipped on Ko‑Fi, click OK to enable supporter mode."
    );
    if (opted) {
      localStorage.setItem(SUPPORTER_KEY, "true");
      supportBadge.style.display = "inline-flex";
      renderGameState();
      renderMonetization();
    }
  });
}

function setupMonetization() {
  entitlements = loadEntitlements();
  renderMonetization();
  const adButton = document.getElementById("ad-bonus");
  if (adButton) {
    adButton.addEventListener("click", claimAdBonus);
  }
  const boosterButtons = document.querySelectorAll(".booster-btn");
  boosterButtons.forEach((btn) => {
    btn.addEventListener("click", () => purchaseBooster(btn.dataset.pack));
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
    // Build a simple payload.  You could include the current reading seed or
    // other game state here.  Using Date.now() as a nonce ensures each
    // session link is unique.  You can later decode this and recreate
    // identical outcomes if your game is deterministic.
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
      // Notify the user they've joined a co‑op session. In a real game, you
      // could apply this seed and timeline to synchronize outcomes.
      console.log("Loaded co‑op session:", data);
      alert(
        "You have joined a shared Labyrinth session! Enjoy this synchronized journey."
      );
    } catch (err) {
      console.error("Failed to parse co‑op session data", err);
    }
  }
}

// Game mode wiring
function setupGame() {
  gameState = loadGameState();
  refreshDailyTarget();
  renderGameState();

  const startBtn = document.getElementById("start-run");
  const nextBtn = document.getElementById("next-room");
  const endBtn = document.getElementById("end-run");
  if (startBtn) startBtn.addEventListener("click", startRun);
  if (nextBtn) nextBtn.addEventListener("click", stepRoom);
  if (endBtn) endBtn.addEventListener("click", () => endRun("You chose to exit."));
}

// When the DOM is ready, wire everything up.  We call init() to load
// profile/history and attach the Meta‑Oracle roll handler, then set up
// supporter/coop after that.  We deliberately initialize meta logic
// first so that any co‑op code can leverage the state if needed.
document.addEventListener("DOMContentLoaded", () => {
  init();
  setupGame();
  setupMonetization();
  setupSupport();
  setupCoop();
});