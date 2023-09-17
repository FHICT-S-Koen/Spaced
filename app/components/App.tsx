import { invoke } from '@tauri-apps/api/tauri';
import type { Setter } from 'solid-js';
import { createResource, createSignal, For } from 'solid-js';

import { Background } from './Background.js';
import { Container } from './Container.js';
import {
  absoluteToRelative,
  relativeToAbsolute,
  scaleViewportOutFrom,
  scaleViewportUpTo,
  Vec2D,
} from '../lib/vector.js';

let pointerDelta = new Vec2D(0, 0);
let lastRelativePointerPosition = new Vec2D(0, 0);

const [factor] = createSignal(1.2);
const [scalar, setScalar] = createSignal(1);
const [absoluteViewportPosition, setAbsoluteViewportPosition] = createSignal(
  new Vec2D(0, 0),
);

function handlePointerMove(event: PointerEvent) {
  pointerDelta = new Vec2D(event.clientX, -event.clientY)
    .sub(lastRelativePointerPosition)
    .div(scalar());
  if (event.buttons === 1) {
    setAbsoluteViewportPosition((prev) => prev.add(pointerDelta.neg()));
  }
  lastRelativePointerPosition = new Vec2D(event.clientX, -event.clientY);
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

export type Item = {
  id?: number;
  x: number;
  y: number;
  text: string;
};

function handleClick(event: MouseEvent, mutate: Setter<Item[] | undefined>) {
  const absolute = relativeToAbsolute(
    new Vec2D(window.innerWidth / 2 - 24, -(window.innerHeight / 2 - 24)),
    absoluteViewportPosition(),
    scalar(),
  );

  invoke('insert_into', {
    x: Math.floor(absolute.x),
    y: Math.floor(absolute.y),
    text: 'test',
  }).then((note) => mutate((prev) => [...(prev ?? []), note] as Item[]));
}

export function App() {
  const [data, { mutate }] = createResource<Item[]>(() => invoke('select'));

  return (
    <div
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
          {(item) => (
            <Container
              id={item.id!}
              text={item.text}
              translation={absoluteToRelative(
                new Vec2D(item.x, item.y),
                absoluteViewportPosition(),
                scalar(),
              )}
              scale={scalar()}
              mutate={mutate}
            />
          )}
        </For>
      </main>
    </div>
  );
}
