import {
  Code2,
  LifeBuoy,
  Settings2,
  SquareUser,
  ListTree,
  Network,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { FC, ReactNode } from "react";
import { Link } from "@tanstack/react-router";

interface NavItemProps {
  to?: string;
  label: string;
  icon: ReactNode;
}

const NavItem: FC<NavItemProps> = ({ to, label, icon }) => {
  const ButtonContent = (
    <Button
      variant="ghost"
      size="icon"
      className="rounded-lg"
      aria-label={label}
    >
      {icon}
    </Button>
  );

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        {to ? (
          <Link
            to={to}
            className="rounded-lg"
            activeProps={{
              className: "bg-muted",
            }}
          >
            {ButtonContent}
          </Link>
        ) : (
          ButtonContent
        )}
      </TooltipTrigger>
      <TooltipContent side="right" sideOffset={5}>
        {label}
      </TooltipContent>
    </Tooltip>
  );
};

export const Navigation: FC = () => {
  return (
    <>
      <nav className="grid gap-1 p-2">
        <NavItem
          to="/"
          label="Tree View"
          icon={<Network className="size-5" />}
        />
        <NavItem
          to="/traversal"
          label="Fractal Traversal"
          icon={<ListTree className="size-5" />}
        />
        <NavItem label="API" icon={<Code2 className="size-5" />} />
        <NavItem label="Settings" icon={<Settings2 className="size-5" />} />
      </nav>
      <nav className="mt-auto grid gap-1 p-2">
        <NavItem label="Help" icon={<LifeBuoy className="size-5" />} />
        <NavItem label="Account" icon={<SquareUser className="size-5" />} />
      </nav>
    </>
  );
};
