import {
  AccessorWithLatest,
  cache,
  createAsync,
  RouteSectionProps,
} from "@solidjs/router";
import { Show } from "solid-js";
import { get } from "~/utils/fetch";

interface AuthInformation {
  discordOauthUrl: string;
  member?: {
    id: number;
    name: string;
    avatar: string;
  };
}

const getAuthInformation = cache(async () => {
  "use server";
  return get<AuthInformation>({
    path: "/api/auth",
    query: { redirect_uri: "http://localhost:3000/auth/redirect" },
  });
}, "auth_information");

function AuthStatus(pros: { auth: AuthInformation }) {
  return (
    <Show
      when={pros.auth.member}
      keyed
      fallback={
        <a
          class="font-medium flex flex-row gap-x-2 items-center bg-transparent text-slate-8 hover:text-slate-8/80"
          href={pros.auth.discordOauthUrl}
        >
          Sign In
          <i class="i-logos-discord-icon w-6 h-6 inline-block"></i>
        </a>
      }
    >
      {(member) => (
        <a href="/" class="flex flex-row gap-x-2 items-center font-medium">
          <p>{member.name}</p>
          <img
            class="w-10 h-10 rounded-full"
            src={member.avatar}
            alt={member.name + "'s avatar"}
          />
        </a>
      )}
    </Show>
  );
}

export default function RootLayout(progs: RouteSectionProps) {
  const auth = createAsync(async () => {
    return getAuthInformation();
  });

  return (
    <>
      <div class="h-16 border-b flex sticky top-0 bg-white z-20">
        <div class="container px-2 flex flex-row justify-between mx-auto">
          <nav class="flex flex-row items-center gap-x-4">
            <a href="/" class="font-semibold text-xl">
              Astral Bells
            </a>
          </nav>

          <Show when={auth()} keyed>
            {(auth) => <AuthStatus auth={auth} />}
          </Show>
        </div>
      </div>

      <div class="my-4 container mx-auto px-2">{progs.children}</div>
    </>
  );
}

export const route = {
  preload: () => getAuthInformation(),
};
