import { Accessor, batch, createMemo, createSignal, For, Show } from "solid-js";
import { draftEventAction, IEvent } from "~/utils/api";
import { UseForm, useForm } from "~/utils/form";
import { Switcher } from "./switcher";
import { Tooltip } from "./tooltip";
import { JobsPicker } from "./jobs-picker";
import cn from "classnames";
import { createStore } from "solid-js/store";
import { JOBS, ROLE_GROUPS, ROLES } from "~/utils/jobs";
import { datetimeLocalFormat } from "~/utils/ui";

export interface IDraftEventForm {
  title: string;
  description?: string;
  slots: { jobs: string[] }[];

  startAt: number;
  deadlineAt?: number;
  duration: number;
  isPrivate: boolean;
}

function InfoCard(props: { form: UseForm<IDraftEventForm> }) {
  return (
    <div class="card">
      <div class="form-control">
        <label for="title" class="form-label">
          Title
        </label>
        <input
          type="text"
          id="title"
          name="title"
          class="input"
          value={props.form.form.title}
          onChange={props.form.formField}
        />
      </div>

      <div class="form-control mt-4">
        <label for="description" class="form-label">
          Description
        </label>
        <input
          type="text"
          id="description"
          name="description"
          class="input"
          value={props.form.form.description ?? ""}
          onChange={props.form.formField}
        />
      </div>
    </div>
  );
}

function SlotButton(props: {
  jobs: string[];
  index: number;
  editing: Accessor<number | undefined>;
  onClick: () => void;
}) {
  const title = () => {
    return props.jobs.length >= 1
      ? props.jobs.map((e) => e.toUpperCase()).join(", ")
      : "Any";
  };

  const label = () => {
    if (props.jobs.length === 0) return "Any";
    else if (props.jobs.length === 1) return props.jobs[0].toUpperCase();
    else {
      // Get the role of all jobs
      const roles = props.jobs.map((job) => JOBS[job].role);
      // Deduplicate the roles
      const uniqueRoles = [...new Set(roles)];

      if (uniqueRoles.length === 1) return ROLES[uniqueRoles[0]].name;
      // Get the group of all roles
      const group = uniqueRoles.map((role) => ROLES[role].group);
      // Deduplicate the groups
      const uniqueGroups = [...new Set(group)];

      if (uniqueGroups.length === 1) return ROLE_GROUPS[uniqueGroups[0]].name;
      else return "Mixed";
    }
  };

  return (
    <Tooltip text={title()}>
      {({ setOpen, ref }) => (
        <button
          type="button"
          class={cn(
            "h-14 w-14 rounded-sm border border-slate-300 bg-slate-300 hover:bg-slate-300/80 font-semibold text-sm",
            props.editing() === props.index ? "ring ring-slate-400" : ""
          )}
          ref={ref}
          onMouseEnter={() => setOpen(true)}
          onMouseLeave={() => setOpen(false)}
          onClick={props.onClick}
        >
          {label()}
        </button>
      )}
    </Tooltip>
  );
}

function SlotEditor(props: {
  editing: number;
  slot: { jobs: string[] };
  onRemove: () => void;
}) {
  const slotStore = createMemo(() => {
    const [slot, setSlot] = createStore(props.slot);
    return { slot, setSlot };
  });

  return (
    <div class="mt-4 flex flex-row">
      <div class="w-5 relative before:content-empty before:absolute before:top-0 before:bottom-0 before:w-2 before:bg-slate-100"></div>

      <div>
        <JobsPicker
          jobs={slotStore().slot.jobs}
          onChanges={(jobs) => slotStore().setSlot("jobs", jobs)}
        />
        <div class="flex-1">
          <div class="mt-6 space-x-2">
            <Show when={props.editing !== 0}>
              <button
                type="button"
                class="btn btn-danger"
                onClick={() => props.onRemove()}
              >
                Remove
              </button>
            </Show>

            <button
              type="button"
              class="btn"
              onClick={() => slotStore().setSlot("jobs", [])}
            >
              Any
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

function SlotCard(props: { form: UseForm<IDraftEventForm> }) {
  const [editing, setEditing] = createSignal<number>();
  const [slots, setSlots] = createStore(props.form.form.slots);

  return (
    <div class="card mt-6">
      <div class="card-header">
        <h2 class="card-title">Slots</h2>
      </div>

      <div class="flex flex-row gap-3 flex-wrap">
        <For each={slots}>
          {(value, i) => (
            <SlotButton
              jobs={value.jobs}
              editing={editing}
              index={i()}
              onClick={() =>
                setEditing((current) => {
                  if (current === i()) return undefined;
                  return i();
                })
              }
            />
          )}
        </For>

        <button
          type="button"
          class="h-14 w-14 rounded-sm border bg-slate-50 hover:bg-slate-100 flex items-center justify-center"
          onClick={() => {
            batch(() => {
              setSlots((prev) => [...prev, { jobs: [] }]);
              setEditing(slots.length - 1);
            });
          }}
        >
          <i class="i-tabler-plus?auto w-6 h-6"></i>
        </button>
      </div>

      <Show when={editing() !== undefined}>
        <SlotEditor
          editing={editing()!}
          slot={slots[editing()!]}
          onRemove={() => {
            batch(() => {
              setSlots((prev) => prev.filter((_, i) => i !== editing()));
              setEditing(undefined);
            });
          }}
        />
      </Show>
    </div>
  );
}

function ScheduleCard(props: { form: UseForm<IDraftEventForm> }) {
  return (
    <div class="card mt-6">
      <div class="card-header">
        <h2 class="card-title">Schedule</h2>
      </div>

      <div class="flex flex-col md:flex-row gap-4">
        <div class="flex-1 form-control">
          <label for="startAt" class="form-label">
            Start At
          </label>
          <input
            id="startAt"
            type="datetime-local"
            name="startAt"
            class="input"
            value={datetimeLocalFormat(props.form.form.startAt)}
            onChange={props.form.formField}
            required
          />
        </div>

        <div class="flex-1 form-control">
          <label for="deadlineAt" class="form-label">
            Deadline
          </label>
          <input
            id="deadlineAt"
            type="datetime-local"
            name="deadlineAt"
            class="input"
            value={datetimeLocalFormat(props.form.form.deadlineAt ?? 0)}
            onChange={props.form.formField}
          />
        </div>
      </div>

      <div class="form-control mt-4">
        <label for="duration" class="form-label">
          Duration
        </label>
        <input
          id="duration"
          type="number"
          name="duration"
          class="input md:max-w-lg"
          min="15"
          max="1440"
          value={props.form.form.duration}
          onChange={props.form.formField}
        />
        <p class="form-hint mt-1">
          Duration by minutes. Minimum 15 minutes, maximum 1440 minutes (24
          hours).
        </p>
      </div>
    </div>
  );
}

export default function DraftEventForm(props: {
  class?: string;
  event?: IEvent;
}) {
  const form = useForm<IDraftEventForm>(
    props.event
      ? {
          title: props.event.title,
          description: props.event.description,
          slots: props.event.slots,

          startAt: props.event.startAt,
          deadlineAt: props.event.deadlineAt,
          duration: props.event.duration,

          isPrivate: false,
        }
      : {
          title: "",
          slots: [{ jobs: [] }],
          startAt: 0,
          duration: 60,
          isPrivate: false,
        },
    draftEventAction
  );

  return (
    <form method="post" class={props.class}>
      <InfoCard form={form} />

      <SlotCard form={form} />

      <ScheduleCard form={form} />

      <div class="card mt-6">
        <div class="card-header">
          <h2 class="card-title">Publishing</h2>
        </div>

        <div class="border rounded-md p-4">
          <Switcher
            name="isPrivate"
            checked={form.form.isPrivate}
            onChange={form.formFieldWith("isPrivate")}
          >
            <p class="font-medium">Publish event as private</p>
            <p class="mt-1 text-sm">
              Private events are only visible to the invited participants.
            </p>
          </Switcher>
        </div>

        <div class="flex flex-row gap-x-2 mt-4">
          <button
            type="submit"
            class="btn btn-primary"
            onClick={form.submitForm("publish")}
          >
            Publish
          </button>
          <button
            type="submit"
            class="btn btn-ghost"
            onClick={form.submitForm("save")}
          >
            Save as draft
          </button>
        </div>
      </div>

      <pre class="p-4 bg-slate-100 rounded-lg mt-6">
        {JSON.stringify(form.form, null, 2)}
      </pre>
    </form>
  );
}
