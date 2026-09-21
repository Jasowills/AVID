import { useRef, useState } from "react";
import type { Timeline } from "@avid/shared-types";
import {
  GUTTER_WIDTH,
  LANE_HEIGHT,
  PX_PER_SECOND,
  formatRulerLabel,
  kindFill,
  layoutTimeline,
  rulerStep,
  rulerTicks,
  snapTime,
} from "./timelineLayout";

export interface TimelineCanvasProps {
  timeline: Timeline;
  selectedId: string | null;
  onSelect: (clipId: string | null) => void;
  /** Horizontal zoom multiplier (default 1). */
  zoom?: number;
  /** Playhead position in seconds (null hides it). */
  playhead?: number | null;
  /** Ruler click → seek time. Absent when seek is unavailable. */
  onSeek?: (time: number) => void;
  /** Drag-move commit (new start). Absent disables dragging. */
  onMoveClip?: (clipId: string, start: number) => void;
  /** Drag-trim commit (new start + duration). Absent disables trimming. */
  onTrimClip?: (clipId: string, start: number, duration: number) => void;
}

interface DragState {
  mode: "move" | "trim-l" | "trim-r";
  id: string;
  origStart: number;
  origDuration: number;
  grabOffset: number;
  curStart: number;
  curDuration: number;
  snapped: boolean;
}

const MIN_DURATION = 0.2;
const EDGE_PX = 8;

/**
 * SVG timeline renderer: tracks, clips, selection, ruler + playhead,
 * drag-move and drag-trim with edge snapping. Deterministic layout math
 * lives in `timelineLayout` (unit-tested); this component owns pointers.
 */
export function TimelineCanvas({
  timeline,
  selectedId,
  onSelect,
  zoom = 1,
  playhead = null,
  onSeek,
  onMoveClip,
  onTrimClip,
}: TimelineCanvasProps) {
  const scale = PX_PER_SECOND * zoom;
  const { rects, totalWidth, totalHeight, duration } = layoutTimeline(timeline, selectedId, zoom);
  const [drag, setDrag] = useState<DragState | null>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const movedRef = useRef(false);

  const step = rulerStep(duration, scale);
  const ticks = rulerTicks(duration, step);

  const timeAt = (clientX: number): number => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return 0;
    return Math.max(0, (clientX - rect.left - GUTTER_WIDTH) / scale);
  };

  const edgesFor = (excludeId: string): number[] => {
    const edges: number[] = [];
    for (const clip of Object.values(timeline.clips)) {
      if (clip.id === excludeId) continue;
      edges.push(clip.start, clip.start + clip.duration);
    }
    return edges;
  };

  const beginDrag = (
    e: React.PointerEvent,
    rect: { id: string; x: number; width: number },
    clip: { start: number; duration: number },
  ): void => {
    if (e.button !== 0 || (!onMoveClip && !onTrimClip)) return;
    const svgX = (e.clientX - (svgRef.current?.getBoundingClientRect().left ?? 0));
    const localX = svgX - rect.x;
    let mode: DragState["mode"] = "move";
    if (localX < EDGE_PX && onTrimClip) mode = "trim-l";
    else if (localX > rect.width - EDGE_PX && onTrimClip) mode = "trim-r";
    else if (!onMoveClip) return;
    (e.target as Element).setPointerCapture?.(e.pointerId);
    movedRef.current = false;
    setDrag({
      mode,
      id: rect.id,
      origStart: clip.start,
      origDuration: clip.duration,
      grabOffset: Math.max(0, (svgX - rect.x) / scale),
      curStart: clip.start,
      curDuration: clip.duration,
      snapped: false,
    });
    onSelect(rect.id);
  };

  const continueDrag = (e: React.PointerEvent): void => {
    if (!drag) return;
    const pointerTime = timeAt(e.clientX);
    // Movement gate keeps clicks selecting without starting drags.
    const svgX = e.clientX - (svgRef.current?.getBoundingClientRect().left ?? 0);
    if (Math.abs(svgX - (GUTTER_WIDTH + (drag.origStart + drag.grabOffset) * scale)) > 3) {
      movedRef.current = true;
    }
    if (!movedRef.current) return;
    const edges = edgesFor(drag.id);
    if (drag.mode === "move") {
      const raw = Math.max(0, pointerTime - drag.grabOffset);
      const { time, snapped } = snapTime(raw, edges, scale);
      setDrag({ ...drag, curStart: time, snapped });
    } else if (drag.mode === "trim-l") {
      const maxStart = drag.origStart + drag.origDuration - MIN_DURATION;
      const raw = Math.min(Math.max(0, pointerTime), maxStart);
      const { time, snapped } = snapTime(raw, edges, scale);
      const clamped = Math.min(time, maxStart);
      setDrag({ ...drag, curStart: clamped, curDuration: drag.origStart + drag.origDuration - clamped, snapped });
    } else {
      const raw = Math.max(MIN_DURATION, pointerTime - drag.origStart);
      const end = drag.origStart + raw;
      const { time, snapped } = snapTime(end, edges, scale);
      setDrag({ ...drag, curDuration: Math.max(MIN_DURATION, time - drag.origStart), snapped });
    }
  };

  const endDrag = (): void => {
    if (drag && movedRef.current) {
      if (drag.mode === "move") {
        onMoveClip?.(drag.id, Math.round(drag.curStart * 1000) / 1000);
      } else {
        onTrimClip?.(
          drag.id,
          Math.round(drag.curStart * 1000) / 1000,
          Math.round(drag.curDuration * 1000) / 1000,
        );
      }
    }
    setDrag(null);
  };

  const rendered = rects.map((rect) => {
    if (drag && drag.id === rect.id) {
      return {
        ...rect,
        x: GUTTER_WIDTH + drag.curStart * scale,
        width: Math.max(4, drag.curDuration * scale),
      };
    }
    return rect;
  });

  return (
    <svg
      ref={svgRef}
      role="img"
      aria-label={`Timeline, ${timeline.tracks.length} tracks, ${rendered.length} clips, ${duration.toFixed(1)} seconds`}
      width="100%"
      height={totalHeight}
      viewBox={`0 0 ${totalWidth} ${totalHeight}`}
      className="block min-h-full touch-none bg-avid-base select-none"
      onClick={() => {
        if (!movedRef.current) onSelect(null);
        movedRef.current = false;
      }}
      onPointerMove={continueDrag}
      onPointerUp={endDrag}
      onPointerCancel={() => setDrag(null)}
    >
      {/* Ruler */}
      <g role="presentation">
        {ticks.map((time) => {
          const x = GUTTER_WIDTH + time * scale;
          return (
            <g key={time}>
              <line x1={x} x2={x} y1={14} y2={28} stroke="#455060" />
              <text x={x + 3} y={12} fill="#6b7480" fontSize={10} fontFamily="monospace">
                {formatRulerLabel(time)}
              </text>
            </g>
          );
        })}
        {onSeek && (
          <rect
            x={GUTTER_WIDTH}
            y={0}
            width={Math.max(totalWidth - GUTTER_WIDTH, 0)}
            height={28}
            fill="transparent"
            style={{ cursor: "pointer" }}
            onClick={(e) => {
              e.stopPropagation();
              const rect = svgRef.current?.getBoundingClientRect();
              if (!rect) return;
              onSeek(Math.max(0, (e.clientX - rect.left - GUTTER_WIDTH) / scale));
            }}
          >
            <title>Seek</title>
          </rect>
        )}
      </g>
      {timeline.tracks.map((track, lane) => (
        <g key={track.id}>
          <rect x={0} y={28 + lane * LANE_HEIGHT} width={totalWidth} height={LANE_HEIGHT} fill="transparent" />
          <text x={8} y={28 + lane * LANE_HEIGHT + 24} fill="#6b7480" fontSize={11}>
            {track.name}
          </text>
          <line
            x1={GUTTER_WIDTH}
            x2={totalWidth}
            y1={28 + lane * LANE_HEIGHT}
            y2={28 + lane * LANE_HEIGHT}
            stroke="#23282f"
          />
        </g>
      ))}
      {rendered.map((rect) => (
        <g
          key={rect.id}
          role="button"
          aria-label={`Clip ${rect.name}${rect.selected ? ", selected" : ""}`}
          aria-pressed={rect.selected}
          tabIndex={0}
          onClick={(e) => {
            e.stopPropagation();
            if (!movedRef.current) onSelect(rect.id);
            movedRef.current = false;
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              onSelect(rect.id);
            }
          }}
          onPointerDown={(e) => {
            const clip = timeline.clips[rect.id];
            if (clip) beginDrag(e, rect, clip);
          }}
          style={{ cursor: onMoveClip || onTrimClip ? "grab" : "pointer" }}
        >
          <rect
            x={rect.x}
            y={28 + rect.lane * LANE_HEIGHT + 5}
            width={rect.width}
            height={LANE_HEIGHT - 10}
            rx={4}
            fill={kindFill(rect.kind)}
            stroke={drag?.id === rect.id ? "#4f8cff" : rect.selected ? "#e8eaed" : "transparent"}
            strokeWidth={drag?.id === rect.id || rect.selected ? 2 : 0}
            strokeDasharray={drag?.snapped && drag.id === rect.id ? "4,2" : undefined}
          />
          {rect.width > 40 && (
            <text x={rect.x + 6} y={28 + rect.lane * LANE_HEIGHT + 25} fill="#e8eaed" fontSize={11}>
              {rect.name}
            </text>
          )}
        </g>
      ))}
      {playhead !== null && Number.isFinite(playhead) && (
        <g role="presentation" pointerEvents="none">
          <line
            x1={GUTTER_WIDTH + playhead * scale}
            x2={GUTTER_WIDTH + playhead * scale}
            y1={0}
            y2={totalHeight}
            stroke="#e8eaed"
            strokeWidth={1.5}
          />
          <polygon
            points={`${GUTTER_WIDTH + playhead * scale - 5},0 ${GUTTER_WIDTH + playhead * scale + 5},0 ${GUTTER_WIDTH + playhead * scale},8`}
            fill="#e8eaed"
          />
        </g>
      )}
    </svg>
  );
}
