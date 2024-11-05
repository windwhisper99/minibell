import { cache, createAsync, redirect, useSearchParams } from "@solidjs/router";
import { Show } from "solid-js";
import CraftEventForm from "~/components/draft-event-form";
import {
  draftEventQuery,
  dutiesQuery,
  IDutyCategory,
  IEvent,
} from "~/utils/api";
import { isLogin } from "~/utils/fetch";

const createEvent = cache(
  async (id?: string): Promise<{ event?: IEvent; duties: IDutyCategory[] }> => {
    "use server";
    if (!isLogin()) throw redirect("/");

    const duties = await dutiesQuery();
    if (id) {
      const event = await draftEventQuery(id);
      return { event, duties };
    }

    return { duties };
  },
  "create_event"
);

export default function Page() {
  const [query] = useSearchParams();
  const data = createAsync(() => createEvent(query.id));

  return (
    <div class="max-w-3xl mx-auto">
      <Show
        when={data()?.event}
        keyed
        fallback={<h1 class="text-2xl font-semibold mt-6">Create Event</h1>}
      >
        {(event) => (
          <h1 class="text-2xl font-semibold mt-6">Edit: {event.id}</h1>
        )}
      </Show>

      <Show when={data()} keyed>
        {(data) => (
          <CraftEventForm
            class="mt-6"
            event={data.event}
            duties={data.duties}
          />
        )}
      </Show>
    </div>
  );
}
