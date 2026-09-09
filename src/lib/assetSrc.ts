import { convertFileSrc } from "@tauri-apps/api/core";
import type { AssetView } from "./types";

/**
 * Turns an absolute path into an `asset:` URL the webview can stream.
 *
 * The asset protocol honours HTTP Range requests, which is what makes seeking
 * inside a multi-gigabyte MP4 work. Media is never base64'd into the DOM.
 */
export function assetUrl(path: string | null | undefined): string | null {
  if (!path) return null;
  try {
    return convertFileSrc(path);
  } catch {
    return null;
  }
}

export const mediaUrl = (a: AssetView | null | undefined): string | null =>
  a?.fileExists ? assetUrl(a.resolvedPath) : null;

export const thumbUrl = (a: AssetView | null | undefined): string | null =>
  assetUrl(a?.thumbPath ?? null);

/**
 * Formats WebView2 can actually decode. Anything else still gets a `<video>`
 * element — the browser is the authority — but this drives the up-front hint.
 */
export function likelyPlayable(name: string | null | undefined): boolean {
  const ext = name?.split(".").pop()?.toLowerCase() ?? "";
  return ["mp4", "m4v", "webm", "ogv", "ogg"].includes(ext);
}
