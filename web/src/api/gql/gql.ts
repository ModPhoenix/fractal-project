/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  fragment Fractal on FractalGraphQL {\n    id\n    name\n    createdAt\n    updatedAt\n  }\n": types.FractalFragmentDoc,
    "\n  query Fractal($name: String, $childrenInput: GetFractalChildrenInput!) {\n    fractal(name: $name) {\n      ...Fractal\n      children(input: $childrenInput) {\n        ...Fractal\n      }\n      parents {\n        ...Fractal\n      }\n      contexts {\n        ...Fractal\n      }\n    }\n  }\n": types.FractalDocument,
    "\n  mutation CreateFractal($input: CreateFractalInput!) {\n    createFractal(input: $input) {\n      ...Fractal\n    }\n  }\n": types.CreateFractalDocument,
    "\n  mutation AddRelation($parentId: UUID!, $childId: UUID!, $contextId: UUID) {\n    addRelation(parentId: $parentId, childId: $childId, contextId: $contextId)\n  }\n": types.AddRelationDocument,
    "\n  mutation DeleteFractal($deleteFractalId: UUID!) {\n    deleteFractal(id: $deleteFractalId)\n  }\n": types.DeleteFractalDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  fragment Fractal on FractalGraphQL {\n    id\n    name\n    createdAt\n    updatedAt\n  }\n"): (typeof documents)["\n  fragment Fractal on FractalGraphQL {\n    id\n    name\n    createdAt\n    updatedAt\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Fractal($name: String, $childrenInput: GetFractalChildrenInput!) {\n    fractal(name: $name) {\n      ...Fractal\n      children(input: $childrenInput) {\n        ...Fractal\n      }\n      parents {\n        ...Fractal\n      }\n      contexts {\n        ...Fractal\n      }\n    }\n  }\n"): (typeof documents)["\n  query Fractal($name: String, $childrenInput: GetFractalChildrenInput!) {\n    fractal(name: $name) {\n      ...Fractal\n      children(input: $childrenInput) {\n        ...Fractal\n      }\n      parents {\n        ...Fractal\n      }\n      contexts {\n        ...Fractal\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateFractal($input: CreateFractalInput!) {\n    createFractal(input: $input) {\n      ...Fractal\n    }\n  }\n"): (typeof documents)["\n  mutation CreateFractal($input: CreateFractalInput!) {\n    createFractal(input: $input) {\n      ...Fractal\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation AddRelation($parentId: UUID!, $childId: UUID!, $contextId: UUID) {\n    addRelation(parentId: $parentId, childId: $childId, contextId: $contextId)\n  }\n"): (typeof documents)["\n  mutation AddRelation($parentId: UUID!, $childId: UUID!, $contextId: UUID) {\n    addRelation(parentId: $parentId, childId: $childId, contextId: $contextId)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation DeleteFractal($deleteFractalId: UUID!) {\n    deleteFractal(id: $deleteFractalId)\n  }\n"): (typeof documents)["\n  mutation DeleteFractal($deleteFractalId: UUID!) {\n    deleteFractal(id: $deleteFractalId)\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;