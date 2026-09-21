import {
  AlertTriangle,
  Check,
  ChevronRight,
  Clock,
  Film,
  Folder,
  Info,
  Layers,
  LayoutGrid,
  List,
  Lock,
  LockOpen,
  Magnet,
  Mic,
  Music,
  Pause,
  Play,
  Plus,
  Scissors,
  Search,
  Settings,
  Split,
  Trash2,
  Undo2,
  Redo2,
  Volume2,
  VolumeX,
  X,
  ZoomIn,
  type LucideIcon,
} from "lucide-react";

/** Single coherent icon set (lucide, ISC). Geometric, one stroke discipline. */
const icons = {
  close: X,
  grid: LayoutGrid,
  list: List,
  music: Music,
  film: Film,
  chevronRight: ChevronRight,
  play: Play,
  pause: Pause,
  plus: Plus,
  search: Search,
  scissors: Scissors,
  split: Split,
  trash: Trash2,
  undo: Undo2,
  redo: Redo2,
  lock: Lock,
  unlock: LockOpen,
  clock: Clock,
  alert: AlertTriangle,
  check: Check,
  info: Info,
  layers: Layers,
  magnet: Magnet,
  mic: Mic,
  settings: Settings,
  volume: Volume2,
  mute: VolumeX,
  folder: Folder,
  zoom: ZoomIn,
} satisfies Record<string, LucideIcon>;

export type IconName = keyof typeof icons;

export interface IconProps {
  name: IconName;
  size?: number;
  className?: string;
}

/** Geometric icon, currentColor, decorative by default (label the button, not the icon). */
export function Icon({ name, size = 16, className = "" }: IconProps) {
  const Component = icons[name];
  return <Component size={size} className={className} aria-hidden="true" focusable="false" />;
}
