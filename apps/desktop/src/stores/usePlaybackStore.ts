import { create } from "zustand";

export interface SeekRequest {
  time: number;
  nonce: number;
}

interface PlaybackState {
  /** Latest seek request (null = none). Consumers act then leave it. */
  seekRequest: SeekRequest | null;
  /** Last reported media time in seconds. */
  currentTime: number;
  requestSeek: (time: number) => void;
  reportTime: (time: number) => void;
}

/**
 * Minimal playback bus between the preview `<video>` and timeline/transcript.
 * No playback engine of its own — the video element is the clock; this store
 * only ferries seek requests outward and time reports inward.
 */
export const usePlaybackStore = create<PlaybackState>()((set) => ({
  seekRequest: null,
  currentTime: 0,
  requestSeek: (time: number) =>
    set({ seekRequest: { time: Math.max(0, time), nonce: Date.now() + Math.random() } }),
  reportTime: (time: number) => set({ currentTime: time }),
}));
