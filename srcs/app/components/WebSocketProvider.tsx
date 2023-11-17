// Create WebSocket connection.
import { io } from 'socket.io-client';
import type { JSXElement } from 'solid-js';
import { useContext, createSignal, Show } from 'solid-js';
import { createContext } from 'solid-js';

const [user, setUser] = createSignal('');

let socket = io(new URL(`ws://${window.location.host}`), {
  autoConnect: false,
  withCredentials: false,
  auth: {
    // eslint-disable-next-line solid/reactivity
    user: user(),
  },
});
function onSubmit(event: SubmitEvent) {
  const formData = new FormData(event.target);
  setUser(formData.get('user'));
  socket = io(new URL(`ws://${window.location.host}`), {
    // autoConnect: false,
    withCredentials: false,
    auth: {
      user: user(),
    },
  });
}

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
    <WebSocketContext.Provider>
      {
        <>
          <Show when={!user()}>
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
