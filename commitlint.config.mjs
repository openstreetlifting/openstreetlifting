export default {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      [
        "feat",
        "fix",
        "perf",
        "refactor",
        "deps",
        "docs",
        "build",
        "ci",
        "chore",
        "test",
        "revert",
        "style",
      ],
    ],
  },
};
