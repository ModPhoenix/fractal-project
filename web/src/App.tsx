import { Layout } from "./components/layout";
import { graphql } from "../src/api/gql";
import { useQuery } from "@apollo/client";

const root = graphql(`
  query Root {
    root {
      id
      name
      children {
        id
        name
      }
    }
  }
`);

export function App() {
  const { data } = useQuery(root);

  console.log("data", data);

  return <Layout />;
}
