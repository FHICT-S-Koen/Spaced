import { useContext, createContext, type JSXElement } from 'solid-js';
import { createStore } from 'solid-js/store';

const [selections, setSelections] = createStore([]);

const context = {
  selections,
  setSelections,
};

const SelectiontContext = createContext(context);

type ProviderProps = {
  children: JSXElement;
};

export function SelectionProvider(props: ProviderProps) {
  return (
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore Ignore since getters and setters are already present
    <SelectiontContext.Provider>{props.children}</SelectiontContext.Provider>
  );
}

export function useSelection() {
  return useContext(SelectiontContext);
}
