import { graphql } from "./gql";

export const Fractal = graphql(/* GraphQL */ `
  fragment Fractal on FractalGraphQL {
    id
    name
    createdAt
    updatedAt
  }
`);

export const FRACTAL = graphql(/* GraphQL */ `
  query Fractal($name: String, $childrenInput: GetFractalChildrenInput!) {
    fractal(name: $name) {
      ...Fractal
      children(input: $childrenInput) {
        ...Fractal
      }
      parents {
        ...Fractal
      }
      contexts {
        ...Fractal
      }
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
  mutation AddRelation($parentId: UUID!, $childId: UUID!, $contextId: UUID) {
    addRelation(parentId: $parentId, childId: $childId, contextId: $contextId)
  }
`);

export const DELETE_FRACTAL = graphql(/* GraphQL */ `
  mutation DeleteFractal($deleteFractalId: UUID!) {
    deleteFractal(id: $deleteFractalId)
  }
`);
