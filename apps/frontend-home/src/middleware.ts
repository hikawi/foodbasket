import { defineMiddleware } from "astro:middleware";

const SUPPORTED_LANGUAGES = ["en", "ja"];

export const onRequest = defineMiddleware((context, next) => {
  const { pathname } = context.url;
  const firstSegment = pathname.split("/").filter(Boolean)[0];
  if (firstSegment && !SUPPORTED_LANGUAGES.includes(firstSegment)) {
    return new Response(null, { status: 404 });
  }

  return next();
});
