type Callback<T> = (...args: unknown[]) => Promise<T> | T;

export function debounce<T>(cb: Callback<T>, delay = 1000): Callback<T> {
  let timeout: ReturnType<typeof setTimeout>;
  let mem: T;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(async () => {
      mem = await cb(...args);
      return mem;
    }, delay);
    return mem;
  };
}

export function throttle<T>(cb: Callback<T>, delay = 1000): Callback<T> {
  let lastCallTime = 0;
  let mem: T;
  return async (...args) => {
    const now = Date.now();
    if (now - lastCallTime >= delay) {
      mem = await cb(...args);
      lastCallTime = now;
      return mem;
    }
    return mem;
  };
}
