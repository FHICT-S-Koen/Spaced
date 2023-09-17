import { invoke } from '@tauri-apps/api/tauri';
import type { Setter } from 'solid-js';
import { createSignal } from 'solid-js';

import type { Item } from './App.js';
import type { Vec2D } from '../lib/vector.js';

type ContainerProperties = {
  id: number;
  text: string;
  translation: Vec2D;
  scale: number;
  mutate: Setter<Item[] | undefined>;
};

export function Container(properties: ContainerProperties) {
  let ref!: HTMLDivElement;
  const [toggle, setToggle] = createSignal(false);
  function handleKeyUp(event: KeyboardEvent) {
    console.log(properties.id);
    if (event.key === 'Delete') {
      invoke('delete', { id: properties.id }).then(() => {
        properties.mutate((prev) => {
          console.log(prev);
          console.log(prev!.findIndex(({ id }) => id === properties.id));

          const items = prev;
          console.log(items);
          items?.pop();
          // items!.splice(
          //   prev!.findIndex(({ id }) => id === properties.id),
          //   1,
          // );
          return items;
        });
      });
    }
  }
  return (
    <div
      ref={ref}
      onKeyUp={handleKeyUp}
      onClick={() => setToggle(true)}
      onBlur={() => setToggle(false)}
      class="absolute h-12 w-12 rounded border bg-white"
      tabIndex={0}
      style={{
        'border-color': toggle() ? 'black' : '',
        'transform-origin': 'top left',
        translate: `
          ${properties.translation.x}px
          ${-properties.translation.y}px
        `,
        scale: `${properties.scale}`,
      }}
    >
      {properties.text}
    </div>
  );
}
