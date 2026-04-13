import { publish } from './event-bus';

let frameId: number = 0;
let lastTick: number = 0;

function tick() {
  const now = Date.now();
  if (now - lastTick >= 1000) {
    publish('TIME_TICK', now);
    lastTick = now;
  }
  frameId = requestAnimationFrame(tick);
}

export function startTimeTicker() {
  if (frameId) return; // Already started
  lastTick = Date.now();
  publish('TIME_TICK', lastTick); // initial tick
  frameId = requestAnimationFrame(tick);

  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      cancelAnimationFrame(frameId);
      frameId = 0;
    } else {
      if (!frameId) {
        lastTick = Date.now();
        publish('TIME_TICK', lastTick);
        frameId = requestAnimationFrame(tick);
      }
    }
  });
}
