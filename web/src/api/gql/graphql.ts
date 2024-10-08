/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: any; output: any; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any; }
};

export type AddKnowledgeInput = {
  content: Scalars['String']['input'];
  context: Array<Scalars['UUID']['input']>;
  fractalId: Scalars['UUID']['input'];
};

export type CreateFractalInput = {
  contextIds?: InputMaybe<Scalars['UUID']['input']>;
  name: Scalars['String']['input'];
  parentId: Scalars['UUID']['input'];
};

export type FractalGraphQl = {
  __typename?: 'FractalGraphQL';
  children: Array<FractalGraphQl>;
  contexts: Array<FractalGraphQl>;
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['UUID']['output'];
  name: Scalars['String']['output'];
  parents: Array<FractalGraphQl>;
  updatedAt: Scalars['DateTime']['output'];
};


export type FractalGraphQlChildrenArgs = {
  input: GetFractalChildrenInput;
};

export type GetFractalChildrenInput = {
  contextId?: InputMaybe<Scalars['UUID']['input']>;
};

export type KnowledgeGraphQl = {
  __typename?: 'KnowledgeGraphQL';
  content: Scalars['String']['output'];
  fractal: FractalGraphQl;
  id: Scalars['UUID']['output'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  addKnowledge: KnowledgeGraphQl;
  addRelation: Scalars['Boolean']['output'];
  createFractal: FractalGraphQl;
  deleteFractal: Scalars['Boolean']['output'];
};


export type MutationRootAddKnowledgeArgs = {
  input: AddKnowledgeInput;
};


export type MutationRootAddRelationArgs = {
  childId: Scalars['UUID']['input'];
  contextId?: InputMaybe<Scalars['UUID']['input']>;
  parentId: Scalars['UUID']['input'];
};


export type MutationRootCreateFractalArgs = {
  input: CreateFractalInput;
};


export type MutationRootDeleteFractalArgs = {
  id: Scalars['UUID']['input'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  fractal: FractalGraphQl;
  knowledge: KnowledgeGraphQl;
};


export type QueryRootFractalArgs = {
  name?: InputMaybe<Scalars['String']['input']>;
};


export type QueryRootKnowledgeArgs = {
  context: Array<Scalars['UUID']['input']>;
  fractalName: Scalars['String']['input'];
};

export type FractalFragment = { __typename?: 'FractalGraphQL', id: any, name: string, createdAt: any, updatedAt: any } & { ' $fragmentName'?: 'FractalFragment' };

export type FractalQueryVariables = Exact<{
  name?: InputMaybe<Scalars['String']['input']>;
  childrenInput: GetFractalChildrenInput;
}>;


export type FractalQuery = { __typename?: 'QueryRoot', fractal: (
    { __typename?: 'FractalGraphQL', children: Array<(
      { __typename?: 'FractalGraphQL' }
      & { ' $fragmentRefs'?: { 'FractalFragment': FractalFragment } }
    )>, parents: Array<(
      { __typename?: 'FractalGraphQL' }
      & { ' $fragmentRefs'?: { 'FractalFragment': FractalFragment } }
    )>, contexts: Array<(
      { __typename?: 'FractalGraphQL' }
      & { ' $fragmentRefs'?: { 'FractalFragment': FractalFragment } }
    )> }
    & { ' $fragmentRefs'?: { 'FractalFragment': FractalFragment } }
  ) };

export type CreateFractalMutationVariables = Exact<{
  input: CreateFractalInput;
}>;


export type CreateFractalMutation = { __typename?: 'MutationRoot', createFractal: (
    { __typename?: 'FractalGraphQL' }
    & { ' $fragmentRefs'?: { 'FractalFragment': FractalFragment } }
  ) };

export type AddRelationMutationVariables = Exact<{
  parentId: Scalars['UUID']['input'];
  childId: Scalars['UUID']['input'];
  contextId?: InputMaybe<Scalars['UUID']['input']>;
}>;


export type AddRelationMutation = { __typename?: 'MutationRoot', addRelation: boolean };

export type DeleteFractalMutationVariables = Exact<{
  deleteFractalId: Scalars['UUID']['input'];
}>;


export type DeleteFractalMutation = { __typename?: 'MutationRoot', deleteFractal: boolean };

export const FractalFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"Fractal"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"FractalGraphQL"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}}]}}]} as unknown as DocumentNode<FractalFragment, unknown>;
export const FractalDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"Fractal"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"name"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"childrenInput"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"GetFractalChildrenInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"fractal"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"name"},"value":{"kind":"Variable","name":{"kind":"Name","value":"name"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"Fractal"}},{"kind":"Field","name":{"kind":"Name","value":"children"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"childrenInput"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"Fractal"}}]}},{"kind":"Field","name":{"kind":"Name","value":"parents"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"Fractal"}}]}},{"kind":"Field","name":{"kind":"Name","value":"contexts"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"Fractal"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"Fractal"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"FractalGraphQL"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}}]}}]} as unknown as DocumentNode<FractalQuery, FractalQueryVariables>;
export const CreateFractalDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateFractal"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"CreateFractalInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createFractal"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"Fractal"}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"Fractal"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"FractalGraphQL"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}}]}}]} as unknown as DocumentNode<CreateFractalMutation, CreateFractalMutationVariables>;
export const AddRelationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"AddRelation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"childId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"contextId"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"addRelation"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"parentId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"parentId"}}},{"kind":"Argument","name":{"kind":"Name","value":"childId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"childId"}}},{"kind":"Argument","name":{"kind":"Name","value":"contextId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"contextId"}}}]}]}}]} as unknown as DocumentNode<AddRelationMutation, AddRelationMutationVariables>;
export const DeleteFractalDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DeleteFractal"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"deleteFractalId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteFractal"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"deleteFractalId"}}}]}]}}]} as unknown as DocumentNode<DeleteFractalMutation, DeleteFractalMutationVariables>;