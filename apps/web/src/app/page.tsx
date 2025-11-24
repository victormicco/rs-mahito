import Image from "next/image";
import Link from "next/link";

export default function HomePage() {
	return (
		<main className="flex min-h-screen flex-col">
			{/* Hero Section */}
			<section className="relative flex min-h-[80vh] flex-col items-center justify-center overflow-hidden p-8">
				{/* Background gradient */}
				<div className="absolute inset-0 bg-gradient-to-b from-purple-900/20 via-transparent to-transparent dark:from-purple-900/40" />

				{/* Hero image */}
				{/*<div className="absolute right-0 top-1/2 -translate-y-1/2 opacity-20 dark:opacity-30 md:opacity-40 md:dark:opacity-50 rounded-full">
					<Image
						src="/mahito-heros.jpg"
						alt="Mahito - Soul Manipulation"
						width={600}
						height={400}
						className="h-auto max-h-[80vh] w-auto object-fit rounded-xl"
						priority
					/>
				</div>*/}

				{/* Content */}
				<div className="relative z-10 max-w-3xl text-center">
					<div className="mb-6 inline-block rounded-full border border-purple-500/30 bg-purple-500/10 px-4 py-1.5 text-sm font-medium text-purple-600 dark:text-purple-400">
						Windows CLI Tool
					</div>
					<h1 className="text-5xl font-bold tracking-tight md:text-6xl lg:text-7xl">
						<span className="bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent dark:from-purple-400 dark:to-pink-400">
							rs-mahito
						</span>
					</h1>
					<p className="mt-6 text-xl text-fd-muted-foreground md:text-2xl">
						Cleanse your files from unwanted metadata.
						<br />
						<span className="text-lg opacity-80">
							Privacy-focused tool for NTFS filesystems.
						</span>
					</p>

					<div className="mt-10 flex flex-wrap justify-center gap-4">
						<a
							href="https://github.com/victormicco/rs-mahito/releases"
							className="inline-flex items-center gap-2 rounded-lg bg-gradient-to-r from-purple-600 to-pink-600 px-8 py-4 font-semibold text-white shadow-lg shadow-purple-500/25 transition-all hover:scale-105 hover:shadow-xl hover:shadow-purple-500/30"
						>
							<svg
								aria-hidden="true"
								className="h-5 w-5"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									strokeLinecap="round"
									strokeLinejoin="round"
									strokeWidth={2}
									d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
								/>
							</svg>
							Download
						</a>
						<Link
							href="/docs"
							className="inline-flex items-center gap-2 rounded-lg border border-fd-border bg-fd-background px-8 py-4 font-semibold transition-all hover:border-purple-500/50 hover:bg-fd-accent"
						>
							<svg
								aria-hidden="true"
								className="h-5 w-5"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									strokeLinecap="round"
									strokeLinejoin="round"
									strokeWidth={2}
									d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
								/>
							</svg>
							Documentation
						</Link>
						<a
							href="https://github.com/victormicco/rs-mahito"
							target="_blank"
							rel="noopener noreferrer"
							className="inline-flex items-center gap-2 rounded-lg border border-fd-border bg-fd-background px-8 py-4 font-semibold transition-all hover:border-purple-500/50 hover:bg-fd-accent"
						>
							<svg
								aria-hidden="true"
								className="h-5 w-5"
								fill="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									fillRule="evenodd"
									d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
									clipRule="evenodd"
								/>
							</svg>
							<svg
								aria-hidden="true"
								className="h-4 w-4 text-yellow-500"
								fill="currentColor"
								viewBox="0 0 24 24"
							>
								<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
							</svg>
							Star on GitHub
						</a>
					</div>
				</div>
			</section>

			{/* Features Section */}
			<section className="border-t border-fd-border bg-fd-background/50 px-8 py-20">
				<div className="mx-auto max-w-5xl">
					<h2 className="mb-4 text-center text-3xl font-bold">
						Clean. Secure. Fast.
					</h2>
					<p className="mb-12 text-center text-fd-muted-foreground">
						Remove privacy-sensitive metadata from your files with a single
						command.
					</p>

					<div className="grid gap-6 md:grid-cols-2">
						<div className="group relative overflow-hidden rounded-xl border border-fd-border bg-fd-card p-6 transition-all hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/5">
							<div className="mb-4 inline-flex rounded-lg bg-purple-500/10 p-3 text-purple-600 dark:text-purple-400">
								<svg
									aria-hidden="true"
									className="h-6 w-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										strokeLinecap="round"
										strokeLinejoin="round"
										strokeWidth={2}
										d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
									/>
								</svg>
							</div>
							<h3 className="mb-2 text-lg font-semibold">
								Zone Identifier Removal
							</h3>
							<p className="text-sm text-fd-muted-foreground">
								Clears the Zone.Identifier alternate data stream that marks
								files as downloaded from the internet.
							</p>
						</div>

						<div className="group relative overflow-hidden rounded-xl border border-fd-border bg-fd-card p-6 transition-all hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/5">
							<div className="mb-4 inline-flex rounded-lg bg-purple-500/10 p-3 text-purple-600 dark:text-purple-400">
								<svg
									aria-hidden="true"
									className="h-6 w-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										strokeLinecap="round"
										strokeLinejoin="round"
										strokeWidth={2}
										d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
									/>
								</svg>
							</div>
							<h3 className="mb-2 text-lg font-semibold">Timestamp Reset</h3>
							<p className="text-sm text-fd-muted-foreground">
								Resets file timestamps to a neutral date, removing temporal
								metadata that could reveal file history.
							</p>
						</div>

						<div className="group relative overflow-hidden rounded-xl border border-fd-border bg-fd-card p-6 transition-all hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/5">
							<div className="mb-4 inline-flex rounded-lg bg-purple-500/10 p-3 text-purple-600 dark:text-purple-400">
								<svg
									aria-hidden="true"
									className="h-6 w-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										strokeLinecap="round"
										strokeLinejoin="round"
										strokeWidth={2}
										d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
									/>
								</svg>
							</div>
							<h3 className="mb-2 text-lg font-semibold">Owner Clearing</h3>
							<p className="text-sm text-fd-muted-foreground">
								Removes file ownership information via SID manipulation,
								eliminating user identification traces.
							</p>
						</div>

						<div className="group relative overflow-hidden rounded-xl border border-fd-border bg-fd-card p-6 transition-all hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/5">
							<div className="mb-4 inline-flex rounded-lg bg-purple-500/10 p-3 text-purple-600 dark:text-purple-400">
								<svg
									aria-hidden="true"
									className="h-6 w-6"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										strokeLinecap="round"
										strokeLinejoin="round"
										strokeWidth={2}
										d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
									/>
								</svg>
							</div>
							<h3 className="mb-2 text-lg font-semibold">
								Office Document Cleaning
							</h3>
							<p className="text-sm text-fd-muted-foreground">
								Strips metadata from Office documents (docx, xlsx, pptx) by
								processing their internal XML structure.
							</p>
						</div>
					</div>
				</div>
			</section>

			{/* Quick Start Section */}
			<section className="border-t border-fd-border px-8 py-20">
				<div className="mx-auto max-w-3xl text-center">
					<h2 className="mb-4 text-3xl font-bold">Quick Start</h2>
					<p className="mb-8 text-fd-muted-foreground">
						Get started in seconds with a simple command.
					</p>
					<div className="overflow-hidden rounded-lg border border-fd-border bg-fd-card">
						<div className="flex items-center justify-between border-b border-fd-border bg-fd-muted/30 px-4 py-2">
							<span className="text-sm text-fd-muted-foreground">Terminal</span>
							<div className="flex gap-1.5">
								<div className="h-3 w-3 rounded-full bg-red-500/50" />
								<div className="h-3 w-3 rounded-full bg-yellow-500/50" />
								<div className="h-3 w-3 rounded-full bg-green-500/50" />
							</div>
						</div>
						<pre className="overflow-x-auto p-6 text-left">
							<code className="text-sm">
								<span className="text-fd-muted-foreground">
									# Clean a single file
								</span>
								{"\n"}
								<span className="text-purple-400">rs-mahito</span> file
								document.pdf
								{"\n\n"}
								<span className="text-fd-muted-foreground">
									# Clean all files in a directory
								</span>
								{"\n"}
								<span className="text-purple-400">rs-mahito</span> dir
								./downloads
								{"\n\n"}
								<span className="text-fd-muted-foreground">
									# Recursive cleaning
								</span>
								{"\n"}
								<span className="text-purple-400">rs-mahito</span> recursive
								./project --yes
							</code>
						</pre>
					</div>
				</div>
			</section>

			{/* Footer */}
			<footer className="border-t border-fd-border px-8 py-8">
				<div className="mx-auto flex max-w-5xl flex-col items-center justify-between gap-4 md:flex-row">
					<div className="flex items-center gap-3">
						<Image
							src="/mahito-laugh.jpg"
							alt="rs-mahito"
							width={32}
							height={32}
							className="rounded-full"
						/>
						<span className="font-semibold">rs-mahito</span>
					</div>
					<p className="text-sm text-fd-muted-foreground">
						Built with Rust. Made for privacy.
					</p>
				</div>
			</footer>
		</main>
	);
}
