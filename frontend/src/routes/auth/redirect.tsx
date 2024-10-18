import type { APIEvent } from "@solidjs/start/server";
import { json, redirect, useLocation } from "@solidjs/router";
// import { GET } from "@solidjs/start";
//
// const redirect = GET(async (name: string) => {
//   console.log(name);
//
//   return json({
//     message: "Redirecting to Discord",
//   });
// });

// export default function Redirect() {
//   const params = useLocation();
//   console.log(params);
//
//   console.log("Redirecting from Discord");
// }

export function GET({ request }: APIEvent) {
  const code = new URL(request.url).searchParams.get("code");

  console.log(code);

  return new Response("Redirecting to Discord", {
    status: 302,
    headers: { Location: "/" },
  });
}
