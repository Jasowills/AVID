import type { Timeline } from "@avid/shared-types";
import { GUTTER_WIDTH, LANE_HEIGHT, kindFill, layoutTimeline } from "./timelineLayout";

export interface TimelineCanvasProps {
  timeline: Timeline;
  selectedId: string | null;
  onSelect: (clipId: string | null) => void;
}

/**
 * SVG timeline renderer (Phase 3 UI slice): tracks, clips, selection.
 * Deterministic SVG (not canvas) — accessible roles, no hit-test code,
 * layout math unit-tested in `timelineLayout`.
 */
export function TimelineCanvas({ timeline, selectedId, onSelect }: TimelineCanvasProps) {
  const { rects, totalWidth, totalHeight, duration } = layoutTimeline(timeline, selectedId);

  return (
    <svg
      role="img"
      aria-label={`Timeline, ${timeline.tracks.length} tracks, ${rects.length} clips, ${duration.toFixed(1)} seconds`}
      width="100%"
      height={totalHeight}
      viewBox={`0 0 ${totalWidth} ${totalHeight}`}
      className="block min-h-full bg-avid-base"
      onClick={() => onSelect(null)}
    >
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
      {rects.map((rect) => (
        <g
          key={rect.id}
          role="button"
          aria-label={`Clip ${rect.name}${rect.selected ? ", selected" : ""}`}
          aria-pressed={rect.selected}
          tabIndex={0}
          onClick={(e) => {
            e.stopPropagation();
            onSelect(rect.id);
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              onSelect(rect.id);
            }
          }}
          style={{ cursor: "pointer" }}
        >
          <rect
            x={rect.x}
            y={28 + rect.lane * LANE_HEIGHT + 5}
            width={rect.width}
            height={LANE_HEIGHT - 10}
            rx={4}
            fill={kindFill(rect.kind)}
            stroke={rect.selected ? "#e8eaed" : "transparent"}
            strokeWidth={rect.selected ? 2 : 0}
          />
          {rect.width > 40 && (
            <text x={rect.x + 6} y={28 + rect.lane * LANE_HEIGHT + 25} fill="#e8eaed" fontSize={11}>
              {rect.name}
            </text>
          )}
        </g>
      ))}
    </svg>
  );
}
