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

const CREATE_FRACTAL = graphql(/* GraphQL */ `
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

type DialogType = "fractal" | "context" | "knowledge";

const FractalNode: React.FC<{
  fractal: DeepPartial<FractalGraphQl>;
  level: number;
}> = ({ fractal, level }) => {
  const [isExpanded, setIsExpanded] = useState(level === 0);
  const [dialogState, setDialogState] = useState<Record<DialogType, boolean>>({
    fractal: false,
    context: false,
    knowledge: false,
  });
  const [inputState, setInputState] = useState<Record<DialogType, string>>({
    fractal: "",
    context: "",
    knowledge: "",
  });

  const [getFractal, { data, loading }] = useLazyQuery(FRACTAL);
  const [createFractal] = useMutation(CREATE_FRACTAL, {
    refetchQueries: [{ query: FRACTAL, variables: { name: fractal.name } }],
  });

  const handleInputChange =
    (type: DialogType) =>
    (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
      setInputState((prev) => ({ ...prev, [type]: e.target.value }));
    };

  const toggleDialog = (type: DialogType, isOpen: boolean) => {
    setDialogState((prev) => ({ ...prev, [type]: isOpen }));
  };

  const handleAddItem = (type: DialogType) => () => {
    const inputValue = inputState[type];
    if (!inputValue) return;

    if (type === "fractal") {
      createFractal({
        variables: {
          input: {
            name: inputValue,
            parentId: fractal.id,
            contextIds: [fractal.id],
          },
        },
      });
    } else {
      console.log(
        `Adding new ${type}: ${inputValue} to fractal: ${fractal.id}`
      );
    }

    setInputState((prev) => ({ ...prev, [type]: "" }));
    toggleDialog(type, false);
  };

  const renderDialog = (type: DialogType, IconComponent: React.ReactNode) => {
    const isOpen = dialogState[type];
    const inputValue = inputState[type];
    const InputComponent = type === "knowledge" ? Textarea : Input;
    const placeholder = `${type.charAt(0).toUpperCase() + type.slice(1)} ${
      type === "knowledge" ? "content" : "name"
    }`;

    return (
      <Dialog
        open={isOpen}
        onOpenChange={(isOpen) => toggleDialog(type, isOpen)}
      >
        <DialogTrigger asChild>
          <Button variant="ghost" size="icon" className="ml-2">
            {IconComponent}
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              Add New {type.charAt(0).toUpperCase() + type.slice(1)}
            </DialogTitle>
          </DialogHeader>
          <div className="flex flex-col space-y-2">
            <InputComponent
              placeholder={placeholder}
              value={inputValue}
              onChange={handleInputChange(type)}
            />
            <Button onClick={handleAddItem(type)}>Add</Button>
          </div>
        </DialogContent>
      </Dialog>
    );
  };

  const dialogConfigs: { type: DialogType; icon: React.ReactNode }[] = [
    { type: "fractal", icon: <PlusCircleIcon className="w-4 h-4" /> },
    { type: "context", icon: <LinkIcon className="w-4 h-4" /> },
    { type: "knowledge", icon: <BookIcon className="w-4 h-4" /> },
  ];

  return (
    <div>
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
            <Loader2 className="h-4 w-4 animate-spin" />
          ) : isExpanded ? (
            <ChevronDownIcon className="w-4 h-4" />
          ) : (
            <ChevronRightIcon className="w-4 h-4" />
          )}
        </Button>
        <span className="font-medium">{fractal.name}</span>
        {dialogConfigs.map(({ type, icon }) => renderDialog(type, icon))}
      </div>
      {isExpanded && (
        <div className="ml-4">
          {(fractal?.contexts?.length ?? 0) > 0 && (
            <div className="mb-2">
              <span className="text-sm font-medium text-gray-500">
                Contexts:
              </span>
              <div className="flex flex-wrap gap-1 mt-1">
                {fractal?.contexts?.map(
                  (context) =>
                    context && (
                      <Badge key={context.id} variant="secondary">
                        {context.name}
                      </Badge>
                    )
                )}
              </div>
            </div>
          )}
          {(
            fractal.children ??
            (data?.fractal as DeepPartial<FractalGraphQl>)?.children
          )?.map(
            (child) =>
              child && (
                <FractalNode key={child.id} fractal={child} level={level + 1} />
              )
          )}
        </div>
      )}
    </div>
  );
};
