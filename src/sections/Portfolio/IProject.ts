export interface IProject {
	key: string,
	thumbnail: string,
	links: ILink[],
	tags: string[],
}

export interface ILink {
	key: string,
	icon: string,
	url: string,
	download?: boolean,
}

export enum Tag {
	Android = "android",
	Kotlin = "kotlin",
	MongoDB = "mongo",
	NodeJS = "nodejs",
	REST = "rest",
	Rust = "rust",
	Socketio = "socketio",
	WebAssembly = "wasm",
}