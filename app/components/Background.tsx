import { createMemo } from 'solid-js';

import { type Vec2D } from '../lib/vector.js';

type BackgroundProps = {
  pos: Vec2D;
  scalar: number;
};

export function Background(props: BackgroundProps) {
  const dist = 25;
  const offsetX = createMemo(() => (-props.pos.x % dist) * props.scalar);
  const offsetY = createMemo(() => (props.pos.y % dist) * props.scalar);
  const size = createMemo(() =>
    props.scalar > 1
      ? (dist * props.scalar) /
        (Math.trunc((dist * Math.log2(props.scalar)) / dist / 2) + 1)
      : dist * props.scalar,
  );
  return (
    <svg class="absolute z-0 h-full w-full">
      <pattern
        id="background"
        x={offsetX()}
        y={offsetY()}
        width={size()}
        height={size()}
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
