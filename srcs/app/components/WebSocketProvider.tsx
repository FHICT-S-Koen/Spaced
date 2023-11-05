// Create WebSocket connection.
import { io } from 'socket.io-client';
import type { JSXElement } from 'solid-js';
import { useContext } from 'solid-js';
import { createContext } from 'solid-js';

const socket = io(
  // Math.random() > 0.5 ? 'ws://localhost:8080' : 'ws://localhost:8081',
  new URL(`ws://${window.location.host}`),
  { withCredentials: false },
);
// let socket = new WebSocket('ws://localhost:8080');
// socket.addEventListener('close', (msg) => {
//   console.log('Conection closed');
//   try {
//     // switch to long-polling
//     socket = new WebSocket('ws://localhost:8080');
//   } catch {
//     // try again in xxx time
//     // add increasing connection timeouts
//   }
//   // connected? > stop long-polling
// });

const context = { socket };

const WebSocketContext = createContext(context);

type WebSocketProps = {
  children: JSXElement;
};

export function WebSocketProvider(props: WebSocketProps) {
  return (
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore Ignore since getters and setters are already present
    <WebSocketContext.Provider>{props.children}</WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  return useContext(WebSocketContext);
}
