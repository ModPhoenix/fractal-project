import { graphql } from "./gql";

export const Fractal = graphql(/* GraphQL */ `
  fragment Fractal on FractalGraphQL {
    id
    name
    createdAt
    updatedAt
    children {
      id
      name
      createdAt
      updatedAt
      children {
        id
        name
      }
    }
    parents {
      id
      name
      createdAt
      updatedAt
    }
    contexts {
      id
      name
      createdAt
      updatedAt
    }
  }
`);

export const FRACTAL = graphql(/* GraphQL */ `
  query Fractal($name: String) {
    fractal(name: $name) {
      ...Fractal
    }
  }
`);

export const CREATE_FRACTAL = graphql(/* GraphQL */ `
  mutation CreateFractal($input: CreateFractalInput!) {
    createFractal(input: $input) {
      ...Fractal
    }
  }
`);

export const ADD_RELATION = graphql(/* GraphQL */ `
  mutation AddRelation($parentId: UUID!, $childId: UUID!) {
    addRelation(parentId: $parentId, childId: $childId)
  }
`);

export const DELETE_FRACTAL = graphql(/* GraphQL */ `
  mutation DeleteFractal($deleteFractalId: UUID!) {
    deleteFractal(id: $deleteFractalId)
  }
`);
