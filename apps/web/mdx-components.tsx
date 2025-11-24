import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import defaultComponents from "fumadocs-ui/mdx";
import type { MDXComponents } from "mdx/types";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

type PreProps = ComponentPropsWithoutRef<typeof Pre> & {
	ref?: unknown;
	children?: ReactNode;
	"data-lang"?: string;
};

export function useMDXComponents(components?: MDXComponents): MDXComponents {
	return {
		...defaultComponents,
		pre: ({ ref: _ref, children, ...props }: PreProps) => (
			<CodeBlock keepBackground {...props}>
				<Pre>{children}</Pre>
			</CodeBlock>
		),
		...components,
	};
}
