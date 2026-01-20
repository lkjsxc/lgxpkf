export const readStorage = (key: string): string | null => {
  try {
    return localStorage.getItem(key);
  } catch (_) {
    return null;
  }
};

export const writeStorage = (key: string, value: string | null): void => {
  try {
    if (value === null) {
      localStorage.removeItem(key);
    } else {
      localStorage.setItem(key, value);
    }
  } catch (_) {
    return;
  }
};
