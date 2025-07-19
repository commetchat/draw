import "@material/web/all"

type WithAnyChildrenAndClasses = { children: any } | {} | { class?: string, classList?: Record<string, boolean> };

type SolidInterface = {
    [P in keyof HTMLElementTagNameMap]: HTMLElementTagNameMap[P] | WithAnyChildrenAndClasses;
};

declare module "solid-js" {
    namespace JSX {
        interface IntrinsicElements extends SolidInterface { }
    }
}