import { createMDX } from "fumadocs-mdx/next";
import type { NextConfig } from "next";

const config: NextConfig = {
	reactStrictMode: true,
	redirects: async () => [
		{
			source: "/docs",
			destination: "/docs/introduction",
			permanent: false,
		},
		{
			source: "/github",
			destination: "https://github.com/your-username/rs-mahito",
			permanent: true,
		},
	],
	typescript: {
		ignoreBuildErrors: false,
	},
};

const withMDX = createMDX();

export default withMDX(config);
