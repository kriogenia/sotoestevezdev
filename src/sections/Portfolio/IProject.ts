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