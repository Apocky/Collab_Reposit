# HALO Pocket Labyrinth

A mobile-first, seedable micro game built on the HALO oracle. Run the HTML offline, seed-sync with friends, and track Momentum/Aegis/Depth/Doom as you descend.

## Play it

Open `index.html` in a mobile browser or add it to your home screen for a lightweight PWA-style shell. State is stored locally in your browser.

To ship a single self-contained file to your phone, run `npm run build` and send `dist/index.html` to your device (AirDrop, email, cloud drive, or a local file server). The build inlines `halo.js` so it loads cleanly from local storage on iOS/Android without needing a separate script.

## Loop

1. Set your handle, quest, mode (solo, co-op, gauntlet), difficulty, and optional seed.
2. Tap **Start / Resume** to lock it in.
3. Tap **Draw next beat** for each turn; the oracle adjusts Momentum, Aegis, Depth, and Doom and gives you a prompt.
4. **Bank rewards / End run** when you want to cash out, or **Reset Run** to start fresh.

Seeds are deterministic—share them so friends can mirror the same Labyrinth on their phones.
