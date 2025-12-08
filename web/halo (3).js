// HALO Meta-Oracle skeleton script.
// This script defines the HALO core engine (dice roll), plugin hooks, and local storage.
// The detailed symbol definitions (Tarot cards, runes, I Ching, etc.) and deeper synthesis logic
// should be filled in later.

(function() {
  const STORAGE_KEY_PROFILE = 'halo_profile_v1';
  const STORAGE_KEY_READINGS = 'halo_meta_readings_v1';
  // Replace the placeholder with your actual API Gateway invoke URL once deployed.
  const API_URL = 'https://YOUR_API_ID.execute-api.YOUR_REGION.amazonaws.com/readings';

  // Axis definitions
  const AXES = ['Body & Hardware','Mind & Narrative','Heart & Relationships','Domain & Magic'];
  const VECTORS = ['Ingress','Grow','Stabilize','Release','Transmute','Witness'];
  const TIMELINES = [
    'Right now','Today','This week','This month','This season','This year',
    '1–3 years','3–7 years','7–20 years','Lifetime','Generational','Meta'
  ];
  const ARCHETYPES = [
    'The Fool','The Magician','The Healer','The Warrior','The Shield','The Hermit','The Lover',
    'The Trickster','The Phoenix','The Architect','The Bridge','The Teacher','The Explorer',
    'The Mirror','The Guardian','The Key','The Storm','The Weaver','The Sovereign'
  ];

  // Basic utility to roll a die
  function rollDie(sides) {
    return Math.floor(Math.random() * sides) + 1;
  }

  // Load/save profile
  function loadProfile() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY_PROFILE)) || {};
    } catch (err) {
      return {};
    }
  }

  function saveProfile(profile) {
    localStorage.setItem(STORAGE_KEY_PROFILE, JSON.stringify(profile));
  }

  // Load/save readings
  function loadReadings() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY_READINGS)) || [];
    } catch (err) {
      return [];
    }
  }

  function saveReadings(readings) {
    localStorage.setItem(STORAGE_KEY_READINGS, JSON.stringify(readings));
  }

  // Fire-and-forget sync to server if API_URL is set.
  async function syncReadingToServer(reading) {
    if (!API_URL || API_URL.includes('YOUR_API_ID')) {
      return;
    }
    try {
      const payload = {
        userId: reading.user?.name || 'anonymous',
        reading
      };
      await fetch(API_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
    } catch (err) {
      console.warn('Sync failed', err);
    }
  }

  // Create a new reading
  function createReading(question, profile) {
    const timestamp = new Date().toISOString();
    const d4 = rollDie(4);
    const d6 = rollDie(6);
    const d12 = rollDie(12);
    const d20 = rollDie(20);

    const reading = {
      id: `${Date.now().toString(36)}-${Math.random().toString(16).slice(2,8)}`,
      question: question || '',
      created_at: timestamp,
      halo: {
        axis: d4,
        vector: d6,
        timeline: d12,
        archetype: d20,
        dice: { d4, d6, d12, d20 }
      },
      user: profile,
      // plugin results can be added here in the future
    };

    const readings = loadReadings();
    readings.push(reading);
    saveReadings(readings);
    syncReadingToServer(reading);
    return reading;
  }

  // Render functions
  function renderHistory() {
    const historyEl = document.getElementById('history');
    const readings = loadReadings();
    if (!readings.length) {
      historyEl.innerHTML = '<p>No readings yet.</p>';
      return;
    }
    const rows = readings.slice().reverse().map(reading => {
      const date = new Date(reading.created_at).toLocaleString();
      const axis = AXES[(reading.halo.axis - 1) % AXES.length];
      const vector = VECTORS[(reading.halo.vector - 1) % VECTORS.length];
      const timeline = TIMELINES[(reading.halo.timeline - 1) % TIMELINES.length];
      const archetype = ARCHETYPES[(reading.halo.archetype - 1) % ARCHETYPES.length];
      return `
        <tr>
          <td>${date}</td>
          <td>${axis}</td>
          <td>${vector}</td>
          <td>${timeline}</td>
          <td>${archetype}</td>
        </tr>
      `;
    }).join('');
    historyEl.innerHTML = `
      <table>
        <thead>
          <tr><th>When</th><th>Axis</th><th>Vector</th><th>Timeline</th><th>Archetype</th></tr>
        </thead>
        <tbody>${rows}</tbody>
      </table>
    `;
  }

  function renderCurrentReading(reading) {
    const container = document.getElementById('current-reading');
    if (!reading) {
      container.innerHTML = '';
      return;
    }
    const halo = reading.halo;
    container.innerHTML = `
      <h3>HALO Roll</h3>
      <p><strong>Axis:</strong> ${AXES[(halo.axis - 1) % AXES.length]} (${halo.dice.d4})</p>
      <p><strong>Vector:</strong> ${VECTORS[(halo.vector - 1) % VECTORS.length]} (${halo.dice.d6})</p>
      <p><strong>Timeline:</strong> ${TIMELINES[(halo.timeline - 1) % TIMELINES.length]} (${halo.dice.d12})</p>
      <p><strong>Archetype:</strong> ${ARCHETYPES[(halo.archetype - 1) % ARCHETYPES.length]} (${halo.dice.d20})</p>
      <p><em>This is a bare-bones skeleton. Interpretations and plugin hooks go here.</em></p>
    `;
  }

  // Initialize UI
  function init() {
    const profile = loadProfile();
    document.getElementById('profile-name').value = profile.name || '';
    document.getElementById('profile-sun').value = profile.sun || '';
    document.getElementById('profile-moon').value = profile.moon || '';
    document.getElementById('profile-rising').value = profile.rising || '';

    document.getElementById('save-profile').addEventListener('click', () => {
      const p = {
        name: document.getElementById('profile-name').value.trim(),
        sun: document.getElementById('profile-sun').value.trim(),
        moon: document.getElementById('profile-moon').value.trim(),
        rising: document.getElementById('profile-rising').value.trim()
      };
      saveProfile(p);
    });

    document.getElementById('roll-btn').addEventListener('click', () => {
      const q = document.getElementById('question').value.trim();
      const p = loadProfile();
      const r = createReading(q, p);
      renderCurrentReading(r);
      renderHistory();
    });

    renderHistory();
  }

  // run
  init();
})();
