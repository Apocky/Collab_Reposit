const WebSocket = require('ws');

const PORT = process.env.PORT || 8787;
const rooms = new Map();

function broadcast(roomId, payload, skip) {
  const room = rooms.get(roomId);
  if (!room) return;
  const data = JSON.stringify(payload);
  for (const peer of room) {
    if (peer === skip || peer.readyState !== WebSocket.OPEN) continue;
    peer.send(data);
  }
}

function prune(roomId) {
  const room = rooms.get(roomId);
  if (!room) return;
  for (const peer of [...room]) {
    if (peer.readyState !== WebSocket.OPEN) room.delete(peer);
  }
  if (!room.size) rooms.delete(roomId);
}

const wss = new WebSocket.Server({ port: PORT });

wss.on('connection', (ws) => {
  ws.meta = { room: null, player: 'anon' };

  ws.on('message', (msg) => {
    let data;
    try {
      data = JSON.parse(msg);
    } catch (err) {
      return;
    }

    if (data.type === 'join') {
      const roomId = (data.room || 'lobby').slice(0, 32);
      const player = (data.player || 'anon').slice(0, 32);
      ws.meta = { room: roomId, player };
      if (!rooms.has(roomId)) rooms.set(roomId, new Set());
      rooms.get(roomId).add(ws);
      broadcast(roomId, { type: 'system', message: `${player} joined ${roomId}` }, ws);
      ws.send(JSON.stringify({ type: 'welcome', room: roomId, peers: rooms.get(roomId).size }));
      return;
    }

    if (!ws.meta.room) return;

    if (data.type === 'sync') {
      broadcast(ws.meta.room, { type: 'sync', from: ws.meta.player, payload: data.payload }, ws);
      return;
    }

    if (data.type === 'chat') {
      broadcast(ws.meta.room, { type: 'chat', from: ws.meta.player, message: data.message }, ws);
      return;
    }
  });

  ws.on('close', () => {
    const { room, player } = ws.meta;
    if (room && rooms.has(room)) {
      broadcast(room, { type: 'system', message: `${player} left ${room}` }, ws);
      rooms.get(room).delete(ws);
      prune(room);
    }
  });
});

wss.on('listening', () => {
  console.log(`HALO relay server running on ws://localhost:${PORT}`);
});
