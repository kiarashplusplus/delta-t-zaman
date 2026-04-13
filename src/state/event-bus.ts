type EventCallback<T = any> = (data: T) => void;

interface EventBusMap {
  [key: string]: EventCallback[];
}

const subscribers: EventBusMap = {};

export function subscribe<T>(event: string, callback: EventCallback<T>): () => void {
  if (!subscribers[event]) {
    subscribers[event] = [];
  }
  subscribers[event].push(callback as EventCallback);
  
  return () => {
    unsubscribe(event, callback);
  };
}

export function publish<T>(event: string, data?: T): void {
  if (!subscribers[event]) return;
  subscribers[event].forEach(callback => callback(data as any));
}

export function unsubscribe<T>(event: string, callback: EventCallback<T>): void {
  if (!subscribers[event]) return;
  subscribers[event] = subscribers[event].filter(cb => cb !== callback as unknown);
}
