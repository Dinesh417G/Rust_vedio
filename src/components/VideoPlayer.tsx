import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  Camera,
  ExternalLink,
  FolderOpen,
  Gauge,
  MapPin,
  Maximize,
  Pause,
  PictureInPicture2,
  Play,
  RotateCcw,
  RotateCw,
  TriangleAlert,
  Volume2,
  VolumeX,
} from "lucide-react";
import type { AssetView, BlockView } from "@/lib/types";
import { likelyPlayable, mediaUrl } from "@/lib/assetSrc";
import { clockTime, cx } from "@/lib/format";
import * as ipc from "@/lib/ipc";
import { useUi } from "@/store/ui";

const SPEEDS = [0.5, 0.75, 1, 1.25, 1.5, 1.75, 2];
/** Position is written every this many seconds, and on pause and unmount. */
const SAVE_EVERY_MS = 5000;
/** Watching this much of a video marks the block complete. */
const COMPLETE_AT = 0.9;
const VOLUME_KEY = "rustclass.volume";

interface Props {
  block: BlockView;
  asset: AssetView;
  authoring: boolean;
  onReveal: () => void;
}

export function VideoPlayer({ block, asset, authoring, onReveal }: Props) {
  const video = useRef<HTMLVideoElement>(null);
  const wrap = useRef<HTMLDivElement>(null);
  const toast = useUi((s) => s.toast);

  const src = useMemo(() => mediaUrl(asset), [asset]);
  const resumeAt = block.progress?.lastPositionSecs ?? 0;

  const [playing, setPlaying] = useState(false);
  const [current, setCurrent] = useState(0);
  const [duration, setDuration] = useState(asset.durationSecs ?? 0);
  const [buffered, setBuffered] = useState(0);
  const [volume, setVolume] = useState(() => {
    const stored = Number(localStorage.getItem(VOLUME_KEY));
    return Number.isFinite(stored) && stored > 0 ? Math.min(1, stored) : 1;
  });
  const [muted, setMuted] = useState(false);
  const [speed, setSpeed] = useState(1);
  const [speedOpen, setSpeedOpen] = useState(false);
  const [failed, setFailed] = useState(false);
  const [showResume, setShowResume] = useState(resumeAt > 5);
  const [markerDraft, setMarkerDraft] = useState<string | null>(null);

  const lastSaved = useRef(0);
  const watched = useRef(block.progress?.watchedSecs ?? 0);
  const completed = useRef(block.progress?.completed ?? false);

  // --- persistence ---------------------------------------------------------

  const save = useCallback(
    (position: number, force = false) => {
      const now = Date.now();
      if (!force && now - lastSaved.current < SAVE_EVERY_MS) return;
      lastSaved.current = now;
      const patch: Parameters<typeof ipc.progressSet>[1] = {
        lastPositionSecs: position,
        watchedSecs: watched.current,
      };
      if (!completed.current && duration > 0 && watched.current / duration >= COMPLETE_AT) {
        completed.current = true;
        patch.completed = true;
      }
      void ipc.progressSet(block.id, patch).catch(() => {
        // A failed position write is not worth interrupting playback for; the
        // next tick will try again.
      });
    },
    [block.id, duration],
  );

  useEffect(() => {
    const el = video.current;
    return () => {
      if (el && el.currentTime > 0) save(el.currentTime, true);
    };
  }, [save]);

  // --- media element wiring ------------------------------------------------

  useEffect(() => {
    const el = video.current;
    if (!el) return;
    el.volume = volume;
    el.muted = muted;
    el.playbackRate = speed;
  }, [volume, muted, speed]);

  const onLoadedMetadata = () => {
    const el = video.current;
    if (!el) return;
    setDuration(el.duration || 0);
    // Duration and dimensions come from the webview — there is no ffmpeg here.
    if (
      Number.isFinite(el.duration) &&
      (asset.durationSecs == null || asset.width == null)
    ) {
      void ipc
        .assetSetMediaMeta(asset.id, el.duration, el.videoWidth || null, el.videoHeight || null)
        .catch(() => undefined);
    }
  };

  const onTimeUpdate = () => {
    const el = video.current;
    if (!el) return;
    const t = el.currentTime;
    // Accumulate only forward motion, so scrubbing cannot inflate watch time.
    if (t > current && t - current < 2) watched.current += t - current;
    setCurrent(t);
    if (el.buffered.length > 0) setBuffered(el.buffered.end(el.buffered.length - 1));
    if (!el.paused) save(t);
  };

  const seek = useCallback(
    (t: number) => {
      const el = video.current;
      if (!el) return;
      el.currentTime = Math.max(0, Math.min(duration || el.duration || 0, t));
      setShowResume(false);
    },
    [duration],
  );

  const togglePlay = useCallback(() => {
    const el = video.current;
    if (!el) return;
    if (el.paused) {
      void el.play().catch(() => setFailed(true));
    } else {
      el.pause();
      save(el.currentTime, true);
    }
  }, [save]);

  // --- keyboard ------------------------------------------------------------

  useEffect(() => {
    const el = wrap.current;
    if (!el) return;
    const onKey = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      if (target && /input|textarea|select/i.test(target.tagName)) return;
      const v = video.current;
      if (!v) return;
      const step = (d: number) => {
        e.preventDefault();
        seek(v.currentTime + d);
      };
      switch (e.key) {
        case " ":
          e.preventDefault();
          togglePlay();
          break;
        case "k":
        case "K":
          e.preventDefault();
          v.pause();
          break;
        case "ArrowLeft":
          step(-5);
          break;
        case "ArrowRight":
          step(5);
          break;
        case "j":
        case "J":
          step(-10);
          break;
        case "l":
        case "L":
          step(10);
          break;
        case ",":
          step(-1 / 30);
          break;
        case ".":
          step(1 / 30);
          break;
        case "<":
          e.preventDefault();
          setSpeed((s) => SPEEDS[Math.max(0, SPEEDS.indexOf(s) - 1)] ?? 0.5);
          break;
        case ">":
          e.preventDefault();
          setSpeed((s) => SPEEDS[Math.min(SPEEDS.length - 1, SPEEDS.indexOf(s) + 1)] ?? 2);
          break;
        case "ArrowUp":
          e.preventDefault();
          setVolume((x) => {
            const n = Math.min(1, x + 0.05);
            localStorage.setItem(VOLUME_KEY, String(n));
            return n;
          });
          break;
        case "ArrowDown":
          e.preventDefault();
          setVolume((x) => {
            const n = Math.max(0, x - 0.05);
            localStorage.setItem(VOLUME_KEY, String(n));
            return n;
          });
          break;
        case "m":
        case "M":
          e.preventDefault();
          setMuted((m) => !m);
          break;
        case "f":
        case "F":
          e.preventDefault();
          void toggleFullscreen();
          break;
        default:
          if (/^[0-9]$/.test(e.key)) {
            e.preventDefault();
            seek(((duration || v.duration || 0) * Number(e.key)) / 10);
          }
      }
    };
    el.addEventListener("keydown", onKey);
    return () => el.removeEventListener("keydown", onKey);
  }, [duration, seek, togglePlay]);

  const toggleFullscreen = async () => {
    try {
      if (document.fullscreenElement) await document.exitFullscreen();
      else await wrap.current?.requestFullscreen();
    } catch {
      toast({ tone: "error", message: "Fullscreen was refused by the window." });
    }
  };

  const togglePip = async () => {
    const el = video.current;
    if (!el) return;
    try {
      if (document.pictureInPictureElement) await document.exitPictureInPicture();
      else await el.requestPictureInPicture();
    } catch {
      toast({ tone: "error", message: "Picture-in-picture is not available for this video." });
    }
  };

  // --- author extras -------------------------------------------------------

  const captureThumbnail = async () => {
    const el = video.current;
    if (!el || !el.videoWidth) {
      toast({ tone: "error", message: "The frame is not ready to capture yet." });
      return;
    }
    const canvas = document.createElement("canvas");
    const scale = Math.min(1, 640 / el.videoWidth);
    canvas.width = Math.round(el.videoWidth * scale);
    canvas.height = Math.round(el.videoHeight * scale);
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.drawImage(el, 0, 0, canvas.width, canvas.height);
    try {
      await ipc.assetSetThumbnail(asset.id, canvas.toDataURL("image/jpeg", 0.7));
      toast({ tone: "success", message: "Thumbnail captured from this frame." });
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const addMarker = async () => {
    const label = markerDraft?.trim();
    if (!label) return;
    try {
      await ipc.markerCreate(block.id, video.current?.currentTime ?? 0, label);
      setMarkerDraft(null);
      toast({ tone: "success", message: `Marker added at ${clockTime(current)}.` });
    } catch (e) {
      toast({ tone: "error", message: ipc.asAppError(e).message });
    }
  };

  const total = duration || asset.durationSecs || 0;
  const pct = total > 0 ? (current / total) * 100 : 0;
  const bufferedPct = total > 0 ? (buffered / total) * 100 : 0;

  return (
    <div className="space-y-2">
      <div
        ref={wrap}
        tabIndex={0}
        className="group/player relative overflow-hidden rounded-xl border border-line bg-black focus-visible:ring-2 focus-visible:ring-brand"
      >
        {src ? (
          <video
            ref={video}
            src={src}
            className="aspect-video w-full bg-black"
            preload="metadata"
            onLoadedMetadata={onLoadedMetadata}
            onTimeUpdate={onTimeUpdate}
            onPlay={() => setPlaying(true)}
            onPause={() => setPlaying(false)}
            onEnded={() => {
              setPlaying(false);
              completed.current = true;
              void ipc.progressSet(block.id, { completed: true }).catch(() => undefined);
            }}
            onError={() => setFailed(true)}
            onClick={togglePlay}
            onDoubleClick={toggleFullscreen}
          />
        ) : (
          <div className="grid aspect-video place-items-center text-sm text-white/70">
            This video could not be loaded.
          </div>
        )}

        {showResume && resumeAt > 5 && (
          <div className="absolute bottom-20 left-4 flex items-center gap-2 rounded-lg bg-black/80 px-3 py-2 text-sm text-white shadow-lg">
            <button
              className="btn btn-sm btn-primary"
              onClick={() => {
                seek(resumeAt);
                void video.current?.play().catch(() => setFailed(true));
              }}
            >
              Resume at {clockTime(resumeAt)}
            </button>
            <button
              className="btn btn-sm border-white/30 text-white hover:bg-white/10"
              onClick={() => {
                seek(0);
                setShowResume(false);
              }}
            >
              Start over
            </button>
          </div>
        )}

        {/* Controls */}
        <div className="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/90 to-transparent px-3 pb-2 pt-8">
          {/* Scrubber with buffered range */}
          <div className="relative h-6">
            <div className="absolute inset-x-0 top-2.5 h-1 rounded-full bg-white/25">
              <div
                className="absolute inset-y-0 left-0 rounded-full bg-white/35"
                style={{ width: `${bufferedPct}%` }}
              />
              <div
                className="absolute inset-y-0 left-0 rounded-full bg-brand"
                style={{ width: `${pct}%` }}
              />
            </div>
            <input
              type="range"
              min={0}
              max={Math.max(total, 0.001)}
              step={0.1}
              value={current}
              aria-label="Seek"
              onChange={(e) => seek(Number(e.target.value))}
              className="absolute inset-x-0 top-0 h-6 w-full cursor-pointer appearance-none bg-transparent
                         [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3
                         [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full
                         [&::-webkit-slider-thumb]:bg-brand"
            />
          </div>

          {/* Marker rail */}
          {block.markers.length > 0 && total > 0 && (
            <div className="relative -mt-1 h-3">
              {block.markers.map((m) => (
                <button
                  key={m.id}
                  title={`${clockTime(m.atSecs)} · ${m.label}`}
                  onClick={() => seek(m.atSecs)}
                  className="absolute top-0 h-3 w-1 -translate-x-1/2 rounded-full bg-warn hover:h-4 hover:bg-white"
                  style={{ left: `${Math.min(100, (m.atSecs / total) * 100)}%` }}
                >
                  <span className="sr-only">Jump to {m.label}</span>
                </button>
              ))}
            </div>
          )}

          <div className="flex flex-wrap items-center gap-1 text-white">
            <ControlButton onClick={togglePlay} label={playing ? "Pause" : "Play"}>
              {playing ? <Pause size={16} /> : <Play size={16} />}
            </ControlButton>
            <ControlButton onClick={() => seek(current - 10)} label="Back 10 seconds">
              <RotateCcw size={15} />
            </ControlButton>
            <ControlButton onClick={() => seek(current + 10)} label="Forward 10 seconds">
              <RotateCw size={15} />
            </ControlButton>

            <span className="ml-1 select-none text-xs tabular-nums text-white/85">
              {clockTime(current)} / {clockTime(total)}
            </span>

            <div className="ml-auto flex items-center gap-1">
              <ControlButton
                onClick={() => setMuted((m) => !m)}
                label={muted ? "Unmute" : "Mute"}
              >
                {muted || volume === 0 ? <VolumeX size={15} /> : <Volume2 size={15} />}
              </ControlButton>
              <input
                type="range"
                min={0}
                max={1}
                step={0.01}
                value={muted ? 0 : volume}
                aria-label="Volume"
                onChange={(e) => {
                  const v = Number(e.target.value);
                  setVolume(v);
                  setMuted(v === 0);
                  localStorage.setItem(VOLUME_KEY, String(v));
                }}
                className="h-1 w-20 cursor-pointer appearance-none rounded-full bg-white/30
                           [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3
                           [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full
                           [&::-webkit-slider-thumb]:bg-white"
              />

              <div className="relative">
                <ControlButton onClick={() => setSpeedOpen((o) => !o)} label="Playback speed">
                  <Gauge size={15} />
                  <span className="ml-1 text-xs tabular-nums">{speed}×</span>
                </ControlButton>
                {speedOpen && (
                  <div className="absolute bottom-full right-0 mb-1 rounded-lg bg-black/95 p-1 shadow-xl">
                    {SPEEDS.map((s) => (
                      <button
                        key={s}
                        className={cx(
                          "block w-full rounded px-3 py-1 text-left text-xs tabular-nums hover:bg-white/15",
                          s === speed && "text-brand",
                        )}
                        onClick={() => {
                          setSpeed(s);
                          setSpeedOpen(false);
                        }}
                      >
                        {s}×
                      </button>
                    ))}
                  </div>
                )}
              </div>

              <ControlButton onClick={togglePip} label="Picture in picture">
                <PictureInPicture2 size={15} />
              </ControlButton>
              <ControlButton onClick={toggleFullscreen} label="Fullscreen">
                <Maximize size={15} />
              </ControlButton>
            </div>
          </div>
        </div>
      </div>

      {/* Codec fallback — never a black box with no explanation. */}
      {(failed || !likelyPlayable(asset.originalName)) && (
        <div
          className={cx(
            "flex flex-wrap items-center gap-2 rounded-lg border px-3 py-2 text-sm",
            failed ? "border-danger/50 bg-danger/10" : "border-warn/40 bg-warn/10",
          )}
        >
          <TriangleAlert size={15} className={failed ? "text-danger" : "text-warn"} />
          <span className="text-ink">
            This file&rsquo;s format may not be supported by the built-in player.
          </span>
          <button
            className="btn btn-sm"
            onClick={() => void ipc.assetOpenExternal(asset.id).catch(() => undefined)}
          >
            <ExternalLink size={13} /> Open in default player
          </button>
          <button className="btn btn-sm" onClick={onReveal}>
            <FolderOpen size={13} /> Show in folder
          </button>
        </div>
      )}

      {authoring && (
        <div className="flex flex-wrap items-center gap-2">
          <button className="btn btn-sm" onClick={captureThumbnail}>
            <Camera size={13} /> Capture thumbnail from current frame
          </button>
          {markerDraft === null ? (
            <button className="btn btn-sm" onClick={() => setMarkerDraft("")}>
              <MapPin size={13} /> Add marker at {clockTime(current)}
            </button>
          ) : (
            <div className="flex items-center gap-1">
              <input
                autoFocus
                className="input w-56 py-1 text-sm"
                placeholder={`Marker at ${clockTime(current)}`}
                value={markerDraft}
                onChange={(e) => setMarkerDraft(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") void addMarker();
                  if (e.key === "Escape") setMarkerDraft(null);
                }}
              />
              <button className="btn btn-sm btn-primary" onClick={addMarker}>
                Add
              </button>
              <button className="btn btn-sm btn-ghost" onClick={() => setMarkerDraft(null)}>
                Cancel
              </button>
            </div>
          )}
        </div>
      )}

      {block.markers.length > 0 && (
        <ul className="flex flex-wrap gap-1.5">
          {block.markers.map((m) => (
            <li key={m.id}>
              <button
                className="chip hover:border-brand hover:text-ink"
                onClick={() => seek(m.atSecs)}
              >
                <MapPin size={11} className="text-warn" />
                <span className="tabular-nums">{clockTime(m.atSecs)}</span>
                <span className="max-w-[14rem] truncate">{m.label}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function ControlButton({
  children,
  onClick,
  label,
}: {
  children: React.ReactNode;
  onClick: () => void;
  label: string;
}) {
  return (
    <button
      onClick={onClick}
      title={label}
      aria-label={label}
      className="inline-flex items-center rounded px-2 py-1.5 text-white/90 transition-colors hover:bg-white/15 hover:text-white"
    >
      {children}
    </button>
  );
}
