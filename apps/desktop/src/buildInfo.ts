/**
 * Bundle identity stamp — injected at build time from git HEAD, so a
 * screenshot or bug report ties to the exact commit the bundle was cut from.
 * Rendered tiny in the Home footer; not product UI.
 */
declare const __AVID_BUILD__: string | undefined;

export const BUILD_ID: string =
  typeof __AVID_BUILD__ === "string" && __AVID_BUILD__.length > 0 ? __AVID_BUILD__ : "dev";
