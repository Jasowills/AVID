# Testing strategy (scaffold stub → expanded per phase)

AGENTS §§91–104. Testing is part of implementation. Every feature: unit + integration (+E2E/UI where applicable) + regression. Deterministic fixtures only (`fixtures/`). Render tests assert streams/duration/resolution, never brittle bytes. AI tests assert validated structured output + rejection of malformed/hallucinated input.
