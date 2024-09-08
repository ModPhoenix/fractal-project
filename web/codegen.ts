import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  overwrite: true,
  schema: "http://localhost:8000",
  documents: "src/**/*.tsx",
  generates: {
    "src/api/gql/": {
      preset: "client",
      plugins: [],
    },
  },
};

export default config;
