import { DocsLayout } from "fumadocs-ui/layouts/docs";
import Image from "next/image";
import type { ReactNode } from "react";
import { source } from "~/lib/source";

interface DocsLayoutProps {
	children: ReactNode;
}

export default function Layout({ children }: DocsLayoutProps) {
	return (
		<DocsLayout
			tree={source.pageTree}
			nav={{
				title: (
					<div className="flex items-center gap-2">
						<Image
							src="/mahito-laugh.jpg"
							alt="rs-mahito"
							width={28}
							height={28}
							className="rounded-full"
						/>
						<span className="font-semibold">rs-mahito</span>
					</div>
				),
			}}
			sidebar={{
				defaultOpenLevel: 1,
			}}
		>
			{children}
		</DocsLayout>
	);
}
