// HALO Pocket Labyrinth: Online roguelike card crawler with gacha and relay-ready co-op

const STORAGE_KEY = "halo_mobile_state_v2";
const ONLINE_DEFAULT = "ws://localhost:8787";
let socket = null;
let warnedStorage = false;
let memoryFallback = null;

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

const RARITY_WEIGHTS = {
  common: 70,
  rare: 25,
  mythic: 5
};

const PACKS = {
  starter: { name: "Pulse Pack", size: 3, cost: { credits: 250 } },
  radiant: { name: "Radiant Pull", size: 5, cost: { embers: 80 }, bonusRare: true }
};

const CARD_POOL = [
  {
    id: "rush",
    name: "Momentum Rush",
    rarity: "common",
    axis: "Mind",
    text: "+1 Momentum. Draw 1."
  },
  {
    id: "ward",
    name: "Aegis Ward",
    rarity: "common",
    axis: "Body",
    text: "Restore 1 Aegis. If a threat is present, prevent 1 Doom."
  },
  {
    id: "spark",
    name: "Prismatic Spark",
    rarity: "common",
    axis: "Spirit",
    text: "Gain 1 Momentum and reveal the boon on this beat if any."
  },
  {
    id: "mirror",
    name: "Mirror Veil",
    rarity: "rare",
    axis: "Mind",
    text: "If a threat exists, turn it into a boon. Otherwise +1 Aegis."
  },
  {
    id: "gate",
    name: "Gatekeeper's Key",
    rarity: "rare",
    axis: "Domain",
    text: "Reduce Doom by 1 and bank current Momentum as Credits."
  },
  {
    id: "star",
    name: "Starfall Surge",
    rarity: "rare",
    axis: "Fate",
    text: "+2 Momentum, then lose 1 Aegis."
  },
  {
    id: "tower",
    name: "Tower Break",
    rarity: "rare",
    axis: "Fate",
    text: "Clear hand, draw 3 fresh cards, Doom cannot increase this beat."
  },
  {
    id: "time",
    name: "Time Lattice",
    rarity: "mythic",
    axis: "Spirit",
    text: "Set Doom to 0 or Depth-1 (whichever is lower). Gain +1 Aegis."
  },
  {
    id: "empress",
    name: "Empress Bloom",
    rarity: "mythic",
    axis: "Body",
    text: "Heal to 3 Aegis, add +2 Momentum, then bank 1 Ember."
  },
  {
    id: "magus",
    name: "Magus Rewrite",
    rarity: "mythic",
    axis: "Mind",
    text: "Replay the last boon you saw and draw 2 cards."
  }
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
    hash |= 0;
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

function shuffle(list, state) {
  const arr = [...list];
  for (let i = arr.length - 1; i > 0; i--) {
    const j = Math.floor(random(state) * (i + 1));
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
  return arr;
}

function generateSeed() {
  const chars = "ABCDEFGHJKMNPQRSTUVWXYZ23456789";
  let out = "HALO-";
  for (let i = 0; i < 6; i++) out += chars[Math.floor(Math.random() * chars.length)];
  return out;
}

function starterCollection() {
  const inventory = {};
  CARD_POOL.forEach((c) => {
    if (c.rarity === "common") inventory[c.id] = 2;
    if (c.rarity === "rare") inventory[c.id] = 1;
  });
  return inventory;
}

function baseState(overrides = {}) {
  const seed = overrides.seed || generateSeed();
  const seedHash = hashSeed(seed);
  return {
    player: overrides.player || "",
    quest: overrides.quest || "",
    mode: overrides.mode || "solo",
    difficulty: overrides.difficulty || "standard",
    seed,
    seedHash,
    cursor: 1,
    currencies: { credits: 800, embers: 160, shards: 0 },
    pity: 0,
    vip: false,
    collection: starterCollection(),
    deck: ["rush", "rush", "ward", "spark", "mirror", "gate", "star", "tower"],
    run: null,
    online: {
      enabled: false,
      url: overrides.url || ONLINE_DEFAULT,
      room: "",
      status: "offline",
      peers: [],
      log: []
    },
    gachaLog: [],
    log: []
  };
}

function clamp(num, min, max) {
  return Math.max(min, Math.min(max, num));
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

function addLog(state, entry) {
  state.log.push({ ...entry, timestamp: Date.now() });
  state.log = state.log.slice(-80);
}

function rarityRoll(state) {
  const bonus = state.vip ? 5 : 0;
  const roll = random(state) * (100 + bonus);
  const mythicCut = RARITY_WEIGHTS.mythic + bonus;
  if (roll >= 100 - mythicCut) return "mythic";
  if (roll >= 100 - RARITY_WEIGHTS.rare) return "rare";
  return "common";
}

function randomCardByRarity(rarity, state) {
  const pool = CARD_POOL.filter((c) => c.rarity === rarity);
  return pick(pool, state);
}

function addToCollection(state, cardId) {
  state.collection[cardId] = (state.collection[cardId] || 0) + 1;
}

function pullPack(state, packKey) {
  const pack = PACKS[packKey];
  if (!pack) return { results: [], reason: "Missing pack" };
  const cost = pack.cost;
  if (cost.credits && state.currencies.credits < cost.credits)
    return { results: [], reason: "Not enough credits" };
  if (cost.embers && state.currencies.embers < cost.embers)
    return { results: [], reason: "Not enough embers" };

  if (cost.credits) state.currencies.credits -= cost.credits;
  if (cost.embers) state.currencies.embers -= cost.embers;

  const results = [];
  for (let i = 0; i < pack.size; i++) {
    let rarity = rarityRoll(state);
    if (state.pity >= 8) rarity = "rare";
    if (pack.bonusRare && i === pack.size - 1) rarity = rarity === "common" ? "rare" : rarity;
    const card = randomCardByRarity(rarity, state);
    addToCollection(state, card.id);
    results.push(card);
    state.pity = rarity === "rare" || rarity === "mythic" ? 0 : state.pity + 1;
  }

  state.gachaLog.push({
    pack: pack.name,
    results,
    timestamp: Date.now()
  });

  return { results };
}

function ensureDeckLegal(state) {
  const cleaned = state.deck.filter((id) => state.collection[id]);
  if (!cleaned.length) cleaned.push("rush", "rush", "ward", "spark");
  state.deck = cleaned.slice(0, 12);
}

function drawCards(run, state, count = 1) {
  for (let i = 0; i < count; i++) {
    if (!run.drawPile.length) {
      run.drawPile = shuffle(run.discard, state);
      run.discard = [];
    }
    if (!run.drawPile.length) break;
    run.hand.push(run.drawPile.shift());
  }
}

function nextEncounter(state) {
  const encounter = {
    axis: pick(AXES, state),
    vector: pick(VECTORS, state),
    timeline: pick(TIMELINES, state),
    boon: random(state) > 0.55,
    threat: random(state) > 0.5,
    situation: pick(
      [
        "A rival faction challenges your route",
        "Two timelines overlap; pick one to stabilize",
        "An ally pings you from deeper layers",
        "A cache of relics hums with risk",
        "A distorted mirror tries to rewrite you",
        "A sealed gate leaks starlight",
        "A phantom deal returns for payment"
      ],
      state
    )
  };
  return encounter;
}

function baseRun(state) {
  const run = {
    status: "running",
    depth: 0,
    momentum: state.difficulty === "chill" ? 3 : 2,
    aegis: state.difficulty === "brutal" ? 1 : 2,
    doom: state.difficulty === "brutal" ? 1 : 0,
    drawPile: shuffle(state.deck, state),
    discard: [],
    hand: [],
    current: null,
    lastBoon: null
  };
  drawCards(run, state, 3);
  run.current = nextEncounter(state);
  return run;
}

function startRun(state) {
  ensureDeckLegal(state);
  state.run = baseRun(state);
  addLog(state, { type: "run", title: "Run initialized", body: `Deck size ${state.deck.length}`, meta: state.seed });
}

function endRun(state, reason = "banked") {
  if (!state.run) return;
  const reward = Math.max(0, state.run.depth + state.run.momentum);
  state.currencies.credits += reward * 20;
  if (reason === "victory") state.currencies.embers += 10;
  addLog(state, {
    type: "cashout",
    title: `Run ${reason}`,
    body: `Depth ${state.run.depth}, Momentum ${state.run.momentum}, Aegis ${state.run.aegis}, Doom ${state.run.doom}. +${
      reward * 20
    } credits`,
    meta: new Date().toLocaleTimeString()
  });
  state.run = null;
}

function doomTick(run) {
  let inc = 0;
  if (run.momentum < 0) inc += 1;
  if (run.aegis <= 0) inc += 1;
  return inc;
}

function applyCard(state, run, card, encounter) {
  switch (card.id) {
    case "rush":
      run.momentum = clamp(run.momentum + 1, -2, 8);
      drawCards(run, state, 1);
      break;
    case "ward":
      run.aegis = clamp(run.aegis + 1, 0, 5);
      if (encounter.threat) run.doom = clamp(run.doom - 1, 0, 6);
      break;
    case "spark":
      run.momentum = clamp(run.momentum + 1, -2, 8);
      if (encounter.boon) run.lastBoon = encounter;
      break;
    case "mirror":
      if (encounter.threat) {
        encounter.threat = false;
        encounter.boon = true;
        run.lastBoon = encounter;
      } else {
        run.aegis = clamp(run.aegis + 1, 0, 5);
      }
      break;
    case "gate":
      run.doom = clamp(run.doom - 1, 0, 6);
      state.currencies.credits += Math.max(0, run.momentum) * 30;
      break;
    case "star":
      run.momentum = clamp(run.momentum + 2, -2, 8);
      run.aegis = clamp(run.aegis - 1, 0, 5);
      break;
    case "tower":
      run.hand = [];
      run.drawPile = shuffle(run.drawPile.concat(run.discard), state);
      run.discard = [];
      drawCards(run, state, 3);
      run.doom = clamp(run.doom, 0, 5);
      break;
    case "time":
      run.doom = clamp(Math.min(run.doom, Math.max(0, run.depth - 1)), 0, 6);
      run.aegis = clamp(run.aegis + 1, 0, 5);
      break;
    case "empress":
      run.aegis = 3;
      run.momentum = clamp(run.momentum + 2, -2, 8);
      state.currencies.embers += 1;
      break;
    case "magus":
      if (run.lastBoon) {
        run.momentum = clamp(run.momentum + 1, -2, 8);
        run.aegis = clamp(run.aegis + 1, 0, 5);
      }
      drawCards(run, state, 2);
      break;
    default:
      break;
  }
}

function playCard(state, cardId) {
  if (!state.run || state.run.status !== "running") return;
  const idx = state.run.hand.indexOf(cardId);
  if (idx === -1) return;
  const [cardRef] = state.run.hand.splice(idx, 1);
  const card = CARD_POOL.find((c) => c.id === cardRef);
  const encounter = state.run.current;
  applyCard(state, state.run, card, encounter);
  state.run.discard.push(cardRef);
}

function resolveBeat(state) {
  if (!state.run || state.run.status !== "running") return;
  const run = state.run;
  run.depth += 1;
  const encounter = run.current;

  // Apply base tick
  run.doom = clamp(run.doom + doomTick(run), 0, 6);

  const title = `${encounter.axis.name} × ${encounter.vector.name} (${encounter.timeline.name})`;
  const lines = [encounter.situation];
  if (encounter.boon) lines.push("✨ Boon in play");
  if (encounter.threat) lines.push("⚠️ Threat in play");

  addLog(state, {
    type: "turn",
    title: `Depth ${run.depth} • Momentum ${run.momentum}`,
    body: lines.join(" • "),
    meta: new Date().toLocaleTimeString()
  });

  if (run.doom >= 6 || run.aegis <= 0) {
    run.status = "crashed";
    addLog(state, {
      type: "crash",
      title: "Run collapsed",
      body: `Doom ${run.doom}, Aegis ${run.aegis}. Bank or reset.`,
      meta: "Labyrinth spits you out"
    });
    pushOnline(state, { kind: "crash", depth: run.depth, doom: run.doom });
    return;
  }

  drawCards(run, state, 1);
  run.current = nextEncounter(state);
  pushOnline(state, { kind: "sync", depth: run.depth, momentum: run.momentum, doom: run.doom });
}

function cashOut(state) {
  endRun(state, "banked");
}

function resetRun(state) {
  state.run = null;
  state.seed = generateSeed();
  state.seedHash = hashSeed(state.seed);
  state.cursor = 1;
}

function toggleVip(state) {
  if (state.vip) return;
  const cost = 120;
  if (state.currencies.embers < cost) return notify("Need more embers for VIP Blessing");
  state.currencies.embers -= cost;
  state.vip = true;
  addLog(state, { type: "vip", title: "VIP Blessing unlocked", body: "Rarity boosts active", meta: "Paid feature" });
}

// Online sync
function connectOnline(state) {
  if (!state.online.enabled) {
    disconnectOnline(state);
    return;
  }
  if (socket && socket.readyState === WebSocket.OPEN) return;
  try {
    socket = new WebSocket(state.online.url || ONLINE_DEFAULT);
  } catch (err) {
    notify("Failed to start socket. Check relay URL.");
    state.online.status = "offline";
    return;
  }
  socket.addEventListener("open", () => {
    state.online.status = "connected";
    socket.send(
      JSON.stringify({ type: "join", room: state.online.room || state.seed, player: state.player || "anon" })
    );
    render(state);
  });
  socket.addEventListener("message", (event) => {
    const data = JSON.parse(event.data);
    if (data.type === "welcome") {
      state.online.status = "connected";
      state.online.log.push({ meta: "system", message: `Joined ${data.room} with ${data.peers} peers` });
    }
    if (data.type === "system") state.online.log.push({ meta: "system", message: data.message });
    if (data.type === "chat") state.online.log.push({ meta: data.from, message: data.message });
    if (data.type === "sync") state.online.log.push({ meta: data.from, message: `Depth ${data.payload.depth}` });
    state.online.log = state.online.log.slice(-30);
    render(state);
  });
  socket.addEventListener("close", () => {
    state.online.status = "offline";
    render(state);
  });
  socket.addEventListener("error", () => {
    notify("Relay connection failed");
    state.online.status = "offline";
    render(state);
  });
}

function disconnectOnline(state) {
  if (socket) socket.close();
  state.online.status = "offline";
}

function pushOnline(state, payload) {
  if (!state.online.enabled || !socket || socket.readyState !== WebSocket.OPEN) return;
  socket.send(JSON.stringify({ type: "sync", payload }));
}

// Rendering helpers
function renderProfile(state) {
  document.getElementById("player-name").value = state.player;
  document.getElementById("player-quest").value = state.quest;
  document.getElementById("mode").value = state.mode;
  document.getElementById("difficulty").value = state.difficulty;
  document.getElementById("seed").value = state.seed;
  document.getElementById("online-url").value = state.online.url;
  document.getElementById("online-room").value = state.online.room || state.seed;
  document.getElementById("online-enabled").checked = state.online.enabled;
  document.getElementById("seed-display").textContent = `Seed: ${state.seed}`;
}

function renderEconomy(state) {
  document.getElementById("stat-credits").textContent = state.currencies.credits;
  document.getElementById("stat-embers").textContent = state.currencies.embers;
  document.getElementById("stat-shards").textContent = state.currencies.shards;
  document.getElementById("vip-status").textContent = state.vip ? "VIP active" : "Standard";
}

function renderCollection(state) {
  const deckList = document.getElementById("deck-list");
  const collectionList = document.getElementById("collection-list");
  deckList.innerHTML = "";
  collectionList.innerHTML = "";

  state.deck.forEach((id, idx) => {
    const card = CARD_POOL.find((c) => c.id === id);
    const el = document.createElement("div");
    el.className = "pill-row card-pill";
    el.innerHTML = `<strong>${card.name}</strong><span class="pill">${card.rarity}</span><button data-remove="${
      idx
    }" class="ghost">Remove</button>`;
    deckList.appendChild(el);
  });

  CARD_POOL.forEach((card) => {
    const owned = state.collection[card.id] || 0;
    const el = document.createElement("div");
    el.className = "pill-row card-pill";
    el.innerHTML = `<div><strong>${card.name}</strong> <span class="pill">${card.rarity}</span> <small>${card.axis}</small></div><div class="pill">x${owned}</div><button data-add="${
      card.id
    }" class="ghost">Add</button>`;
    collectionList.appendChild(el);
  });

  deckList.querySelectorAll("button[data-remove]").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const idx = parseInt(e.target.getAttribute("data-remove"), 10);
      state.deck.splice(idx, 1);
      saveState(state);
      render(state);
    });
  });

  collectionList.querySelectorAll("button[data-add]").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const id = e.target.getAttribute("data-add");
      if ((state.collection[id] || 0) <= state.deck.filter((c) => c === id).length) return;
      state.deck.push(id);
      ensureDeckLegal(state);
      saveState(state);
      render(state);
    });
  });
}

function renderRun(state) {
  const run = state.run;
  document.getElementById("stat-momentum").textContent = run ? run.momentum : "–";
  document.getElementById("stat-aegis").textContent = run ? run.aegis : "–";
  document.getElementById("stat-depth").textContent = run ? run.depth : "–";
  document.getElementById("stat-doom").textContent = run ? run.doom : "–";

  const current = document.getElementById("current-event");
  current.innerHTML = "";
  if (!run) {
    current.innerHTML = '<div class="empty-state">Start a run to see encounters.</div>';
  } else {
    const enc = run.current;
    const card = document.createElement("div");
    card.className = "log-entry";
    card.innerHTML = `<h3>${enc.axis.name} × ${enc.vector.name} (${enc.timeline.name})</h3><p>${enc.situation}</p><small>${
      enc.boon ? "✨ Boon available" : ""
    } ${enc.threat ? "⚠️ Threat active" : ""}</small>`;
    current.appendChild(card);
  }

  const hand = document.getElementById("hand");
  hand.innerHTML = "";
  if (run) {
    run.hand.forEach((cardId) => {
      const card = CARD_POOL.find((c) => c.id === cardId);
      const el = document.createElement("button");
      el.className = "card-btn";
      el.textContent = `${card.name} (${card.rarity})`;
      el.addEventListener("click", () => {
        playCard(state, cardId);
        render(state);
      });
      hand.appendChild(el);
    });
  }

  const logBox = document.getElementById("run-log");
  logBox.innerHTML = "";
  state.log
    .slice(-14)
    .reverse()
    .forEach((entry) => {
      const el = document.createElement("div");
      el.className = "log-entry";
      el.innerHTML = `<h3>${entry.title}</h3><p>${entry.body}</p><small>${entry.meta}</small>`;
      logBox.appendChild(el);
    });

  document.getElementById("play-turn").disabled = !run;
  document.getElementById("cash-out").disabled = !run;
}

function renderGacha(state) {
  const results = document.getElementById("gacha-results");
  const last = state.gachaLog[state.gachaLog.length - 1];
  if (!last) {
    results.innerHTML = '<div class="empty-state">Open a pack to see pulls.</div>';
    return;
  }
  results.innerHTML = `<p><strong>${last.pack}</strong> yielded:</p>`;
  last.results.forEach((card) => {
    const el = document.createElement("div");
    el.className = "log-entry";
    el.innerHTML = `<h3>${card.name}</h3><p>${card.text}</p><small>${card.rarity} • ${card.axis}</small>`;
    results.appendChild(el);
  });
}

function renderOnline(state) {
  const status = document.getElementById("online-status");
  status.textContent = state.online.status;
  const log = document.getElementById("online-log");
  log.innerHTML = "";
  state.online.log
    .slice(-8)
    .reverse()
    .forEach((entry) => {
      const el = document.createElement("div");
      el.className = "log-entry";
      el.innerHTML = `<h3>${entry.meta}</h3><p>${entry.message}</p>`;
      log.appendChild(el);
    });
}

function render(state) {
  renderProfile(state);
  renderEconomy(state);
  renderCollection(state);
  renderRun(state);
  renderGacha(state);
  renderOnline(state);
}

function bootstrap() {
  let state = loadState() || baseState();
  render(state);

  document.getElementById("start-run").addEventListener("click", () => {
    state.player = document.getElementById("player-name").value.trim();
    state.quest = document.getElementById("player-quest").value.trim();
    state.mode = document.getElementById("mode").value;
    state.difficulty = document.getElementById("difficulty").value;
    state.seed = document.getElementById("seed").value.trim() || generateSeed();
    state.seedHash = hashSeed(state.seed);
    state.cursor = 1;
    startRun(state);
    saveState(state);
    render(state);
  });

  document.getElementById("reset-run").addEventListener("click", () => {
    if (!confirm("Reset the current run?")) return;
    resetRun(state);
    saveState(state);
    render(state);
  });

  document.getElementById("play-turn").addEventListener("click", () => {
    resolveBeat(state);
    saveState(state);
    render(state);
  });

  document.getElementById("cash-out").addEventListener("click", () => {
    cashOut(state);
    saveState(state);
    render(state);
  });

  document.getElementById("pull-starter").addEventListener("click", () => {
    const res = pullPack(state, "starter");
    if (!res.results.length && res.reason) return notify(res.reason);
    saveState(state);
    render(state);
  });

  document.getElementById("pull-radiant").addEventListener("click", () => {
    const res = pullPack(state, "radiant");
    if (!res.results.length && res.reason) return notify(res.reason);
    saveState(state);
    render(state);
  });

  document.getElementById("buy-embers").addEventListener("click", () => {
    state.currencies.embers += 300;
    addLog(state, { type: "purchase", title: "Simulated purchase", body: "+300 Embers", meta: "Test harness" });
    saveState(state);
    render(state);
  });

  document.getElementById("vip-upgrade").addEventListener("click", () => {
    toggleVip(state);
    saveState(state);
    render(state);
  });

  document.getElementById("online-enabled").addEventListener("change", (e) => {
    state.online.enabled = e.target.checked;
    state.online.url = document.getElementById("online-url").value.trim() || ONLINE_DEFAULT;
    state.online.room = document.getElementById("online-room").value.trim() || state.seed;
    saveState(state);
    connectOnline(state);
    render(state);
  });

  document.getElementById("online-url").addEventListener("change", (e) => {
    state.online.url = e.target.value.trim();
    saveState(state);
  });

  document.getElementById("online-room").addEventListener("change", (e) => {
    state.online.room = e.target.value.trim();
    saveState(state);
  });

  document.getElementById("send-chat").addEventListener("click", () => {
    const text = document.getElementById("chat-text").value.trim();
    if (!text || !socket || socket.readyState !== WebSocket.OPEN) return;
    socket.send(JSON.stringify({ type: "chat", message: text }));
    document.getElementById("chat-text").value = "";
  });
}

document.addEventListener("DOMContentLoaded", bootstrap);
