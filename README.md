# HALO Meta Labyrinth — Archetype oracle roguelike

Mobile-first HALO rebuilt as a meta-divination card roguelike with a Wu Xing elemental matchup system, gacha economy, and optional WebSocket relay for room sync/chat. Everything is bundled for offline play or side-loading into the Android WebView shell.

## Play it

- **Phone sideload:** run `npm run build` and send `dist/index.html` to your phone. The bundle inlines `halo.js` so it runs offline.
- **Home screen:** open `index.html` in mobile Safari/Chrome and “Add to Home Screen.” State stays on-device with an in-memory fallback if storage is blocked.
- **Android APK (WebView shell):** build with Android Studio after running `./scripts/sync_android_assets.sh` to copy the latest `dist/index.html` into `android-app/app/src/main/assets/`.

## Core loop

1. Set pilot, quest, seed, difficulty, and mode from the main menu. Seeds are deterministic so squads can mirror runs.
2. Build a 12-card deck from Wu Xing archetype cards. Elemental advantage follows the controlling cycle (Wood>Earth>Water>Fire>Metal>Wood).
3. Enter a run: draw an oracle-driven encounter, play archetype cards from your hand, then resolve the beat. Momentum/Aegis/Doom govern survival; Depth tracks progress.
4. Cash out to bank Credits/Embers/Shards or crash if Doom/Aegis fail. Credits buy Pulse packs; Embers/Shards feed Radiant/Axis pulls and VIP rarity boosts.

## Multiplayer relay

- Start the relay locally: `npm install` then `npm run server` (defaults to `ws://localhost:8787`).
- In the client, toggle **Enable relay**, set the relay URL and room (defaults to the seed), and play. Depth pings and chat flow automatically once connected.

## Build

- `npm run build` – inlines `halo.js` into `dist/index.html` and copies assets for sideloading or the Android shell.
- `npm run server` – lightweight WebSocket relay for squad sync/chat (optional).

## Serverless API (DynamoDB) deployment note

The Lambda handler in `halo_lambda/index.js` auto-corrects placeholder or malformed regions (e.g., `MY_AWS_REGION`, `LOCAL`, blank) to `us-east-1` so builds do not fail when a default value is left unchanged. For production, set `AWS_REGION` or `AWS_DEFAULT_REGION` to a valid AWS region pattern (for example, `us-west-2`, `us-gov-west-1`, or `cn-north-1`) before deploying the function.
