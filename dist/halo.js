// HALO Meta-Divination Engine — rebuilt from scratch for the meta-game roguelike
// Main menu + multiplayer relay + gacha + Wu Xing archetype combat + oracle-driven encounters

const STORAGE_KEY = "halo_meta_state_v1";
const LEGACY_KEYS = ["halo_mobile_state_v2", "halo_state", "halo_meta_arc_v0"];
const ONLINE_DEFAULT = "ws://localhost:8787";
let socket = null;
let warnedStorage = false;
let memoryFallback = null;

// Wu Xing elemental wheel (using the controlling cycle for advantage)
const ELEMENTS = {
  wood: { strong: "earth", weak: "metal", color: "#4CAF50" },
  fire: { strong: "metal", weak: "water", color: "#FF7043" },
  earth: { strong: "water", weak: "wood", color: "#C0A46B" },
  metal: { strong: "wood", weak: "fire", color: "#B0BEC5" },
  water: { strong: "fire", weak: "earth", color: "#4FC3F7" }
};

const DIVINATION_DICE = [
  { name: "Tarot Major", sides: 22 },
  { name: "I Ching", sides: 64 },
  { name: "Runes", sides: 24 },
  { name: "Astral Houses", sides: 12 },
  { name: "Void Die", sides: 33 }
];

const DIVINATION_THEMES = [
  "Initiation", "Challenge", "Reversal", "Breakthrough", "Union", "Fragment", "Signal", "Riddle",
  "Gift", "Debt", "Memory", "Future Echo", "Threshold", "Labyrinth", "Guardian", "Bloom"
];

const ARCHETYPES = [
  {
    id: "sovereign",
    name: "Sovereign Ember",
    rarity: "common",
    element: "fire",
    text: "+2 Momentum. If advantaged, burn 1 Doom.",
    effect: { momentum: 2, doom: -1 }
  },
  {
    id: "warden",
    name: "Warden of Roots",
    rarity: "common",
    element: "wood",
    text: "Restore 1 Aegis. If advantaged, draw 1 archetype.",
    effect: { aegis: 1, draw: 1 }
  },
  {
    id: "seeker",
    name: "Seeker of Currents",
    rarity: "common",
    element: "water",
    text: "+1 Momentum, reveal omen. If advantaged, convert omen to boon.",
    effect: { momentum: 1, boon: true }
  },
  {
    id: "architect",
    name: "Architect of Stone",
    rarity: "rare",
    element: "earth",
    text: "Stabilize: set Doom to 0 if advantaged; otherwise -1 Doom.",
    effect: { doom: -1, stabilize: true }
  },
  {
    id: "mirror",
    name: "Mirror of Blades",
    rarity: "rare",
    element: "metal",
    text: "Reflect threat. If advantaged, gain 1 Momentum and 1 Aegis.",
    effect: { reflect: true, momentum: 1, aegis: 1 }
  },
  {
    id: "oracle",
    name: "Prismatic Oracle",
    rarity: "rare",
    element: "water",
    text: "Roll two oracle dice, pick best. Gain +1 Momentum per hit.",
    effect: { doubleRoll: true }
  },
  {
    id: "phoenix",
    name: "Phoenix Crown",
    rarity: "mythic",
    element: "fire",
    text: "Heal to 3 Aegis, +2 Momentum. Advantage adds bonus credit.",
    effect: { aegis: 3, momentum: 2, bonus: true }
  },
  {
    id: "river",
    name: "River Between Worlds",
    rarity: "mythic",
    element: "water",
    text: "Summon ally: set Momentum to 3, draw 2, reveal omen.",
    effect: { momentumSet: 3, draw: 2, boon: true }
  },
  {
    id: "forge",
    name: "Starforge Anvil",
    rarity: "mythic",
    element: "metal",
    text: "Cash any Momentum into Credits x10, then set Momentum to 1.",
    effect: { cashMomentum: true }
  },
  {
    id: "labyrinth",
    name: "Labyrinth Keeper",
    rarity: "rare",
    element: "earth",
    text: "Mark the path: reduce Depth cost by 1 this beat; draw 1.",
    effect: { depthShield: true, draw: 1 }
  }
];

const RARITY_WEIGHTS = { common: 68, rare: 25, mythic: 7 };
const PACKS = {
  pulse: { name: "Pulse Pack", size: 3, cost: { credits: 250 } },
  radiant: { name: "Radiant Cache", size: 5, cost: { embers: 80 }, bonusRare: true },
  legend: { name: "Axis Vault", size: 1, cost: { shards: 1 }, guaranteed: "mythic" }
};

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
  ARCHETYPES.forEach((c) => {
    if (c.rarity === "common") inventory[c.id] = 2;
    if (c.rarity === "rare") inventory[c.id] = 1;
  });
  return inventory;
}

function baseState(overrides = {}) {
  const seed = overrides.seed || generateSeed();
  const seedHash = hashSeed(seed);
  return {
    profile: {
      pilot: overrides.pilot || "",
      title: "Axis Runner",
      quest: overrides.quest || "",
      seed,
      difficulty: overrides.difficulty || "standard",
      mode: overrides.mode || "solo"
    },
    seedHash,
    cursor: 1,
    currencies: { credits: 1200, embers: 220, shards: 1 },
    vip: false,
    pity: 0,
    collection: starterCollection(),
    deck: ["sovereign", "warden", "seeker", "architect", "mirror", "labyrinth"],
    relics: [],
    run: null,
    online: {
      enabled: false,
      url: overrides.url || ONLINE_DEFAULT,
      room: "",
      status: "offline",
      peers: [],
      log: []
    },
    codex: [],
    gachaLog: [],
    log: []
  };
}

function migrateLegacy() {
  try {
    for (const key of LEGACY_KEYS) {
      const raw = localStorage.getItem(key);
      if (raw) {
        const parsed = JSON.parse(raw);
        memoryFallback = parsed;
        return parsed;
      }
    }
  } catch (err) {
    console.warn("Legacy migration failed", err);
  }
  return null;
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
    console.warn("Failed to load state", err);
    if (memoryFallback) return memoryFallback;
  }
  const legacy = migrateLegacy();
  if (legacy) return { ...baseState(), ...legacy };
  return memoryFallback || baseState();
}

function addLog(state, entry) {
  state.log.push({ ...entry, timestamp: Date.now() });
  state.log = state.log.slice(-120);
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
  const pool = ARCHETYPES.filter((c) => c.rarity === rarity);
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
  if (cost.shards && state.currencies.shards < cost.shards)
    return { results: [], reason: "Not enough shards" };

  if (cost.credits) state.currencies.credits -= cost.credits;
  if (cost.embers) state.currencies.embers -= cost.embers;
  if (cost.shards) state.currencies.shards -= cost.shards;

  const results = [];
  for (let i = 0; i < pack.size; i++) {
    let rarity = pack.guaranteed || rarityRoll(state);
    if (state.pity >= 8) rarity = "rare";
    if (pack.bonusRare && i === pack.size - 1 && rarity === "common") rarity = "rare";
    const card = randomCardByRarity(rarity, state);
    addToCollection(state, card.id);
    results.push(card);
    state.pity = rarity === "rare" || rarity === "mythic" ? 0 : state.pity + 1;
  }

  state.gachaLog.push({ pack: pack.name, results, timestamp: Date.now() });
  return { results };
}

function ensureDeckLegal(state) {
  const cleaned = state.deck.filter((id) => state.collection[id]);
  if (!cleaned.length) cleaned.push("sovereign", "warden", "seeker");
  state.deck = cleaned.slice(0, 12);
}

function elementAdvantage(cardElement, encounterElement) {
  if (!ELEMENTS[cardElement] || !ELEMENTS[encounterElement]) return 0;
  if (ELEMENTS[cardElement].strong === encounterElement) return 1;
  if (ELEMENTS[cardElement].weak === encounterElement) return -1;
  return 0;
}

function rollOracleDie(state) {
  const die = pick(DIVINATION_DICE, state);
  const roll = Math.floor(random(state) * die.sides) + 1;
  return { die: die.name, sides: die.sides, roll };
}

function nextEncounter(state) {
  const omen = pick(DIVINATION_THEMES, state);
  const elementKeys = Object.keys(ELEMENTS);
  return {
    element: pick(elementKeys, state),
    theme: omen,
    dice: rollOracleDie(state),
    boon: random(state) > 0.55,
    threat: random(state) > 0.45,
    lore: pick(
      [
        "Echoes of past selves ask for alignment.",
        "Two timelines braid together and demand a choice.",
        "A silent guardian flips an unseen coin.",
        "The Labyrinth grows new corridors in real time.",
        "Archetype avatars convene at the Inner Court.",
        "A future self hands you a mirrored key.",
        "A rival faction offers a pact of convenience."
      ],
      state
    )
  };
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

function baseRun(state) {
  const run = {
    status: "running",
    depth: 0,
    momentum: state.profile.difficulty === "chill" ? 3 : 2,
    aegis: state.profile.difficulty === "brutal" ? 1 : 3,
    doom: 0,
    drawPile: shuffle(state.deck, state),
    discard: [],
    hand: [],
    current: null,
    lastOmen: null,
    route: []
  };
  drawCards(run, state, 4);
  run.current = nextEncounter(state);
  return run;
}

function startRun(state) {
  ensureDeckLegal(state);
  state.run = baseRun(state);
  addLog(state, { type: "run", title: "Run initialized", body: `Seed ${state.profile.seed}` });
}

function endRun(state, reason = "banked") {
  if (!state.run) return;
  const reward = Math.max(0, state.run.depth + state.run.momentum);
  state.currencies.credits += reward * 15;
  if (reason === "victory") state.currencies.embers += 12;
  addLog(state, {
    type: "cashout",
    title: `Run ${reason}`,
    body: `Depth ${state.run.depth}, Momentum ${state.run.momentum}, Aegis ${state.run.aegis}, Doom ${state.run.doom}. +${
      reward * 15
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

function resolveCard(state, run, card) {
  const encounter = run.current;
  const advantage = elementAdvantage(card.element, encounter.element);
  let bonusCredits = 0;

  if (card.effect.momentum) run.momentum = clamp(run.momentum + card.effect.momentum + (advantage > 0 ? 1 : 0), -3, 9);
  if (card.effect.aegis)
    run.aegis = clamp(run.aegis + card.effect.aegis + (advantage > 0 ? 1 : 0), 0, 6);
  if (card.effect.doom) run.doom = clamp(run.doom + card.effect.doom - (advantage > 0 ? 1 : 0), 0, 8);
  if (card.effect.draw) drawCards(run, state, card.effect.draw);
  if (card.effect.boon) run.lastOmen = encounter;
  if (card.effect.reflect && encounter.threat) {
    encounter.threat = false;
    encounter.boon = true;
  }
  if (card.effect.stabilize && advantage > 0) run.doom = 0;
  if (card.effect.doubleRoll) {
    const first = rollOracleDie(state);
    const second = rollOracleDie(state);
    const winner = first.roll >= second.roll ? first : second;
    run.lastOmen = { ...encounter, dice: winner };
    run.momentum = clamp(run.momentum + 1 + (winner.roll > winner.sides / 2 ? 1 : 0), -3, 9);
  }
  if (card.effect.momentumSet !== undefined) run.momentum = card.effect.momentumSet;
  if (card.effect.bonus) bonusCredits += 50;
  if (card.effect.cashMomentum) {
    bonusCredits += run.momentum * 10;
    run.momentum = 1;
  }
  if (card.effect.depthShield) run.route.push({ depth: run.depth, note: "Marked" });

  if (advantage < 0) run.doom = clamp(run.doom + 1, 0, 8);
  state.currencies.credits += bonusCredits;
}

function resolveEncounter(state) {
  const run = state.run;
  if (!run) return;

  // Threat/Boon gates
  if (run.current.threat && run.doom >= 6) {
    run.aegis -= 1;
    addLog(state, { type: "threat", title: "Threat overloaded", body: "Aegis cracked under pressure" });
  }
  if (run.current.boon && run.momentum >= 4) {
    state.currencies.credits += 20;
    state.currencies.embers += 2;
  }

  run.depth += 1;
  run.discard.push(...run.hand);
  run.hand = [];
  drawCards(run, state, 4);
  run.current = nextEncounter(state);

  run.doom = clamp(run.doom + doomTick(run), 0, 8);
  if (run.aegis <= 0 || run.doom >= 8) {
    run.status = "crashed";
    endRun(state, "crashed");
  }
}

function playCard(cardId) {
  const state = window.haloState;
  if (!state.run) return;
  const idx = state.run.hand.findIndex((c) => c === cardId);
  if (idx === -1) return;
  const card = ARCHETYPES.find((c) => c.id === cardId);
  if (!card) return;
  state.run.hand.splice(idx, 1);
  resolveCard(state, state.run, card);
  state.run.discard.push(cardId);
  addLog(state, { type: "card", title: card.name, body: card.text, meta: `vs ${state.run.current.element}` });
  saveState(state);
  render();
}

function resolveBeat() {
  const state = window.haloState;
  if (!state.run) return;
  resolveEncounter(state);
  saveState(state);
  render();
}

function cashOut() {
  const state = window.haloState;
  if (!state.run) return;
  endRun(state, "banked");
  saveState(state);
  render();
}

function addCardToDeck(cardId) {
  const state = window.haloState;
  if (!state.collection[cardId]) return;
  if (state.deck.length >= 12) return;
  state.deck.push(cardId);
  saveState(state);
  render();
}

function removeCardFromDeck(cardId) {
  const state = window.haloState;
  const idx = state.deck.indexOf(cardId);
  if (idx > -1) state.deck.splice(idx, 1);
  saveState(state);
  render();
}

function toggleVIP() {
  const state = window.haloState;
  state.vip = !state.vip;
  saveState(state);
  render();
}

function connectRelay() {
  const state = window.haloState;
  if (!state.online.enabled) return;
  if (socket) socket.close();
  socket = new WebSocket(state.online.url);
  socket.onopen = () => {
    state.online.status = "online";
    state.online.log.push({ sender: "system", message: "Connected" });
    sendRelay({ type: "join", room: state.online.room || state.profile.seed, seed: state.profile.seed });
    render();
  };
  socket.onmessage = (ev) => {
    try {
      const msg = JSON.parse(ev.data);
      if (msg.type === "chat") state.online.log.push({ sender: msg.from, message: msg.text });
      if (msg.type === "sync") {
        state.online.peers = msg.peers || [];
        if (state.run) state.run.depth = Math.max(state.run.depth, msg.depth || 0);
      }
      render();
    } catch (err) {
      console.error("Relay parse", err);
    }
  };
  socket.onclose = () => {
    state.online.status = "offline";
    render();
  };
}

function sendRelay(payload) {
  const state = window.haloState;
  if (!socket || socket.readyState !== 1) return;
  const packet = { room: state.online.room || state.profile.seed, ...payload };
  socket.send(JSON.stringify(packet));
}

function sendChat(text) {
  const state = window.haloState;
  if (!state.online.enabled || !text) return;
  sendRelay({ type: "chat", from: state.profile.pilot || "anon", text });
  state.online.log.push({ sender: state.profile.pilot || "me", message: text });
  render();
}

function syncDepth() {
  const state = window.haloState;
  if (!state.run) return;
  sendRelay({ type: "sync", depth: state.run.depth, seed: state.profile.seed });
}

function renderDeck() {
  const deckEl = document.getElementById("deck-list");
  const poolEl = document.getElementById("collection-list");
  const state = window.haloState;
  if (!deckEl || !poolEl) return;
  deckEl.innerHTML = "";
  poolEl.innerHTML = "";

  state.deck.forEach((id) => {
    const card = ARCHETYPES.find((c) => c.id === id);
    if (!card) return;
    const li = document.createElement("li");
    li.textContent = `${card.name} (${card.element})`;
    li.onclick = () => removeCardFromDeck(id);
    deckEl.appendChild(li);
  });

  ARCHETYPES.forEach((card) => {
    const owned = state.collection[card.id] || 0;
    const li = document.createElement("li");
    li.textContent = `${card.name} [${card.rarity}] (${owned} owned)`;
    li.style.color = ELEMENTS[card.element].color;
    li.onclick = () => addCardToDeck(card.id);
    poolEl.appendChild(li);
  });
}

function renderRun() {
  const state = window.haloState;
  const runPanel = document.getElementById("run-panel");
  if (!runPanel) return;
  if (!state.run) {
    runPanel.innerHTML = "<p>No active run. Start from the main menu.</p>";
    return;
  }

  const encounter = state.run.current;
  const handButtons = state.run.hand
    .map((id) => {
      const card = ARCHETYPES.find((c) => c.id === id);
      return `<button class="card-btn" data-card="${id}" style="border-color:${ELEMENTS[card.element].color}">${card.name}<br/><small>${card.element}</small></button>`;
    })
    .join("");

  runPanel.innerHTML = `
    <div class="run-stats">
      <div>Depth: ${state.run.depth}</div>
      <div>Momentum: ${state.run.momentum}</div>
      <div>Aegis: ${state.run.aegis}</div>
      <div>Doom: ${state.run.doom}</div>
    </div>
    <div class="encounter">
      <strong>Encounter</strong>
      <div>Element: ${encounter.element}</div>
      <div>Theme: ${encounter.theme}</div>
      <div>Die: ${encounter.dice.die} → ${encounter.dice.roll}/${encounter.dice.sides}</div>
      <div>${encounter.lore}</div>
    </div>
    <div class="hand">${handButtons || "<em>Empty hand</em>"}</div>
    <div class="actions">
      <button id="resolve-beat">Resolve Beat</button>
      <button id="cash-out">Cash Out</button>
      <button id="sync-depth">Sync Depth</button>
    </div>
  `;

  runPanel.querySelectorAll(".card-btn").forEach((btn) => {
    btn.onclick = () => playCard(btn.dataset.card);
  });
  const resBtn = document.getElementById("resolve-beat");
  if (resBtn) resBtn.onclick = resolveBeat;
  const cashBtn = document.getElementById("cash-out");
  if (cashBtn) cashBtn.onclick = cashOut;
  const syncBtn = document.getElementById("sync-depth");
  if (syncBtn) syncBtn.onclick = syncDepth;
}

function renderMeta() {
  const state = window.haloState;
  const currencyEl = document.getElementById("currency");
  const logEl = document.getElementById("log");
  if (currencyEl)
    currencyEl.textContent = `Credits ${state.currencies.credits} | Embers ${state.currencies.embers} | Shards ${state.currencies.shards}`;
  if (logEl) {
    logEl.innerHTML = state.log
      .map((l) => `<li><strong>${l.title}</strong> — ${l.body || ""} <small>${new Date(l.timestamp).toLocaleTimeString()}</small></li>`)
      .join("");
  }
}

function renderRelay() {
  const state = window.haloState;
  const relayEl = document.getElementById("relay-log");
  const peersEl = document.getElementById("peers");
  if (relayEl)
    relayEl.innerHTML = state.online.log
      .slice(-20)
      .map((m) => `<li><strong>${m.sender}:</strong> ${m.message}</li>`)
      .join("");
  if (peersEl) peersEl.textContent = state.online.peers.join(", ");
}

function renderGachaResult(results) {
  const box = document.getElementById("gacha-results");
  if (!box) return;
  if (!results.length) {
    box.innerHTML = "<p>No pull yet.</p>";
    return;
  }
  box.innerHTML = results
    .map((c) => `<div class="gacha-card" style="border-color:${ELEMENTS[c.element].color}">${c.name}<br/><small>${c.rarity}</small></div>`)
    .join("");
}

function render() {
  renderDeck();
  renderRun();
  renderMeta();
  renderRelay();
}

function bindUI() {
  const startBtn = document.getElementById("start-run");
  const pilotInput = document.getElementById("pilot-name");
  const questInput = document.getElementById("quest");
  const seedInput = document.getElementById("seed");
  const diffInput = document.getElementById("difficulty");
  const modeInput = document.getElementById("mode");
  const vipBtn = document.getElementById("toggle-vip");
  const packBtns = document.querySelectorAll("[data-pack]");
  const chatForm = document.getElementById("chat-form");
  const chatInput = document.getElementById("chat-text");
  const relayToggle = document.getElementById("relay-enabled");
  const relayUrl = document.getElementById("relay-url");
  const relayRoom = document.getElementById("relay-room");

  if (startBtn) {
    startBtn.onclick = () => {
      const state = window.haloState;
      state.profile.pilot = pilotInput.value;
      state.profile.quest = questInput.value;
      state.profile.seed = seedInput.value || generateSeed();
      state.profile.difficulty = diffInput.value;
      state.profile.mode = modeInput.value;
      state.seedHash = hashSeed(state.profile.seed);
      state.cursor = 1;
      startRun(state);
      saveState(state);
      render();
    };
  }

  const resumeBtn = document.getElementById("resume-run");
  if (resumeBtn) resumeBtn.onclick = () => {
    const state = window.haloState;
    if (!state.run) startRun(state);
    render();
  };

  if (vipBtn) vipBtn.onclick = toggleVIP;

  packBtns.forEach((btn) => {
    btn.onclick = () => {
      const state = window.haloState;
      const packKey = btn.dataset.pack;
      const { results, reason } = pullPack(state, packKey);
      if (!results.length && reason) notify(reason);
      else renderGachaResult(results);
      saveState(state);
      render();
    };
  });

  if (chatForm) {
    chatForm.onsubmit = (e) => {
      e.preventDefault();
      sendChat(chatInput.value);
      chatInput.value = "";
    };
  }

  if (relayToggle)
    relayToggle.onchange = (e) => {
      const state = window.haloState;
      state.online.enabled = e.target.checked;
      if (state.online.enabled) connectRelay();
      saveState(state);
      render();
    };
  if (relayUrl)
    relayUrl.onchange = (e) => {
      const state = window.haloState;
      state.online.url = e.target.value;
      saveState(state);
    };
  if (relayRoom)
    relayRoom.onchange = (e) => {
      const state = window.haloState;
      state.online.room = e.target.value;
      saveState(state);
    };
}

function init() {
  window.haloState = loadState();
  const state = window.haloState;
  document.getElementById("pilot-name").value = state.profile.pilot;
  document.getElementById("quest").value = state.profile.quest;
  document.getElementById("seed").value = state.profile.seed;
  document.getElementById("difficulty").value = state.profile.difficulty;
  document.getElementById("mode").value = state.profile.mode;
  document.getElementById("relay-enabled").checked = state.online.enabled;
  document.getElementById("relay-url").value = state.online.url;
  document.getElementById("relay-room").value = state.online.room;
  bindUI();
  render();
  if (state.online.enabled) connectRelay();
}

window.addEventListener("DOMContentLoaded", init);
