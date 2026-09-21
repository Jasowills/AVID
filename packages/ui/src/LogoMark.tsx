export interface LogoMarkProps {
  size?: number;
  className?: string;
}

/**
 * AVID mark: two timeline bars split by an edit point, with a playhead
 * dropping into the cut. Geometric, monochrome-safe (currentColor),
 * legible at favicon size. No gradients, no glow, no clichés.
 */
export function LogoMark({ size = 20, className = "" }: LogoMarkProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden="true"
      focusable="false"
      className={className}
    >
      <path d="M12 1.5 L15.5 6 L8.5 6 Z" fill="currentColor" />
      <rect x="4" y="8" width="5" height="13" rx="1.5" fill="currentColor" />
      <rect x="15" y="8" width="5" height="13" rx="1.5" fill="currentColor" />
    </svg>
  );
}
