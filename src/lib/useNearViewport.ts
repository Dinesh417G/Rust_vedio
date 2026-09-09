import { useEffect, useRef, useState } from "react";

/**
 * True once the element has come within `margin` of the viewport, and stays
 * true afterwards.
 *
 * A chapter with a hundred video blocks would otherwise create a hundred
 * `<video>` elements on open, each fetching metadata. Mounting the player only
 * when the block is nearly on screen keeps that cost proportional to what is
 * actually being read, and never unmounts it again so playback is not
 * interrupted by scrolling.
 */
export function useNearViewport<T extends HTMLElement>(margin = "600px") {
  const ref = useRef<T | null>(null);
  const [near, setNear] = useState(false);

  useEffect(() => {
    if (near) return;
    const el = ref.current;
    if (!el) return;
    // Without IntersectionObserver, show everything rather than nothing.
    if (typeof IntersectionObserver === "undefined") {
      setNear(true);
      return;
    }
    const io = new IntersectionObserver(
      (entries) => {
        if (entries.some((e) => e.isIntersecting)) {
          setNear(true);
          io.disconnect();
        }
      },
      { rootMargin: margin },
    );
    io.observe(el);
    return () => io.disconnect();
  }, [near, margin]);

  return [ref, near] as const;
}
