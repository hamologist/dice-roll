interface DiscordContext {
    discordApplicationId: string,
    messageToken: string,
}

export const isDiscordContext = (obj: any): obj is DiscordContext => {
    if (obj === undefined || obj === null) {
        return false;
    }

    if (obj.discordApplicationId === undefined || typeof obj.discordApplicationId !== 'string') {
        return false;
    }

    if (obj.messageToken === undefined || typeof obj.messageToken !== 'string') {
        return false;
    }

    return true;
};
