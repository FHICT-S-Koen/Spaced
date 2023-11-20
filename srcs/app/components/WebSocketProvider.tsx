// Create WebSocket connection.
import { io } from 'socket.io-client';
import type { JSXElement } from 'solid-js';
import { useContext, createSignal, Show } from 'solid-js';
import { createContext } from 'solid-js';

const [connected, setConnected] = createSignal(false);

const url = new URL(`http://${window.location.host}`);
const socket = io(url, {
  autoConnect: false,
});

function onSubmit(event: SubmitEvent) {
  event.preventDefault();
  const target = event.target as HTMLFormElement | null;
  if (target) {
    const formData = new FormData(target);
    socket.auth = { user: formData.get('user') };
    socket.connect();
    setConnected(true);
  }
}

const context = { socket };

const WebSocketContext = createContext(context);

type WebSocketProps = {
  children: JSXElement;
};

export function WebSocketProvider(props: WebSocketProps) {
  return (
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore Ignore since getters and setters are already present
    <WebSocketContext.Provider>
      {
        <>
          <Show when={!connected()}>
            <form onSubmit={onSubmit} class="absolute z-50 p-2">
              <input placeholder="user" name="user" class="m-2 rounded"></input>
            </form>
          </Show>
          {props.children}
        </>
      }
    </WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  return useContext(WebSocketContext);
}
