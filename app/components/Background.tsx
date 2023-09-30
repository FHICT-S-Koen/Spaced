import { createMemo } from 'solid-js';

import type { Vec2D } from '../lib/vector.js';

type BackgroundProps = {
  pos: Vec2D;
  scalar: number;
};

export function Background(props: BackgroundProps) {
  const dist = 25;
  const x = createMemo(() => (-props.pos.x % dist) * props.scalar);
  const y = createMemo(() => (props.pos.y % dist) * props.scalar);
  const s = createMemo(() => dist * props.scalar);

  return (
    <svg class="absolute z-0 h-full w-full">
      <pattern
        id="background"
        x={x()}
        y={y()}
        width={s()}
        height={s()}
        patternUnits="userSpaceOnUse"
      >
        <circle cx="1.7" cy="1.7" r="1.7" fill="red" />
      </pattern>
      <rect
        x="0"
        y="0"
        fill="url(#background)"
        class="h-full w-full"
        shape-rendering="optimizeSpeed"
      />
    </svg>
  );
}
