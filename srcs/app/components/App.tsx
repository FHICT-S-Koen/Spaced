import { invoke } from '@tauri-apps/api/tauri';
import { For, createEffect, createSignal, on } from 'solid-js';

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
  const [items, setItems] = createSignal<Item[]>([]);

  socket.on('item:updates', (item: Item | Item[]) => {
    // eslint-disable-next-line unicorn/prefer-spread
    setItems((value) => value.concat(item));
  });

  createEffect(
    on(
      absoluteViewportPosition,
      throttle(async (pos: Vec2D) => {
        const bb = {
          xmin: Math.round(pos.x),
          ymin: -Math.round(pos.y),
          xmax: Math.round(pos.x) + Math.round(window.innerWidth / scalar()),
          ymax: -Math.round(pos.y) + Math.round(window.innerHeight / scalar()),
        };
        const response: Item[] = await socket.emitWithAck(
          'item:get_nearby',
          bb,
        );
        // eslint-disable-next-line unicorn/prefer-spread
        setItems((value) => value.concat(response));
      }, 300),
    ),
  );

  function handlePointerMove(event: PointerEvent) {
    pointerDelta = new Vec2D(event.clientX, -event.clientY)
      .sub(lastRelativePointerPosition)
      .div(scalar());
    if (event.shiftKey && event.buttons === 1) {
      const selected = getSelected();
      const moved_items = items().map((item) =>
        selected.has(item.id!)
          ? {
              ...item,
              x: item.x + pointerDelta.x,
              y: item.y + pointerDelta.y,
            }
          : item,
      );
      for (const item of moved_items!
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
      setItems(moved_items);
    } else if (event.buttons === 1) {
      setAbsoluteViewportPosition((prev) => prev.add(pointerDelta.neg()));
    }
    lastRelativePointerPosition = new Vec2D(event.clientX, -event.clientY);
  }
  function handleClick() {
    const absolute = relativeToAbsolute(
      new Vec2D(window.innerWidth / 2 - 24, -(window.innerHeight / 2 - 24)),
      absoluteViewportPosition(),
      scalar(),
    );

    socket
      .emitWithAck('item:create', {
        id: 0,
        x: Math.floor(absolute.x),
        y: Math.floor(absolute.y),
        w: 0,
        h: 0,
        name: 'test',
        schema: 'test',
      } as Item)
      .then((response: Item) => {
        // eslint-disable-next-line unicorn/prefer-spread
        setItems((value) => value.concat(response));
      });
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
                  onClick={handleClick}
                  class="absolute bottom-1 left-1 z-50 rounded border-2 border-slate-600 bg-slate-500 text-white shadow"
                >
                  Create 🚀
                </button>
                <For each={items()}>
                  {(item, index) => (
                    <Container
                      index={index()}
                      id={item.id!}
                      {...item}
                      setItems={setItems}
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
