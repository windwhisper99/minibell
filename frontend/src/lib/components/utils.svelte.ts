export interface CreateTransformationProps<T> {
  input: () => T;
  update?: (value: T) => void;
}

export function createTransformation<T>(config: CreateTransformationProps<T>) {
  const output = $derived.by(() => {
    return config.input();
  });

  return {
    get value() {
      return output;
    },
    set value(value: T) {
      if (config.update) config.update(value);
    },
  };
}
