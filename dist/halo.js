// HALO Pocket Labyrinth – mobile-first micro game
// Deterministic, seedable oracle with light resource management.

const STORAGE_KEY = "halo_mobile_state";
let memoryFallback = null;
let warnedStorage = false;

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
  { name: "The Weaver", tagline: "Fates and patterns" },
  { name: "The Gatekeeper", tagline: "Thresholds and choices" },
  { name: "The Fool", tagline: "Beginner's mind" },
  { name: "The Magician", tagline: "Will and manifestation" },
  { name: "The Empress", tagline: "Fertility and nurture" },
  { name: "The Hermit", tagline: "Inner search" },
  { name: "The Tower", tagline: "Sudden change" },
  { name: "The Star", tagline: "Hope and renewal" },
  { name: "The Sun", tagline: "Clarity and vitality" }
];

const SITUATIONS = [
  "A locked door within a shifting hallway",
  "A bargain you thought you understood twists",
  "A mirror shows a version of you you almost forgot",
  "Two timelines overlap; pick one to stabilize",
  "An old ally calls in a favor",
  "A signal appears from deep within the Labyrinth",
  "A beacon flickers; it's both a trap and an invitation",
  "You find a cache of forgotten notes",
  "The path folds; shortcuts reveal hidden costs",
  "A rival steps aside, revealing a deeper threat"
];

const BOONS = [
  "Gain clarity: +1 Momentum",
  "Shield spark: restore 1 Aegis",
  "Shortcut: skip a depth penalty",
  "Companion: reroll one bad beat this run",
  "Archive ping: lock in current seed for sync",
  "Hidden stash: bank your current Momentum",
  "Anchor: reduce Doom by 1",
  "Insight: write one true thing about your quest"
];

const THREATS = [
  "Static surge: lose 1 Aegis",
  "False lead: -1 Momentum",
  "Depth collapse: +1 Doom if Momentum is 0",
  "Shadow bargain: trade 1 Aegis for +2 Momentum",
  "Exhaustion: Momentum cannot exceed 3 until you rest",
  "Glitch: repeat the next beat twice",
  "Echo: your last log entry replays with a darker spin",
  "Lockdown: you must cash out after the next beat"
];

function notify(message) {
  const el = document.getElementById("notice");
  if (!el) return;
  if (!message) {
    el.textContent = "";
    el.classList.remove("show");
    return;
  }
  el.textContent = message;
  el.classList.add("show");
}

function hashSeed(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) - hash + str.charCodeAt(i);
    hash |= 0; // force 32-bit
  }
  return Math.abs(hash) + 1;
}

function random(state) {
  const raw = Math.sin(state.seedHash + state.cursor) * 10000;
  state.cursor += 1;
  return raw - Math.floor(raw);
}

function pick(list, state) {
  const idx = Math.floor(random(state) * list.length);
  return list[idx];
}

function generateSeed() {
  const chars = "ABCDEFGHJKMNPQRSTUVWXYZ23456789";
  let out = "HALO-";
  for (let i = 0; i < 6; i++) {
    out += chars[Math.floor(Math.random() * chars.length)];
  }
  return out;
}

function baseState(overrides = {}) {
  const seed = overrides.seed || generateSeed();
  const seedHash = hashSeed(seed);
  return {
    id: Date.now(),
    player: overrides.player || "", 
    quest: overrides.quest || "",
    mode: overrides.mode || "solo",
    difficulty: overrides.difficulty || "standard",
    momentum: 2,
    aegis: 2,
    depth: 0,
    doom: 0,
    cursor: 0,
    seed,
    seedHash,
    log: [],
    status: "running",
    flags: {}
  };
}

function applyDifficulty(state) {
  if (state.difficulty === "chill") {
    state.momentum = 3;
    state.aegis = 3;
  } else if (state.difficulty === "brutal") {
    state.momentum = 2;
    state.aegis = 1;
    state.doom = 1;
  }
}

function saveState(state) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    memoryFallback = state;
  } catch (err) {
    memoryFallback = state;
    if (!warnedStorage) {
      console.warn("Local storage unavailable; keeping state in memory only", err);
      notify("Local storage is blocked. Progress will reset if you close this tab.");
      warnedStorage = true;
    }
  }
}

function loadState() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw);
  } catch (err) {
    if (!warnedStorage) {
      console.warn("Failed to load state", err);
      notify("Cannot access local storage. Using in-memory state for this session.");
      warnedStorage = true;
    }
    if (memoryFallback) return memoryFallback;
  }
  return memoryFallback;
}

function statusPills(state) {
  const pills = [];
  if (state.mode === "coop") pills.push("Co-op seed active");
  if (state.mode === "gauntlet") pills.push("Gauntlet: doom ticks faster");
  pills.push(`Seed ${state.seed}`);
  pills.push(`Quest: ${state.quest || "open"}`);
  return pills;
}

function renderState(state) {
  document.getElementById("stat-momentum").textContent = state.momentum;
  document.getElementById("stat-aegis").textContent = state.aegis;
  document.getElementById("stat-depth").textContent = state.depth;
  document.getElementById("stat-doom").textContent = state.doom;

  const statusLine = document.getElementById("status-line");
  statusLine.innerHTML = statusPills(state)
    .map((p) => `<span class="pill">${p}</span>`)
    .join("");

  document.getElementById("seed-display").textContent =
    state.status === "running"
      ? `Active seed: ${state.seed}`
      : `Run ended. Seed was ${state.seed}`;

  renderLog(state);
  renderCurrent(state);

  const playBtn = document.getElementById("play-turn");
  const cashBtn = document.getElementById("cash-out");
  playBtn.disabled = state.status !== "running";
  cashBtn.disabled = state.status !== "running";
}

function renderCurrent(state) {
  const slot = document.getElementById("current-event");
  slot.innerHTML = "";
  const latest = [...state.log].reverse().find((entry) => entry.type === "turn");
  if (!latest) {
    slot.innerHTML = '<div class="empty-state">No beats yet. Tap “Draw next beat.”</div>';
    return;
  }
  const el = document.createElement("div");
  el.className = "log-entry";
  el.innerHTML = `
    <h3>${latest.title}</h3>
    <p>${latest.body}</p>
    <small>${latest.meta}</small>
  `;
  slot.appendChild(el);
}

function renderLog(state) {
  const container = document.getElementById("run-log");
  container.innerHTML = "";
  if (!state.log.length) {
    container.innerHTML = '<div class="empty-state">No history yet.</div>';
    return;
  }
  state.log
    .slice(-12)
    .reverse()
    .forEach((entry) => {
      const card = document.createElement("div");
      card.className = "log-entry";
      card.innerHTML = `
        <h3>${entry.title}</h3>
        <p>${entry.body}</p>
        <small>${entry.meta}</small>
      `;
      container.appendChild(card);
    });
}

function addLog(state, entry) {
  state.log.push({ ...entry, timestamp: Date.now() });
}

function clamp(num, min, max) {
  return Math.max(min, Math.min(num, max));
}

function momentumShift(state) {
  const roll = Math.floor(random(state) * 6) + 1;
  if (roll === 1) return -1;
  if (roll === 6) return 2;
  if (roll >= 5) return 1;
  return 0;
}

function aegisShift(state) {
  const roll = Math.floor(random(state) * 6) + 1;
  return roll === 1 ? -1 : 0;
}

function doomShift(state) {
  let increment = 0;
  if (state.mode === "gauntlet") increment += 1;
  if (state.momentum < 0) increment += 1;
  if (state.aegis <= 0) increment += 1;
  return increment;
}

function takeTurn(state) {
  if (state.status !== "running") return;

  const axis = pick(AXES, state);
  const vector = pick(VECTORS, state);
  const timeline = pick(TIMELINES, state);
  const archetype = pick(ARCHETYPES, state);
  const situation = pick(SITUATIONS, state);
  const boon = random(state) > 0.55 ? pick(BOONS, state) : null;
  const threat = random(state) > 0.55 ? pick(THREATS, state) : null;

  state.depth += 1;
  state.momentum = clamp(state.momentum + momentumShift(state), -2, 6);
  state.aegis = clamp(state.aegis + aegisShift(state), 0, 4);
  state.doom = clamp(state.doom + doomShift(state), 0, 6);

  const lines = [
    `${axis.name} × ${vector.name} (${timeline.name})`,
    `${archetype.name}: ${archetype.tagline}`,
    situation
  ];
  if (boon) lines.push(`✨ ${boon}`);
  if (threat) lines.push(`⚠️ ${threat}`);

  addLog(state, {
    type: "turn",
    title: `Depth ${state.depth} • Momentum ${state.momentum}`,
    body: lines.join(" • "),
    meta: new Date().toLocaleTimeString()
  });

  if (state.doom >= 6 || state.aegis <= 0) {
    addLog(state, {
      type: "crash",
      title: "Run collapsed",
      body: `Doom hit ${state.doom}. Aegis at ${state.aegis}. Cash out or reset.`,
      meta: "Labyrinth spits you out"
    });
    state.status = "ended";
  }
}

function cashOut(state) {
  if (state.status !== "running") return;
  state.status = "ended";
  addLog(state, {
    type: "cashout",
    title: "Run banked",
    body: `Depth ${state.depth}, Momentum ${state.momentum}, Aegis ${state.aegis}, Doom ${state.doom}. Share seed ${state.seed}.`,
    meta: "You step out with what you can carry"
  });
}

function resetRun() {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch (err) {
    console.warn("Could not clear stored state", err);
  }
  memoryFallback = null;
  return baseState();
}

function startOrResume(currentState) {
  const nameInput = document.getElementById("player-name").value.trim();
  const questInput = document.getElementById("player-quest").value.trim();
  const mode = document.getElementById("mode").value;
  const difficulty = document.getElementById("difficulty").value;
  const seedInput = document.getElementById("seed").value.trim();

  let next = currentState;
  if (!currentState || currentState.status !== "running") {
    next = baseState({
      player: nameInput,
      quest: questInput,
      mode,
      difficulty,
      seed: seedInput || undefined
    });
    applyDifficulty(next);
    addLog(next, {
      type: "start",
      title: "Run initialized",
      body: `Mode ${mode}, difficulty ${difficulty}, quest ${questInput || "open"}. Seed ${next.seed}.`,
      meta: "All threads aligned"
    });
  }
  return next;
}

function hydrateInputs(state) {
  document.getElementById("player-name").value = state.player || "";
  document.getElementById("player-quest").value = state.quest || "";
  document.getElementById("mode").value = state.mode;
  document.getElementById("difficulty").value = state.difficulty;
  document.getElementById("seed").value = state.seed;
}

function bootstrap() {
  let state = loadState() || baseState();
  applyDifficulty(state);
  hydrateInputs(state);
  notify(null);
  renderState(state);

  document.getElementById("start-run").addEventListener("click", () => {
    state = startOrResume(state);
    hydrateInputs(state);
    saveState(state);
    renderState(state);
  });

  document.getElementById("play-turn").addEventListener("click", () => {
    state = startOrResume(state);
    takeTurn(state);
    saveState(state);
    renderState(state);
  });

  document.getElementById("cash-out").addEventListener("click", () => {
    cashOut(state);
    saveState(state);
    renderState(state);
  });

  document.getElementById("reset-run").addEventListener("click", () => {
    if (!confirm("Reset the current run?")) return;
    state = resetRun();
    applyDifficulty(state);
    hydrateInputs(state);
    renderState(state);
  });
}

document.addEventListener("DOMContentLoaded", bootstrap);
