import { NavLink } from "react-router-dom";
import { Icon, LogoMark, type IconName } from "@avid/ui";

const ITEMS: Array<{ to: string; label: string; icon: IconName; end: boolean }> = [
  { to: "/", label: "Projects", icon: "house", end: true },
  { to: "/projects/new", label: "New project", icon: "plus", end: false },
  { to: "/settings", label: "Settings", icon: "settings", end: false },
];

/** Slim persistent nav rail (56px): product mark + icon navigation. */
export function NavRail() {
  return (
    <nav
      aria-label="Primary"
      className="flex w-14 shrink-0 flex-col items-center gap-1 border-r border-avid-border bg-avid-panel py-3"
    >
      <span aria-hidden="true" className="mb-3 flex size-8 items-center justify-center text-avid-primary">
        <LogoMark size={22} />
      </span>
      {ITEMS.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          end={item.end}
          aria-label={item.label}
          title={item.label}
          className={({ isActive }) =>
            `rounded-avid-md p-2 transition-colors ${
              isActive
                ? "bg-avid-raised text-avid-primary"
                : "text-avid-muted hover:bg-avid-raised hover:text-avid-secondary"
            }`
          }
        >
          <Icon name={item.icon} size={18} />
        </NavLink>
      ))}
    </nav>
  );
}
