import type { ButtonHTMLAttributes, ReactNode } from "react";

export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  children: ReactNode;
}

const variantClass: Record<ButtonVariant, string> = {
  primary: "bg-avid-accent text-white hover:bg-avid-accent-hover",
  secondary: "bg-avid-raised text-avid-primary border border-avid-border hover:border-avid-border-strong",
  ghost: "text-avid-secondary hover:text-avid-primary hover:bg-avid-raised",
  danger: "bg-transparent text-avid-danger border border-avid-danger/40 hover:bg-avid-danger/10",
};

/**
 * Primary action button. Always pairs color with a text label —
 * never color-alone semantics (AGENTS §59).
 */
export function Button({ variant = "secondary", children, className = "", ...rest }: ButtonProps) {
  return (
    <button
      className={`inline-flex items-center justify-center gap-2 rounded-avid-md px-4 py-2 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-avid-accent disabled:cursor-not-allowed disabled:opacity-50 ${variantClass[variant]} ${className}`}
      {...rest}
    >
      {children}
    </button>
  );
}
