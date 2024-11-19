export default function Page() {
  return (
    <>
      <div class="flex flex-col gap-y-4">
        <h1 class="text-2xl font-semibold">Welcome to Astral Bells</h1>
        <p>
          Astral Bells is a simple web application that allows you to create and
          manage your own bell schedules.
        </p>
        <p>
          You can create a bell schedule by clicking on the "Create Event"
          button below.
        </p>

        <div>
          <a href="/events/create" class="btn btn-primary">
            Create Event
          </a>
        </div>
      </div>
    </>
  );
}
