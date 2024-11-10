// This file only serves as a formatter for markdown files, providing greater agility in organizing guide materials.
import markdown from "eslint-plugin-markdown";

export default [
  {
    files: ["**/*.md"], // Inclui todos os arquivos .md
    ignores: [
      "**/node_modules/**",
      "**/target/**",
      "**/dist/**",
      "**/generated/**",
    ],
    languageOptions: {
      parser: markdown.processors.markdown,
    },
    plugins: {
      markdown,
    },
    rules: {},
  },
];
