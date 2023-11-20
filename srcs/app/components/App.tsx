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
import { debounce, getBoundingBox, throttle } from '../lib/utils.js';
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
  socket.on('item:updates', (item: Item) => {
    // eslint-disable-next-line unicorn/prefer-spread
    setItems((value) =>
      value.map((i) => {
        if (item.id === i.id) {
          item.schema = i.schema;
        }
        return item;
      }),
    );
  });

  createEffect(
    on(
      absoluteViewportPosition,
      throttle(async (pos: Vec2D) => {
        const bb = getBoundingBox(pos);
        const response = (await socket.emitWithAck('item:get_nearby', bb)) as
          | Item
          | Item[];
        if (response) {
          setItems((items) => {
            // eslint-disable-next-line unicorn/prefer-array-flat
            const newItems = []
              // eslint-disable-next-line unicorn/prefer-array-flat, unicorn/prefer-spread
              .concat(response)
              .filter((item2) => !items.some((item1) => item1.id === item2.id));
            // eslint-disable-next-line unicorn/prefer-spread
            return items.concat(newItems);
          });
        }
      }, 200),
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
      new Vec2D(window.innerWidth / 2 - 24, -window.innerHeight / 2 - 24),
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
