import { invoke } from '@tauri-apps/api/tauri';
import axios from 'axios';
import { type Setter, createResource, For } from 'solid-js';

import { AuthProvider } from './AuthProvider.js';
import { Background } from './Background.js';
import { Container } from './Container.js';
import { ContextmenuProvider } from './ContextmenuProvider.js';
import { useSelection } from './SelectionProvider.js';
import { useViewport, ViewportProvider } from './ViewportProvider.js';
import { useWebSocket, WebSocketProvider } from './WebSocketProvider.js';
import type { Item } from '../lib/types.js';
import { debounce, throttle } from '../lib/utils.js';
import {
  relativeToAbsolute,
  scaleViewportOutFrom,
  scaleViewportUpTo,
  Vec2D,
} from '../lib/vector.js';

export function App() {
  let pointerDelta = new Vec2D(0, 0);
  let lastRelativePointerPosition = new Vec2D(0, 0);
  const {
    absoluteViewportPosition,
    factor,
    scalar,
    setAbsoluteViewportPosition,
    setScalar,
  } = useViewport();
  const { getSelected } = useSelection();
  const { socket } = useWebSocket();
  socket.on('/', (arg) => {
    console.log('On /: ', arg);
  });

  const [data, { mutate }] = createResource<Item[], Vec2D>(
    absoluteViewportPosition,
    throttle(async (output) => {
      console.log(window.innerWidth, window.innerHeight);
      const bb = {
        xmin: Math.round((output as Vec2D).x),
        ymin: -Math.round((output as Vec2D).y),
        xmax:
          Math.round((output as Vec2D).x) +
          Math.round(window.innerWidth / scalar()),
        ymax:
          -Math.round((output as Vec2D).y) +
          Math.round(window.innerHeight / scalar()),
      };
      console.log(bb);
      // eslint-disable-next-line unicorn/no-await-expression-member
      return (await axios.post('http://localhost:8081/item', bb)).data;
    }, 300),
  );

  function handlePointerMove(event: PointerEvent) {
    pointerDelta = new Vec2D(event.clientX, -event.clientY)
      .sub(lastRelativePointerPosition)
      .div(scalar());
    if (event.shiftKey && event.buttons === 1) {
      const selected = getSelected();
      const items = data.latest?.map((item) =>
        selected.has(item.id!)
          ? {
              ...item,
              x: item.x + pointerDelta.x,
              y: item.y + pointerDelta.y,
            }
          : item,
      );
      for (const item of items!
        .map((item) => ({
          ...item,
          x: Math.floor(item.x),
          y: Math.floor(item.y),
        }))
        .filter((item) => selected.has(item.id!))) {
        debounce(async () => {
          await invoke('update', item);
        }, 100)();
      }
      mutate(items);
    } else if (event.buttons === 1) {
      setAbsoluteViewportPosition((prev) => prev.add(pointerDelta.neg()));
    }
    lastRelativePointerPosition = new Vec2D(event.clientX, -event.clientY);
  }
  function handleClick(event: MouseEvent, mutate: Setter<Item[] | undefined>) {
    const absolute = relativeToAbsolute(
      new Vec2D(window.innerWidth / 2 - 24, -(window.innerHeight / 2 - 24)),
      absoluteViewportPosition(),
      scalar(),
    );

    invoke('insert', {
      x: Math.floor(absolute.x),
      y: Math.floor(absolute.y),
      data: '',
    }).then((note) => mutate((prev) => [...(prev ?? []), note] as Item[]));
  }
  function handleWheel(event: WheelEvent) {
    if (event.deltaY < 0 && scalar() < 160) {
      setAbsoluteViewportPosition((prev) =>
        scaleViewportUpTo(
          new Vec2D(event.clientX, -event.clientY),
          prev,
          scalar(),
          factor(),
        ),
      );
      setScalar((prev) => prev * factor());
    } else if (event.deltaY > 0 && scalar() > 0.01) {
      setAbsoluteViewportPosition((prev) =>
        scaleViewportOutFrom(
          new Vec2D(event.clientX, -event.clientY),
          prev,
          scalar(),
          factor(),
        ),
      );
      setScalar((prev) => prev / factor());
    }
  }

  return (
    <AuthProvider>
      <ViewportProvider>
        <ContextmenuProvider>
          <WebSocketProvider>
            <button
              class="absolute z-50"
              onClick={() => socket.emit('message', 'Send message')}
            >
              Send message
            </button>
            <div
              id="viewport"
              class="h-full w-full overflow-hidden"
              onPointerMove={handlePointerMove}
              onWheel={handleWheel}
            >
              {/* TODO: resolve FOUC */}
              <Background />
              <main class="absolute h-full w-full">
                <button
                  onClick={(event) => handleClick(event, mutate)}
                  class="absolute bottom-1 left-1 z-50 rounded border-2 border-slate-600 bg-slate-500 text-white shadow"
                >
                  Create 🚀
                </button>
                <For each={data.latest}>
                  {(item, index) => (
                    <Container
                      index={index()}
                      id={item.id!}
                      {...item}
                      mutate={mutate}
                    />
                  )}
                </For>
              </main>
            </div>
          </WebSocketProvider>
        </ContextmenuProvider>
      </ViewportProvider>
    </AuthProvider>
  );
}
