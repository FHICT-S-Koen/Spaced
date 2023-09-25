import { invoke } from '@tauri-apps/api/tauri';
import { type Setter, createResource, For } from 'solid-js';

import { Background } from './Background.js';
import { Container } from './Container.js';
import { ContextmenuProvider } from './ContextmenuProvider.js';
import { useViewport, ViewportProvider } from './ViewportProvider.js';
import type { Item } from '../lib/types.js';
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

  function handlePointerMove(event: PointerEvent) {
    pointerDelta = new Vec2D(event.clientX, -event.clientY)
      .sub(lastRelativePointerPosition)
      .div(scalar());
    if (event.buttons === 1) {
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
  const [data, { mutate }] = createResource<Item[]>(() => invoke('select'));
  return (
    <ViewportProvider>
      <ContextmenuProvider>
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
      </ContextmenuProvider>
    </ViewportProvider>
  );
}
