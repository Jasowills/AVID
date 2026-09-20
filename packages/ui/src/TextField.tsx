import type { InputHTMLAttributes } from "react";
import { useId } from "react";

export interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
}

/** Labeled input with error text wired via aria-describedby. */
export function TextField({ label, error, id, className = "", ...rest }: TextFieldProps) {
  const autoId = useId();
  const inputId = id ?? autoId;
  const errorId = `${inputId}-error`;
  return (
    <div className="flex flex-col gap-1">
      <label htmlFor={inputId} className="text-sm text-avid-secondary">
        {label}
      </label>
      <input
        id={inputId}
        aria-invalid={error ? true : undefined}
        aria-describedby={error ? errorId : undefined}
        className={`rounded-avid-md border bg-avid-raised px-3 py-2 text-sm text-avid-primary placeholder:text-avid-muted focus-visible:outline-2 focus-visible:outline-avid-accent ${
          error ? "border-avid-danger" : "border-avid-border"
        } ${className}`}
        {...rest}
      />
      {error && (
        <p id={errorId} role="alert" className="text-xs text-avid-danger">
          {error}
        </p>
      )}
    </div>
  );
}
