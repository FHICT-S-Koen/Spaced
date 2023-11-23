// import { invoke } from '@tauri-apps/api/tauri';
import { createMemo, createSignal, type Setter } from 'solid-js';

// import { useContextmenu } from './ContextmenuProvider.js';
import { useSelection } from './SelectionProvider.js';
import { useViewport } from './ViewportProvider.js';
import { useWebSocket } from './WebSocketProvider.js';
import { type Item } from '../lib/types.js';
import { absoluteToRelative, Vec2D } from '../lib/vector.js';

type ContainerProps = {
  index: number;
  setItems: Setter<Item[]>;
} & Item;

export function Container(properties: ContainerProps) {
  // eslint-disable-next-line solid/reactivity
  const [schema, setSchema] = createSignal(properties.schema);
  const { absoluteViewportPosition, scalar } = useViewport();
  const { getSelected, holdingCtrl, holdingShift, register, unregister } =
    useSelection();
  const { socket } = useWebSocket();
  const selected = createMemo(() => getSelected().has(properties.id!));
  const translation = createMemo(() =>
    absoluteToRelative(
      new Vec2D(properties.x, properties.y),
      absoluteViewportPosition(),
      scalar(),
    ),
  );

  // function handleContextmenu(event: MouseEvent) {
  //   event.preventDefault();
  //   setAbsoluteMenuPosition(translation());
  //   setIsOpen((prev) => !prev);
  //   setCallback(() => {
  //     invoke('update', {
  //       ...properties,
  //       color: selectedMenuItem(),
  //       data: ref.textContent,
  //     }).then(() => {
  //       properties.mutate((prev) => {
  //         const items = [...prev!];
  //         items[properties.index] = {
  //           ...items[properties.index],
  //           color: selectedMenuItem() as string | undefined,
  //           data: ref.textContent!,
  //         };
  //         return items;
  //       });
  //     });
  //   });
  // }
  let ref!: HTMLDivElement;
  function handleClick() {
    register(properties.id!);
  }
  function handleBlur() {
    if (holdingCtrl() || holdingShift()) {
      return;
    }
    unregister(properties.id!);
  }

  // let savedSelection: Range = null;

  // const saveSelection = () => {
  //   const selection = window.getSelection()!;
  //   if (selection.rangeCount > 0) {
  //     savedSelection = selection.getRangeAt(0).cloneRange();
  //   }
  // };

  // // Function to restore the saved selection
  // const restoreSelection = () => {
  //   const selection = window.getSelection()!;
  //   if (savedSelection && selection.rangeCount > 0) {
  //     selection.removeAllRanges();
  //     selection.addRange(savedSelection);
  //   }
  // };

  socket.on('item:updates', (item: Item) => {
    // setItems((items) =>
    //   items.map((i) => {
    //     if (item.id === i.id) {
    //       i = { ...i, schema: item.schema };
    //     }
    //     return i;
    //   }),
    // );
    setSchema(item.schema);
    // restoreSelection();
  });

  function handleKeyUp() {
    // if (event.shiftKey && event.key === 'Delete') {
    //   invoke('delete', { id: properties.id }).then(() => {
    //     properties.mutate((prev) => {
    //       // TODO: needs to be copied?
    //       const items = [...prev!];
    //       items?.splice(properties.index, 1);
    //       return items;
    //     });
    //   });
    // } else if (event.key === 'Escape') {
    //   ref.blur();
    // }
    // saveSelection();
    socket.emit('item:update_inner', {
      ...properties,
      schema: ref.textContent!,
    } as Item);
    // setSchema(ref.textContent!);
    // restoreSelection();

    // .then((response: Item) => {
    //   // eslint-disable-next-line unicorn/prefer-spread
    //   setItems((value) => value.concat(response));
    // });
    // invoke('update', {
    //   ...properties,
    //   // TODO: create class with normalize method for items...
    //   x: Math.floor(properties.x),
    //   y: Math.floor(properties.y),
    //   data: ref.textContent,
    // });
  }
  // eslint-disable-next-line unicorn/consistent-function-scoping
  function handleBeforeInput(event: InputEvent) {
    if (event.inputType === 'insertParagraph') {
      event.preventDefault();
      const text = document.createTextNode('\n');
      const selection = window.getSelection();
      if (selection) {
        const range = selection.getRangeAt(0);
        range.deleteContents();
        range.insertNode(text);
        range.setStartAfter(text);
        range.setEndAfter(text);
        selection.removeAllRanges();
        selection.addRange(range);
      }
    }
  }

  return (
    <div
      ref={ref}
      onBeforeInput={handleBeforeInput}
      onInput={handleKeyUp}
      onClick={handleClick}
      onBlur={handleBlur}
      // onContextMenu={handleContextmenu}
      class="absolute min-h-[30px] min-w-[30px] whitespace-pre p-1 outline outline-1"
      tabIndex="0"
      contenteditable={selected()}
      style={{
        'outline-color': selected()
          ? 'black'
          : properties.schema && 'transparent',
        'transform-origin': 'top left',
        'background-color': 'transparent',
        'pointer-events': 'all',
        'line-height': '1rem',
        translate: `
          ${translation().x}px
          ${-translation().y}px
        `,
        scale: `${scalar()}`,
      }}
    >
      {schema()}
    </div>
  );
}
