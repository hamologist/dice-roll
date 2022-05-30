export interface RollResponse {
    step: Array<Step>
}

export interface Step {
    rolls: Array<Roll>,
    total: number
}

export interface Roll {
    count: number,
    sides: number,
    modifier: number,
    rolls: Array<number>,
    total: number
}
