import {
  AlertTriangle,
  Check,
  ChevronDown,
  ChevronRight,
  ChevronUp,
  Clock,
  Eye,
  EyeOff,
  Film,
  Folder,
  House,
  Info,
  Layers,
  LayoutGrid,
  List,
  Lock,
  LockOpen,
  Magnet,
  Mic,
  Music,
  PanelBottom,
  PanelLeft,
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
  chevronDown: ChevronDown,
  chevronUp: ChevronUp,
  hideUI: EyeOff,
  showUI: Eye,
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
  panelLeft: PanelLeft,
  panelBottom: PanelBottom,
  settings: Settings,
  volume: Volume2,
  mute: VolumeX,
  folder: Folder,
  house: House,
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
