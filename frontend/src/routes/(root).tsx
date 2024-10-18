import { RouteSectionProps } from "@solidjs/router";
import { createResource } from "solid-js";

export default function RootLayout(progs: RouteSectionProps) {
  const [data] = createResource(async () => {
    "use server";

    const redirectUri = encodeURIComponent(
      "http://localhost:3000/auth/redirect"
    );

    console.log("fetching data");

    const res = await fetch(
      `http://localhost:8080/api/auth?redirect_uri=${redirectUri}`
    );
    const data: { discordOauthUrl: string } = await res.json();
    return data;
  });

  return (
    <div>
      <nav class="flex flex-row gap-x-4">
        <a href="/">Home</a>
        <a href="/demo">Demo</a>

        <a href={data()?.discordOauthUrl}>Sign In</a>
      </nav>

      {progs.children}
    </div>
  );
}
