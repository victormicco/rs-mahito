import { RootProvider } from "fumadocs-ui/provider/next";
import type { Metadata } from "next";
import type { ReactNode } from "react";
import "~/styles/globals.css";

export const metadata: Metadata = {
	title: {
		default: "rs-mahito",
		template: "%s | rs-mahito",
	},
	description:
		"A Windows CLI tool for removing metadata from files on NTFS filesystems",
	keywords: [
		"metadata",
		"cleaner",
		"windows",
		"ntfs",
		"privacy",
		"cli",
		"rust",
	],
};

interface RootLayoutProps {
	children: ReactNode;
}

export default function RootLayout({ children }: RootLayoutProps) {
	return (
		<html lang="en" suppressHydrationWarning>
			<body>
				<RootProvider>{children}</RootProvider>
			</body>
		</html>
	);
}
