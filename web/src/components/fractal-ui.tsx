import { useState } from "react";
import { graphql } from "@/api/gql";
import {
  ChevronDownIcon,
  ChevronRightIcon,
  PlusCircleIcon,
  BookIcon,
  LinkIcon,
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
import { useQuery } from "@apollo/client";
import { FractalGraphQl } from "@/api/gql/graphql";

const FractalNode: React.FC<{
  fractal: Partial<FractalGraphQl>;
  level: number;
}> = ({ fractal, level }) => {
  const [isExpanded, setIsExpanded] = useState(level === 0);
  const [isAddFractalOpen, setIsAddFractalOpen] = useState(false);
  const [isAddContextOpen, setIsAddContextOpen] = useState(false);
  const [isAddKnowledgeOpen, setIsAddKnowledgeOpen] = useState(false);
  const [newFractalName, setNewFractalName] = useState("");
  const [newContextName, setNewContextName] = useState("");
  const [newKnowledge, setNewKnowledge] = useState("");

  const handleAddFractal = () => {
    console.log(
      `Adding new fractal: ${newFractalName} to parent: ${fractal.id}`
    );
    setNewFractalName("");
    setIsAddFractalOpen(false);
  };

  const handleAddContext = () => {
    console.log(
      `Adding new context: ${newContextName} to fractal: ${fractal.id}`
    );
    setNewContextName("");
    setIsAddContextOpen(false);
  };

  const handleAddKnowledge = () => {
    console.log(
      `Adding new knowledge: ${newKnowledge} to fractal: ${fractal.id}`
    );
    setNewKnowledge("");
    setIsAddKnowledgeOpen(false);
  };

  return (
    <div className="ml-4">
      <div className="flex items-center mb-2">
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="mr-2 focus:outline-none"
          aria-label={isExpanded ? "Collapse" : "Expand"}
        >
          {fractal?.children?.length > 0 ? (
            isExpanded ? (
              <ChevronDownIcon className="w-4 h-4" />
            ) : (
              <ChevronRightIcon className="w-4 h-4" />
            )
          ) : (
            <div className="w-4" />
          )}
        </button>
        <span className="font-medium">{fractal.name}</span>
        <Dialog open={isAddFractalOpen} onOpenChange={setIsAddFractalOpen}>
          <DialogTrigger asChild>
            <Button variant="ghost" size="icon" className="ml-2">
              <PlusCircleIcon className="w-4 h-4" />
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Add New Fractal</DialogTitle>
            </DialogHeader>
            <div className="flex items-center space-x-2">
              <Input
                placeholder="Fractal name"
                value={newFractalName}
                onChange={(e) => setNewFractalName(e.target.value)}
              />
              <Button onClick={handleAddFractal}>Add</Button>
            </div>
          </DialogContent>
        </Dialog>
        <Dialog open={isAddContextOpen} onOpenChange={setIsAddContextOpen}>
          <DialogTrigger asChild>
            <Button variant="ghost" size="icon" className="ml-2">
              <LinkIcon className="w-4 h-4" />
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Add New Context</DialogTitle>
            </DialogHeader>
            <div className="flex items-center space-x-2">
              <Input
                placeholder="Context name"
                value={newContextName}
                onChange={(e) => setNewContextName(e.target.value)}
              />
              <Button onClick={handleAddContext}>Add</Button>
            </div>
          </DialogContent>
        </Dialog>
        <Dialog open={isAddKnowledgeOpen} onOpenChange={setIsAddKnowledgeOpen}>
          <DialogTrigger asChild>
            <Button variant="ghost" size="icon" className="ml-2">
              <BookIcon className="w-4 h-4" />
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Add New Knowledge</DialogTitle>
            </DialogHeader>
            <div className="flex flex-col space-y-2">
              <Textarea
                placeholder="Knowledge content"
                value={newKnowledge}
                onChange={(e) => setNewKnowledge(e.target.value)}
              />
              <Button onClick={handleAddKnowledge}>Add</Button>
            </div>
          </DialogContent>
        </Dialog>
      </div>
      {isExpanded && (
        <div className="ml-4">
          {fractal?.contexts?.length > 0 && (
            <div className="mb-2">
              <span className="text-sm font-medium text-gray-500">
                Contexts:
              </span>
              <div className="flex flex-wrap gap-1 mt-1">
                {fractal?.contexts?.map((context) => (
                  <Badge key={context.id} variant="secondary">
                    {context.name}
                  </Badge>
                ))}
              </div>
            </div>
          )}
          {fractal?.children?.map((child) => (
            <FractalNode key={child.id} fractal={child} level={level + 1} />
          ))}
        </div>
      )}
    </div>
  );
};

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

export function FractalUi() {
  const { data } = useQuery(root);

  console.log("data", data);

  return (
    <div className="p-6 rounded-lg shadow-sm">
      <h1 className="text-2xl font-bold mb-4">Fractal Structure</h1>
      {data ? (
        <FractalNode fractal={data.root} level={0} />
      ) : (
        <div>Loading...</div>
      )}
    </div>
  );
}
