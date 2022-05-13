export interface IStack {
	key: string,
	techs: ITech[]
}

export interface ITech {
	key: string,
	name: string,
	level: number,
	badge?: string
}