export const routes = {
  authMe: "/v1/auth/me",
};

export default function scopedFetch({
  path,
  contentType = "application/json",
  branchId,
  method,
  body,
}: {
  path: string;
  contentType?: "application/json";
  branchId?: string;
  method?: string;
  body?: BodyInit;
}) {
  const parts = window.location.hostname.split(".");
  const slug = parts[0]?.toLowerCase();

  return fetch(`${import.meta.env.VITE_PUBLIC_API}${path}`, {
    method,
    mode: "cors",
    credentials: "include",
    headers: {
      "Content-Type": contentType,
      "X-App-Context": "pos",
      ...(slug ? { "X-Tenant-Slug": slug } : {}),
      ...(branchId ? { "X-Branch-ID": branchId } : {}),
    },
    body,
  });
}
