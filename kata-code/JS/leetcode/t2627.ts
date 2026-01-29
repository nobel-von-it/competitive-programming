type F = (...args: number[]) => void;

var debounce = (fn: F, t: number): F => {
  let timeId: number;

  return (...args) => {
    clearTimeout(timeId);
    timeId = setTimeout(() => {
      fn(...args);
    }, t);
  };
};
