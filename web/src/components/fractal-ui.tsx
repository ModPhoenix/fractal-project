import { useState } from "react";
import {
  ChevronDownIcon,
  ChevronRightIcon,
  PlusCircleIcon,
  BookIcon,
  LinkIcon,
  Loader2,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Textarea } from "@/components/ui/textarea";
import { Badge } from "@/components/ui/badge";
import { useLazyQuery, useMutation, useQuery } from "@apollo/client";
import { FractalGraphQl, graphql } from "@/api";
import { DeepPartial } from "@apollo/client/utilities";
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

const FRACTAL = graphql(/* GraphQL */ `
  query Fractal($name: String) {
    fractal(name: $name) {
      ...Fractal
    }
  }
`);

const createFractalMutation = graphql(/* GraphQL */ `
  mutation CreateFractal($input: CreateFractalInput!) {
    createFractal(input: $input) {
      ...Fractal
    }
  }
`);

export const FractalUi: React.FC = () => {
  const { data, loading, error } = useQuery(FRACTAL);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error || !data) {
    return <div>Error: {error?.message ?? "No data"}</div>;
  }

  return <FractalNode fractal={data.fractal} level={0} />;
};

const FractalNode: React.FC<{
  fractal: DeepPartial<FractalGraphQl>;
  level: number;
}> = ({ fractal, level }) => {
  const [isExpanded, setIsExpanded] = useState(level === 0);
  const [dialogState, setDialogState] = useState({
    isAddFractalOpen: false,
    isAddContextOpen: false,
    isAddKnowledgeOpen: false,
  });
  const [inputState, setInputState] = useState({
    newFractal: "",
    newContext: "",
    newKnowledge: "",
  });

  const [getFractal, { data, loading }] = useLazyQuery(FRACTAL);
  const [createFractal] = useMutation(createFractalMutation, {
    refetchQueries: [{ query: FRACTAL, variables: { name: fractal.name } }],
  });

  const handleInputChange =
    (key: keyof typeof inputState) =>
    (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
      setInputState((prev) => ({ ...prev, [key]: e.target.value }));
    };

  const handleDialogChange =
    (key: keyof typeof dialogState) => (isOpen: boolean) => {
      setDialogState((prev) => ({ ...prev, [key]: isOpen }));
    };

  const handleAddItem = (type: "fractal" | "context" | "knowledge") => () => {
    const {
      newFractal: newFractalName,
      newContext: newContextName,
      newKnowledge,
    } = inputState;
    switch (type) {
      case "fractal":
        createFractal({
          variables: {
            input: {
              name: newFractalName,
              parentId: fractal.id,
              contextIds: [fractal.id],
            },
          },
        });
        console.log(
          `Adding new fractal: ${newFractalName} to parent: ${fractal.id}`
        );
        break;
      case "context":
        console.log(
          `Adding new context: ${newContextName} to fractal: ${fractal.id}`
        );
        break;
      case "knowledge":
        console.log(
          `Adding new knowledge: ${newKnowledge} to fractal: ${fractal.id}`
        );
        break;
    }

    const capitalizedType = (type.charAt(0).toUpperCase() +
      type.slice(1)) as Capitalize<typeof type>;

    setInputState((prev) => ({
      ...prev,
      [`new${capitalizedType}Name`]: "",
    }));

    handleDialogChange(`isAdd${capitalizedType}Open`)(false);
  };

  const renderDialog = (type: "fractal" | "context" | "knowledge") => {
    const capitalizedType = (type.charAt(0).toUpperCase() +
      type.slice(1)) as Capitalize<typeof type>;
    const isOpen =
      dialogState[`isAdd${capitalizedType}Open` as keyof typeof dialogState];
    const inputValue =
      inputState[`new${capitalizedType}Name` as keyof typeof inputState];
    const InputComponent = type === "knowledge" ? Textarea : Input;

    return (
      <Dialog
        open={isOpen}
        onOpenChange={handleDialogChange(`isAdd${capitalizedType}Open`)}
      >
        <DialogTrigger asChild>
          <Button variant="ghost" size="icon" className="ml-2">
            {type === "fractal" ? (
              <PlusCircleIcon className="w-4 h-4" />
            ) : type === "context" ? (
              <LinkIcon className="w-4 h-4" />
            ) : (
              <BookIcon className="w-4 h-4" />
            )}
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add New {capitalizedType}</DialogTitle>
          </DialogHeader>
          <div
            className={`flex ${type === "knowledge" ? "flex-col space-y-2" : "items-center space-x-2"}`}
          >
            <InputComponent
              placeholder={`${capitalizedType} ${type === "knowledge" ? "content" : "name"}`}
              value={inputValue}
              onChange={handleInputChange(`new${capitalizedType}`)}
            />
            <Button onClick={handleAddItem(type)}>Add</Button>
          </div>
        </DialogContent>
      </Dialog>
    );
  };

  return (
    <div className="">
      <div className="flex items-center mb-2">
        <Button
          size="icon"
          variant="outline"
          onClick={async () => {
            if (!fractal.children) {
              await getFractal({ variables: { name: fractal.name } });
            }
            setIsExpanded((prev) => !prev);
          }}
          className="mr-2"
          aria-label={isExpanded ? "Collapse" : "Expand"}
        >
          {loading ? (
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          ) : isExpanded ? (
            <ChevronDownIcon className="w-4 h-4" />
          ) : (
            <ChevronRightIcon className="w-4 h-4" />
          )}
        </Button>
        <span className="font-medium">{fractal.name}</span>
        {renderDialog("fractal")}
        {renderDialog("context")}
        {renderDialog("knowledge")}
      </div>
      {isExpanded && (
        <div className="ml-4">
          {Boolean(fractal?.contexts?.length) && (
            <div className="mb-2">
              <span className="text-sm font-medium text-gray-500">
                Contexts:
              </span>
              <div className="flex flex-wrap gap-1 mt-1">
                {fractal?.contexts?.map((context) => (
                  <Badge key={context?.id} variant="secondary">
                    {context?.name}
                  </Badge>
                ))}
              </div>
            </div>
          )}
          {(fractal?.children ?? data?.fractal?.children)?.map((child) =>
            child ? (
              <FractalNode key={child?.id} fractal={child} level={level + 1} />
            ) : null
          )}
        </div>
      )}
    </div>
  );
};
