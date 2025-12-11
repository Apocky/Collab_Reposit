# HALO Pocket Labyrinth — Online Card Roguelike

Mobile-first HALO built out as a roguelike card crawler with gacha, paid VIP boost, and an optional WebSocket relay for multiplayer sync and chat.

## Play it

* **Phone sideload:** run `npm run build` and send `dist/index.html` to your device (AirDrop, email, cloud drive, or a local file server). The build inlines `halo.js` so it runs offline without extra assets.
* **Home screen:** open `index.html` in mobile Safari/Chrome and add it to your home screen for a lightweight PWA shell. State is on-device (localStorage), with an in-memory fallback if storage is blocked.

## Android APK (WebView shell)

1. Run `npm run build` to refresh `dist/index.html`.
2. Sync the HTML into the Android project: `./scripts/sync_android_assets.sh`.
3. Open the `android-app` folder in Android Studio (Giraffe+), let it download the Android Gradle Plugin, and build **app → assembleDebug**.
4. Install `app-debug.apk` on your device. The shell runs offline and keeps relay/WebSocket support when you point it at your server.

## Core loop

1. Define pilot, quest, seed, difficulty, and mode. Seeds are deterministic—share them so squads can mirror the same Labyrinth.
2. Build a 12-card Adventure Deck from your owned collection.
3. Enter a run, draw encounters, play cards from your hand, then tap **Resolve Beat**. Momentum/Aegis/Depth/Doom drive survival; rewards convert to Credits.
4. Cash out to bank rewards or crash when Doom/Aegis fail. Credits buy Pulse packs; Embers buy Radiant pulls and the VIP Blessing (rarity boost). Purchases are simulated for testing only.

## Online relay

An optional, ultra-light relay server lets multiple pilots sync depth/seed metadata and chat while playing the same run.

* Start the relay locally: `npm install` then `npm run server` (defaults to `ws://localhost:8787`).
* In the client, toggle **Enable relay**, set the relay URL and room (use your seed or a custom code), and hit **Start / Resume Run**. Sync + chat messages flow automatically once connected.

## Files

* `index.html` – mobile UI shell.
* `halo.js` – HALO oracle, roguelike loop, gacha/deck logic, and relay client.
* `scripts/build.js` – inlines `halo.js` into `dist/index.html`.
* `scripts/server.js` – minimal WebSocket relay for multiplayer metadata/chat.

All content stays in this repo for easy sideloading. No external CDNs or assets are required.

## Serverless API (DynamoDB) deployment note

The Lambda handler in `halo_lambda/index.js` now auto-corrects placeholder or
malformed regions (e.g., `MY_AWS_REGION`, `LOCAL`, blank) to `us-east-1` so
builds do not fail when a default value is left unchanged. For production, set
`AWS_REGION` or `AWS_DEFAULT_REGION` to a valid AWS region pattern (for
example, `us-west-2`, `us-gov-west-1`, or `cn-north-1`) before deploying the
function.
