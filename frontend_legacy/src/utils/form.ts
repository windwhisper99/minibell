import { Action, useAction } from "@solidjs/router";
import { Accessor, createSignal } from "solid-js";
import { createStore, SetStoreFunction, unwrap } from "solid-js/store";

export type UseForm<T> = {
  form: T;
  loading: Accessor<boolean>;
  formField: (e: Event) => void;
  formFieldWith: (name: string | string[]) => (value: any) => void;
  updateForm: SetStoreFunction<T>;
  submitForm: (submitType?: string) => (e: Event) => Promise<void>;
};

/**
 * Assign a value to an object at a given path.
 * Path can be object key or array index.
 * If is array index, it must be a string.
 */
function setValue(obj: any, path: string[], value: (prev: any) => any) {
  let ref = obj;
  for (let i = 0; i < path.length; i++) {
    const key = path[i];

    if (i === path.length - 1) {
      ref[key] = value(ref[key]);
    } else {
      if (!(key in ref)) {
        if (isNaN(Number(path[i + 1]))) {
          ref[key] = {};
        } else {
          ref[key] = [];
        }
      }

      ref = ref[key];
    }
  }
}

/**
 * Get value from an input element.
 * If input string is empty, return null.
 * If input type is datetime-local, return value as second timestamp number.
 */
function getValue(ele: HTMLInputElement) {
  if (ele.type === "datetime-local") return new Date(ele.value).getTime();
  if (ele.type === "checkbox") return ele.checked;
  if (ele.type === "number") return parseInt(ele.value);

  // If input have [data-array] attribute, split value by comma
  if (ele.dataset.array) {
    if (ele.value === "") return [];
    return ele.value.split(",").map((v) => v.trim());
  }

  if (ele.value === "") return null;
  return ele.value;
}

export function useForm<T extends object>(
  initial: T,
  action: Action<any, any>
): UseForm<T> {
  const [form, setForm] = createStore<T>(initial);
  const [loading, setLoading] = createSignal(false);
  const submitAction = useAction(action);

  const formField = (e: Event) => {
    const ele = e.target as HTMLInputElement;
    const path = ele.name.split(".");
    const value = getValue(ele);

    if (value === null) return;
    setFormField(path, value);
  };

  const formFieldWith = (name: string | string[]) => (value: any) => {
    const path = Array.isArray(name) ? name : name.split(".");
    setFormField(path, value);
  };

  const setFormField = (name: string | string[], value: any) => {
    const path = Array.isArray(name) ? name : name.split(".");

    setForm((prevForm) => {
      const newForm = { ...prevForm };
      setValue(newForm, path, (prev) => {
        if (typeof value === "function") return value(prev);
        return value;
      });
      return newForm;
    });
  };

  const submitForm = (submitType?: string) => async (e: Event) => {
    e.preventDefault();

    setLoading(true);

    const data = unwrap(form);
    if (submitType) {
      await submitAction({ ...data, submitType });
    } else {
      await submitAction(data);
    }

    setLoading(false);
  };

  return {
    form,
    formField,
    formFieldWith,
    updateForm: setForm,
    submitForm,
    loading,
  };
}
