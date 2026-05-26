"use client";

import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { PanelLeftIcon } from "lucide-react";
import * as React from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle } from "@/components/ui/sheet";
import { Skeleton } from "@/components/ui/skeleton";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { useIsMobile } from "@/hooks/use-mobile";
import { cn } from "@/lib/utils";

const SIDEBAR_WIDTH = "16rem";
const SIDEBAR_WIDTH_MOBILE = "18rem";
const SIDEBAR_WIDTH_ICON = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT = "b";

interface SidebarContextProps {
	isMobile: boolean;
	open: boolean;
	openMobile: boolean;
	setOpen: (open: boolean) => void;
	setOpenMobile: (open: boolean) => void;
	state: "collapsed" | "expanded";
	toggleSidebar: () => void;
}

const SidebarContext = React.createContext<null | SidebarContextProps>(null);

function Sidebar({
	children,
	className,
	collapsible = "offcanvas",
	side = "left",
	variant = "sidebar",
	...props
}: {
	collapsible?: "icon" | "none" | "offcanvas";
	side?: "left" | "right";
	variant?: "floating" | "inset" | "sidebar";
} & React.ComponentProps<"div">) {
	const { isMobile, openMobile, setOpenMobile, state } = useSidebar();

	if (collapsible === "none") {
		return (
			<div
				className={cn("bg-sidebar text-sidebar-foreground flex h-full w-(--sidebar-width) flex-col", className)}
				data-slot="sidebar"
				{...props}
			>
				{children}
			</div>
		);
	}

	if (isMobile) {
		return (
			<Sheet onOpenChange={setOpenMobile} open={openMobile} {...props}>
				<SheetContent
					className="bg-sidebar text-sidebar-foreground w-(--sidebar-width) p-0 [&>button]:hidden"
					data-mobile="true"
					data-sidebar="sidebar"
					data-slot="sidebar"
					side={side}
					style={
						{
							"--sidebar-width": SIDEBAR_WIDTH_MOBILE,
						} as React.CSSProperties
					}
				>
					<SheetHeader className="sr-only">
						<SheetTitle>Sidebar</SheetTitle>
						<SheetDescription>Displays the mobile sidebar.</SheetDescription>
					</SheetHeader>
					<div className="flex h-full w-full flex-col">{children}</div>
				</SheetContent>
			</Sheet>
		);
	}

	return (
		<div
			className="group peer text-sidebar-foreground hidden md:block"
			data-collapsible={state === "collapsed" ? collapsible : ""}
			data-side={side}
			data-slot="sidebar"
			data-state={state}
			data-variant={variant}
		>
			{}
			<div
				className={cn(
					"relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear",
					"group-data-[collapsible=offcanvas]:w-0",
					"group-data-[side=right]:rotate-180",
					variant === "floating" || variant === "inset"
						? "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
						: "group-data-[collapsible=icon]:w-(--sidebar-width-icon)",
				)}
				data-slot="sidebar-gap"
			/>
			<div
				className={cn(
					"fixed inset-y-0 z-10 hidden h-svh w-(--sidebar-width) transition-[left,right,width] duration-200 ease-linear md:flex",
					side === "left"
						? "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]"
						: "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]",
					variant === "floating" || variant === "inset"
						? "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
						: "group-data-[collapsible=icon]:w-(--sidebar-width-icon) group-data-[side=left]:border-r group-data-[side=right]:border-l",
					className,
				)}
				data-slot="sidebar-container"
				{...props}
			>
				<div
					className="bg-sidebar group-data-[variant=floating]:border-sidebar-border flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-sm"
					data-sidebar="sidebar"
					data-slot="sidebar-inner"
				>
					{children}
				</div>
			</div>
		</div>
	);
}

function SidebarContent({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn(
				"flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden",
				className,
			)}
			data-sidebar="content"
			data-slot="sidebar-content"
			{...props}
		/>
	);
}

function SidebarFooter({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn("flex flex-col gap-2 p-2", className)}
			data-sidebar="footer"
			data-slot="sidebar-footer"
			{...props}
		/>
	);
}

function SidebarGroup({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn("relative flex w-full min-w-0 flex-col p-2", className)}
			data-sidebar="group"
			data-slot="sidebar-group"
			{...props}
		/>
	);
}

function SidebarGroupAction({
	asChild = false,
	className,
	...props
}: { asChild?: boolean } & React.ComponentProps<"button">) {
	const Comp = asChild ? Slot : "button";

	return (
		<Comp
			className={cn(
				"text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground absolute top-3.5 right-3 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
				"after:absolute after:-inset-2 md:after:hidden",
				"group-data-[collapsible=icon]:hidden",
				className,
			)}
			data-sidebar="group-action"
			data-slot="sidebar-group-action"
			{...(props as any)}
		/>
	);
}

function SidebarGroupContent({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn("w-full text-sm", className)}
			data-sidebar="group-content"
			data-slot="sidebar-group-content"
			{...props}
		/>
	);
}

function SidebarGroupLabel({
	asChild = false,
	className,
	...props
}: { asChild?: boolean } & React.ComponentProps<"div">) {
	const Comp = asChild ? Slot : "div";

	return (
		<Comp
			className={cn(
				"text-sidebar-foreground/70 ring-sidebar-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
				"group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
				className,
			)}
			data-sidebar="group-label"
			data-slot="sidebar-group-label"
			{...(props as any)}
		/>
	);
}

function SidebarHeader({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn("flex flex-col gap-2 p-2", className)}
			data-sidebar="header"
			data-slot="sidebar-header"
			{...props}
		/>
	);
}

function SidebarInput({ className, ...props }: React.ComponentProps<typeof Input>) {
	return (
		<Input
			className={cn("bg-background h-8 w-full shadow-none", className)}
			data-sidebar="input"
			data-slot="sidebar-input"
			{...props}
		/>
	);
}

function SidebarInset({ className, ...props }: React.ComponentProps<"main">) {
	return (
		<main
			className={cn(
				"bg-background relative flex w-full flex-1 flex-col",
				"md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow-sm md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2",
				className,
			)}
			data-slot="sidebar-inset"
			{...props}
		/>
	);
}

function SidebarMenu({ className, ...props }: React.ComponentProps<"ul">) {
	return (
		<ul
			className={cn("flex w-full min-w-0 flex-col gap-1", className)}
			data-sidebar="menu"
			data-slot="sidebar-menu"
			{...props}
		/>
	);
}

function SidebarMenuItem({ className, ...props }: React.ComponentProps<"li">) {
	return (
		<li
			className={cn("group/menu-item relative", className)}
			data-sidebar="menu-item"
			data-slot="sidebar-menu-item"
			{...props}
		/>
	);
}

function SidebarProvider({
	children,
	className,
	defaultOpen = true,
	onOpenChange: setOpenProp,
	open: openProp,
	style,
	...props
}: {
	defaultOpen?: boolean;
	onOpenChange?: (open: boolean) => void;
	open?: boolean;
} & React.ComponentProps<"div">) {
	const isMobile = useIsMobile();
	const [openMobile, setOpenMobile] = React.useState(false);

	const [_open, _setOpen] = React.useState(defaultOpen);
	const open = openProp ?? _open;
	const setOpen = React.useCallback(
		(value: ((value: boolean) => boolean) | boolean) => {
			const openState = typeof value === "function" ? value(open) : value;
			if (setOpenProp) {
				setOpenProp(openState);
			} else {
				_setOpen(openState);
			}
		},
		[setOpenProp, open],
	);

	const toggleSidebar = React.useCallback(() => {
		if (isMobile) {
			setOpenMobile((open) => !open);
		} else {
			setOpen((open) => !open);
		}
	}, [isMobile, setOpen]);

	React.useEffect(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === SIDEBAR_KEYBOARD_SHORTCUT && (event.metaKey || event.ctrlKey)) {
				event.preventDefault();
				toggleSidebar();
			}
		};

		globalThis.addEventListener("keydown", handleKeyDown);
		return () => {
			globalThis.removeEventListener("keydown", handleKeyDown);
		};
	}, [toggleSidebar]);

	const state = open ? "expanded" : "collapsed";

	const contextValue = React.useMemo<SidebarContextProps>(
		() => ({
			isMobile,
			open,
			openMobile,
			setOpen,
			setOpenMobile,
			state,
			toggleSidebar,
		}),
		[state, open, setOpen, isMobile, openMobile, toggleSidebar],
	);

	return (
		<SidebarContext.Provider value={contextValue}>
			<TooltipProvider delayDuration={0}>
				<div
					className={cn(
						"group/sidebar-wrapper has-data-[variant=inset]:bg-sidebar flex min-h-svh w-full",
						className,
					)}
					data-slot="sidebar-wrapper"
					style={
						{
							"--sidebar-width": SIDEBAR_WIDTH,
							"--sidebar-width-icon": SIDEBAR_WIDTH_ICON,
							...style,
						} as React.CSSProperties
					}
					{...props}
				>
					{children}
				</div>
			</TooltipProvider>
		</SidebarContext.Provider>
	);
}

function SidebarSeparator({ className, ...props }: React.ComponentProps<typeof Separator>) {
	return (
		<Separator
			className={cn("mx-2 w-auto", className)}
			data-sidebar="separator"
			data-slot="sidebar-separator"
			{...props}
		/>
	);
}

function SidebarTrigger({ className, onClick, ...props }: React.ComponentProps<typeof Button>) {
	const { toggleSidebar } = useSidebar();

	return (
		<Button
			className={cn("size-7", className)}
			data-sidebar="trigger"
			data-slot="sidebar-trigger"
			onClick={(event) => {
				onClick?.(event);
				toggleSidebar();
			}}
			size="icon"
			variant="ghost"
			{...props}
		>
			<PanelLeftIcon />
			<span className="sr-only">Toggle Sidebar</span>
		</Button>
	);
}

function useSidebar() {
	const context = React.useContext(SidebarContext);
	if (!context) {
		throw new Error("useSidebar must be used within a SidebarProvider.");
	}

	return context;
}

const sidebarMenuButtonVariants = cva(
	"peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-hidden ring-sidebar-ring transition-[width,height,padding] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidebar=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! group-data-[collapsible=icon]:hover:bg-transparent cursor-pointer [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
	{
		defaultVariants: {
			size: "default",
			variant: "default",
		},
		variants: {
			size: {
				default: "h-8 text-sm",
				lg: "h-12 text-sm group-data-[collapsible=icon]:p-0!",
				sm: "h-7 text-xs",
			},
			variant: {
				default: "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
				outline:
					"bg-background shadow-[0_0_0_1px_hsl(var(--sidebar-border))] hover:bg-sidebar-accent hover:text-sidebar-accent-foreground hover:shadow-[0_0_0_1px_hsl(var(--sidebar-accent))]",
			},
		},
	},
);

function SidebarMenuAction({
	asChild = false,
	className,
	showOnHover = false,
	...props
}: {
	asChild?: boolean;
	showOnHover?: boolean;
} & React.ComponentProps<"button">) {
	const Comp = asChild ? Slot : "button";

	return (
		<Comp
			className={cn(
				"text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground peer-hover/menu-button:text-sidebar-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
				"after:absolute after:-inset-2 md:after:hidden",
				"peer-data-[size=sm]/menu-button:top-1",
				"peer-data-[size=default]/menu-button:top-1.5",
				"peer-data-[size=lg]/menu-button:top-2.5",
				"group-data-[collapsible=icon]:hidden",
				showOnHover &&
					"peer-data-[active=true]/menu-button:text-sidebar-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0",
				className,
			)}
			data-sidebar="menu-action"
			data-slot="sidebar-menu-action"
			{...(props as any)}
		/>
	);
}

function SidebarMenuBadge({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			className={cn(
				"text-sidebar-foreground pointer-events-none absolute right-1 flex h-5 min-w-5 items-center justify-center rounded-md px-1 text-xs font-medium tabular-nums select-none",
				"peer-hover/menu-button:text-sidebar-accent-foreground peer-data-[active=true]/menu-button:text-sidebar-accent-foreground",
				"peer-data-[size=sm]/menu-button:top-1",
				"peer-data-[size=default]/menu-button:top-1.5",
				"peer-data-[size=lg]/menu-button:top-2.5",
				"group-data-[collapsible=icon]:hidden",
				className,
			)}
			data-sidebar="menu-badge"
			data-slot="sidebar-menu-badge"
			{...props}
		/>
	);
}

function SidebarMenuButton({
	asChild = false,
	className,
	isActive = false,
	size = "default",
	tooltip,
	variant = "default",
	...props
}: {
	asChild?: boolean;
	isActive?: boolean;
	tooltip?: React.ComponentProps<typeof TooltipContent> | string;
} & React.ComponentProps<"button"> &
	VariantProps<typeof sidebarMenuButtonVariants>) {
	const Comp = asChild ? Slot : "button";
	const { isMobile, state } = useSidebar();

	const button = (
		<Comp
			className={cn(sidebarMenuButtonVariants({ size, variant }), className)}
			data-active={isActive}
			data-sidebar="menu-button"
			data-size={size}
			data-slot="sidebar-menu-button"
			{...(props as any)}
		/>
	);

	if (!tooltip) {
		return button;
	}

	if (typeof tooltip === "string") {
		tooltip = {
			children: tooltip,
		};
	}

	return (
		<Tooltip>
			<TooltipTrigger asChild>{button}</TooltipTrigger>
			<TooltipContent
				align="center"
				hidden={state !== "collapsed" || isMobile}
				side="right"
				sideOffset={11}
				{...tooltip}
			/>
		</Tooltip>
	);
}

function SidebarMenuSkeleton({
	className,
	showIcon = false,
	...props
}: {
	showIcon?: boolean;
} & React.ComponentProps<"div">) {
	const [width] = React.useState(() => `${Math.floor(Math.random() * 40) + 50}%`);

	return (
		<div
			className={cn("flex h-8 items-center gap-2 rounded-md px-2", className)}
			data-sidebar="menu-skeleton"
			data-slot="sidebar-menu-skeleton"
			{...props}
		>
			{showIcon && <Skeleton className="size-4 rounded-md" data-sidebar="menu-skeleton-icon" />}
			<Skeleton
				className="h-4 max-w-(--skeleton-width) flex-1"
				data-sidebar="menu-skeleton-text"
				style={
					{
						"--skeleton-width": width,
					} as React.CSSProperties
				}
			/>
		</div>
	);
}

function SidebarMenuSub({ className, ...props }: React.ComponentProps<"ul">) {
	return (
		<ul
			className={cn(
				"border-sidebar-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5",
				"group-data-[collapsible=icon]:hidden",
				className,
			)}
			data-sidebar="menu-sub"
			data-slot="sidebar-menu-sub"
			{...props}
		/>
	);
}

function SidebarMenuSubButton({
	asChild = false,
	className,
	isActive = false,
	size = "md",
	...props
}: {
	asChild?: boolean;
	isActive?: boolean;
	size?: "md" | "sm";
} & React.ComponentProps<"a">) {
	const Comp = asChild ? Slot : "a";

	return (
		<Comp
			className={cn(
				"text-sidebar-foreground ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground active:bg-sidebar-accent active:text-sidebar-accent-foreground [&>svg]:text-sidebar-accent-foreground flex h-7 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-md px-2 outline-hidden focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
				"data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground",
				size === "sm" && "text-xs",
				size === "md" && "text-sm",
				"group-data-[collapsible=icon]:hidden group-data-[collapsible=icon]:hover:bg-transparent cursor-pointer",
				className,
			)}
			data-active={isActive}
			data-sidebar="menu-sub-button"
			data-size={size}
			data-slot="sidebar-menu-sub-button"
			{...(props as any)}
		/>
	);
}

function SidebarMenuSubItem({ className, ...props }: React.ComponentProps<"li">) {
	return (
		<li
			className={cn("group/menu-sub-item relative", className)}
			data-sidebar="menu-sub-item"
			data-slot="sidebar-menu-sub-item"
			{...props}
		/>
	);
}

export {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarGroupAction,
	SidebarGroupContent,
	SidebarGroupLabel,
	SidebarHeader,
	SidebarInput,
	SidebarInset,
	SidebarMenu,
	SidebarMenuAction,
	SidebarMenuBadge,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarMenuSkeleton,
	SidebarMenuSub,
	SidebarMenuSubButton,
	SidebarMenuSubItem,
	SidebarProvider,
	SidebarSeparator,
	SidebarTrigger,
	useSidebar,
};
