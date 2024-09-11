import { ApolloClient, HttpLink, InMemoryCache, from } from "@apollo/client";
import { onError } from "@apollo/client/link/error";
import { toast } from "@/hooks/use-toast";

const errorLink = onError(({ graphQLErrors, networkError }) => {
  if (graphQLErrors)
    graphQLErrors.forEach(({ message }) =>
      toast({ title: "Uh oh! Something went wrong.", description: message })
    );
  if (networkError) {
    toast({ title: "Network error", description: networkError.message });
  }
});

const httpLink = new HttpLink({ uri: "/graphql" });

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: from([errorLink, httpLink]),
});
