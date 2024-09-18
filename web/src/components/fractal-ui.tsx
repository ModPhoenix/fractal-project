import {
  BookIcon,
  CheckIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  CopyIcon,
  LinkIcon,
  Loader2,
  PlusCircleIcon,
  TrashIcon,
} from "lucide-react";

import { useCallback, useState } from "react";

import {
  ADD_RELATION,
  CREATE_FRACTAL,
  DELETE_FRACTAL,
  FRACTAL,
  FractalGraphQl,
} from "@/api";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";

import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useToast } from "@/hooks";
import { useLazyQuery, useMutation, useQuery } from "@apollo/client";
import { DeepPartial } from "@apollo/client/utilities";

export const FractalUi: React.FC = () => {
  const { data, loading, error } = useQuery(FRACTAL);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error || !data) {
    return <div>Error: {error?.message ?? "No data"}</div>;
  }

  return (
    <>
      <FractalNode fractal={data.fractal} level={0} />
    </>
  );
};

type DialogType = "fractal" | "context" | "knowledge";

const FractalNode: React.FC<{
  fractal: DeepPartial<FractalGraphQl>;
  level: number;
  parentName?: string;
}> = ({ fractal, level, parentName }) => {
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
  const { toast } = useToast();

  const [isCopied, setIsCopied] = useState(false);

  const [getFractal, { data, loading }] = useLazyQuery(FRACTAL);
  const [createFractal] = useMutation(CREATE_FRACTAL, {
    refetchQueries: [{ query: FRACTAL, variables: { name: fractal.name } }],
  });
  const [deleteFractal] = useMutation(DELETE_FRACTAL, {
    refetchQueries: parentName
      ? [{ query: FRACTAL, variables: { name: parentName } }]
      : [],
  });
  const [addRelation] = useMutation(ADD_RELATION, {
    refetchQueries: [{ query: FRACTAL, variables: { name: fractal.name } }],
  });

  const handleDelete = async () => {
    if (fractal.id) {
      try {
        await deleteFractal({
          variables: { deleteFractalId: fractal.id },
        });
        toast({
          title: "Deleted",
          description: "Fractal has been successfully deleted.",
        });
      } catch (error) {
        console.error("Failed to delete fractal:", error);
        toast({
          title: "Error",
          description: "Failed to delete fractal.",
          variant: "destructive",
        });
      }
    }
  };

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
    } else if (type === "context") {
      addRelation({
        variables: {
          parentId: fractal.id,
          childId: inputValue,
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
        key={type}
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

  const copyToClipboard = useCallback(() => {
    if (fractal.id) {
      console.log("Copying: ", fractal.id);
      navigator.clipboard
        .writeText(fractal.id)
        .then(() => {
          setIsCopied(true);
          toast({
            title: "Copied!",
            description: "Fractal ID has been copied to clipboard.",
          });
          setTimeout(() => setIsCopied(false), 2000);
        })
        .catch((err) => {
          console.error("Failed to copy: ", err);
          toast({
            title: "Error",
            description: "Failed to copy Fractal ID.",
            variant: "destructive",
          });
        });
    }
  }, [fractal.id]);

  const dialogConfigs: { type: DialogType; icon: React.ReactNode }[] = [
    { type: "fractal", icon: <PlusCircleIcon className="w-4 h-4" /> },
    { type: "context", icon: <LinkIcon className="w-4 h-4" /> },
    { type: "knowledge", icon: <BookIcon className="w-4 h-4" /> },
  ];

  const DeleteConfirmDialog = () => (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          className="ml-2"
          title="Delete Fractal"
        >
          <TrashIcon className="w-4 h-4" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            This action cannot be undone. This will permanently delete the
            fractal and all its associated data.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={handleDelete}>Delete</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );

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
        <Button
          variant="ghost"
          size="icon"
          className="ml-2"
          onClick={copyToClipboard}
          title="Copy Fractal ID"
        >
          {isCopied ? (
            <CheckIcon className="w-4 h-4 text-green-500" />
          ) : (
            <CopyIcon className="w-4 h-4" />
          )}
        </Button>
        <span className="font-medium">{fractal.name}</span>
        {dialogConfigs.map(({ type, icon }) => renderDialog(type, icon))}
        <DeleteConfirmDialog />
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
                <FractalNode
                  key={child.id}
                  fractal={child}
                  level={level + 1}
                  parentName={fractal.name}
                />
              )
          )}
        </div>
      )}
    </div>
  );
};
